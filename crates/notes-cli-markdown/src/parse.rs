use crate::render::HEADER;
use core::error::Error;
use core::fmt::{Display, Formatter, Result as FmtResult};
use core::iter::{Enumerate, Peekable};
use core::str::Lines as StrLines;
use notes_cli_describe::{ArgumentSpec, CliSpec, CommandSpec, Outcome, Shape, Source};

/// A line of Markdown that does not state part of a spec.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseError {
    /// The line, counting from one.
    pub line: usize,
    /// What the line should have stated.
    pub expected: String,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "line {}: expected {}", self.line, self.expected)
    }
}

impl Error for ParseError {}

/// Reads the Markdown [`render`](crate::render) writes as the spec it states.
///
/// # Errors
///
/// Answers with the first line that does not state its part of the spec.
pub fn parse(markdown: &str) -> Result<CliSpec, ParseError> {
    let mut lines = Lines { lines: markdown.lines().enumerate().peekable() };

    let name = lines.expect_prefix("# ", "a `# name` title")?.to_owned();
    let mut commands = Vec::new();
    while lines.skip_blank() {
        commands.push(lines.command(&name)?);
    }
    Ok(CliSpec { name, commands })
}

struct Lines<'a> {
    lines: Peekable<Enumerate<StrLines<'a>>>,
}

impl<'a> Lines<'a> {
    /// Skips blank lines, answering whether any line is left.
    fn skip_blank(&mut self) -> bool {
        while self.lines.next_if(|(_, line)| line.trim().is_empty()).is_some() {}
        self.lines.peek().is_some()
    }

    fn error(&mut self, expected: &str) -> ParseError {
        let line = self.lines.peek().map_or(0, |(index, _)| index + 1);
        ParseError { line, expected: expected.to_owned() }
    }

    fn expect_prefix(&mut self, prefix: &str, expected: &str) -> Result<&'a str, ParseError> {
        self.skip_blank();
        match self.lines.peek() {
            Some((_, line)) if line.starts_with(prefix) => {
                let rest = &line[prefix.len()..];
                self.lines.next();
                Ok(rest.trim())
            }
            _ => Err(self.error(expected)),
        }
    }

    fn command(&mut self, program: &str) -> Result<CommandSpec, ParseError> {
        let heading = self.expect_prefix("## ", "a ``## `name command` `` heading")?;
        let words = code(heading).ok_or_else(|| self.error("a heading in backticks"))?;
        let mut words = words.split(' ').map(str::to_owned);
        if words.next().as_deref() != Some(program) {
            return Err(self.error(&format!("a command of `{program}`")));
        }
        let path = words.collect();

        // The help is every paragraph before the operation.
        let mut doc = Vec::new();
        self.skip_blank();
        while let Some((_, line)) = self.lines.next_if(|(_, line)| !line.starts_with("- Operation:")) {
            doc.push(line);
        }
        let doc = doc.join("\n").trim().to_owned();

        let operation = self.expect_prefix("- Operation:", "`- Operation: `name``")?;
        let operation = code(operation).ok_or_else(|| self.error("the operation in backticks"))?.to_owned();
        let outcome = self.expect_prefix("- Outcome:", "`- Outcome: …`")?;
        let outcome = parse_outcome(outcome).ok_or_else(|| self.error("total, fallible, or fallible by error"))?;

        let mut arguments = Vec::new();
        self.skip_blank();
        if self.lines.next_if(|(_, line)| line.starts_with('|')).is_some() {
            self.lines.next_if(|(_, line)| line.starts_with('|'));
            while let Some((index, line)) = self.lines.next_if(|(_, line)| line.starts_with('|')) {
                arguments.push(argument(line).map_err(|expected| ParseError { line: index + 1, expected })?);
            }
        }
        Ok(CommandSpec { path, operation, outcome, doc, arguments })
    }
}

fn code(text: &str) -> Option<&str> {
    text.strip_prefix('`')?.strip_suffix('`')
}

fn parse_outcome(text: &str) -> Option<Outcome> {
    match text {
        "total" => Some(Outcome::Total),
        "fallible" => Some(Outcome::Fallible(1)),
        "fallible by error" => Some(Outcome::FallibleByError),
        _ => text.strip_prefix("fallible with status ")?.parse().ok().map(Outcome::Fallible),
    }
}

fn argument(line: &str) -> Result<ArgumentSpec, String> {
    let cells = cells(line);
    let [name, source, shape, env, default, delimiter, doc] =
        <[String; 7]>::try_from(cells).map_err(|_| format!("a row of {} cells", HEADER.len()))?;
    let optional_code = |cell: &str| -> Result<Option<String>, String> {
        if cell.is_empty() {
            return Ok(None);
        }
        code(cell).map(|text| Some(text.to_owned())).ok_or_else(|| format!("`{cell}` in backticks"))
    };

    let name = code(&name).ok_or("the argument name in backticks")?.to_owned();
    let source = match source.as_str() {
        "positional" => Source::Positional,
        spelled => {
            let mut spellings = spelled.split(", ").map(code);
            let long = spellings.next().flatten().and_then(|long| long.strip_prefix("--"));
            let long = long.ok_or("positional, or `--long` and maybe `-s`")?.to_owned();
            let short = match spellings.next() {
                None => None,
                Some(short) => {
                    let short = short.and_then(|short| short.strip_prefix('-')).ok_or("`-s`")?;
                    let mut chars = short.chars();
                    Some(chars.next().filter(|_| chars.next().is_none()).ok_or("a one-letter `-s`")?)
                }
            };
            Source::Named { long, short }
        }
    };
    let shape = match shape.as_str() {
        "one" => Shape::One,
        "optional" => Shape::Optional,
        "many" => Shape::Many,
        "switch" => Shape::Switch,
        _ => return Err("one, optional, many, or switch".to_owned()),
    };
    let delimiter = match optional_code(&delimiter)? {
        None => None,
        Some(text) => {
            let mut chars = text.chars();
            Some(chars.next().filter(|_| chars.next().is_none()).ok_or("one delimiter character")?)
        }
    };
    Ok(ArgumentSpec {
        name,
        source,
        shape,
        env: optional_code(&env)?,
        default: optional_code(&default)?,
        delimiter,
        doc: doc.replace("\\|", "|"),
    })
}

/// Splits a table row on the bars no backslash escapes.
fn cells(line: &str) -> Vec<String> {
    let inner = line.trim().strip_prefix('|').unwrap_or(line);
    let inner = inner.strip_suffix('|').unwrap_or(inner);
    let mut cells = vec![String::new()];
    let mut escaped = false;
    for char in inner.chars() {
        match char {
            '|' if !escaped => cells.push(String::new()),
            _ => cells.last_mut().expect("one cell at least").push(char),
        }
        escaped = char == '\\';
    }
    cells.into_iter().map(|cell| cell.trim().to_owned()).collect()
}
