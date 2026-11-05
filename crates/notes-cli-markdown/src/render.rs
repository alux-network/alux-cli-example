use core::array::from_fn;
use core::fmt::Write;
use core::iter::once;
use notes_cli_describe::{ArgumentSpec, CliSpec, CommandSpec, Outcome, Shape, Source};

/// The header of every argument table.
pub(crate) const HEADER: [&str; 7] = ["Argument", "Source", "Shape", "Variable", "Default", "Delimiter", "Help"];

/// Renders `spec` as Markdown: a title, then one section per command.
#[must_use]
pub fn render(spec: &CliSpec) -> String {
    let mut out = format!("# {}\n", spec.name);
    for command in &spec.commands {
        out.push('\n');
        render_command(&spec.name, command, &mut out);
    }
    out
}

fn render_command(program: &str, command: &CommandSpec, out: &mut String) {
    let path = once(program).chain(command.path.iter().map(String::as_str));
    let _ = writeln!(out, "## `{}`\n", path.collect::<Vec<_>>().join(" "));
    if !command.doc.is_empty() {
        let _ = writeln!(out, "{}\n", command.doc);
    }
    let _ = writeln!(out, "- Operation: `{}`", command.operation);
    let _ = writeln!(out, "- Outcome: {}", outcome(command.outcome));
    if command.arguments.is_empty() {
        return;
    }
    let rows = command.arguments.iter().map(cells).collect::<Vec<_>>();
    // Each column is as wide as its widest cell, so the table reads aligned as plain text.
    let widths: [usize; 7] = from_fn(|column| {
        rows.iter().map(|row| row[column].chars().count()).chain([HEADER[column].len()]).max().unwrap_or(0)
    });
    let _ = writeln!(out, "\n{}", row(&HEADER.map(String::from), widths));
    let _ = writeln!(out, "{}", row(&widths.map(|width| "-".repeat(width)), widths));
    for cells in &rows {
        let _ = writeln!(out, "{}", row(cells, widths));
    }
}

pub(crate) fn outcome(outcome: Outcome) -> String {
    match outcome {
        Outcome::Total => "total".to_owned(),
        Outcome::Fallible(1) => "fallible".to_owned(),
        Outcome::Fallible(status) => format!("fallible with status {status}"),
        Outcome::FallibleByError => "fallible by error".to_owned(),
    }
}

pub(crate) fn shape(shape: Shape) -> &'static str {
    match shape {
        Shape::One => "one",
        Shape::Optional => "optional",
        Shape::Many => "many",
        Shape::Switch => "switch",
    }
}

fn cells(argument: &ArgumentSpec) -> [String; 7] {
    let code = |text: &str| format!("`{text}`");
    let source = match &argument.source {
        Source::Positional => "positional".to_owned(),
        Source::Named { long, short: None } => code(&format!("--{long}")),
        Source::Named { long, short: Some(short) } => {
            format!("{}, {}", code(&format!("--{long}")), code(&format!("-{short}")))
        }
    };
    [
        code(&argument.name),
        source,
        shape(argument.shape).to_owned(),
        argument.env.as_deref().map(code).unwrap_or_default(),
        argument.default.as_deref().map(code).unwrap_or_default(),
        argument.delimiter.map(|delimiter| code(&delimiter.to_string())).unwrap_or_default(),
        argument.doc.replace('|', "\\|"),
    ]
}

fn row(cells: &[String; 7], widths: [usize; 7]) -> String {
    let cells = cells.iter().zip(widths).map(|(cell, width)| format!("{cell:width$}"));
    format!("| {} |", cells.collect::<Vec<_>>().join(" | "))
}
