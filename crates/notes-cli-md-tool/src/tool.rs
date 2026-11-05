use crate::NotesCliMdToolCliExt;
use alux_cli::CliProgramBuilder;
use alux_ext::ext;
use anyhow::{Context, Result, bail};
use notes_cli_describe::Describe;
use notes_cli_markdown::{CliDocsAlg, parse, render};
use notes_cli_rust_gen::generate;
use std::fs;
use std::path::Path;

// The generated declaration applies this operation as `Machine::docs`.
pub(crate) use notes_cli_markdown::Op_docs;

/// Reads a Markdown spec from a file.
pub trait SpecFilesAlg {
    /// Reads the spec at `path`.
    ///
    /// # Errors
    ///
    /// Answers with the I/O error that stopped the read.
    fn read_spec(&self, path: &Path) -> Result<String> {
        fs::read_to_string(path).with_context(|| format!("cannot read {}", path.display()))
    }
}

/// The tool's operations over Markdown specs.
#[ext(name = SpecToolExt, defunc)]
pub impl<This> This
where
    This: SpecFilesAlg,
{
    /// Prints the Rust declaration a Markdown spec compiles to.
    fn rust(&self, markdown: String) -> Result<String> {
        let spec = parse(&self.read_spec(Path::new(&markdown))?)?;
        Ok(generate(&spec)?)
    }

    /// Checks that a Markdown spec reads back exactly as it is written.
    fn check(&self, markdown: String) -> Result<String> {
        let text = self.read_spec(Path::new(&markdown))?;
        let rendered = render(&parse(&text)?);
        if let Some((line, (written, read))) =
            text.lines().zip(rendered.lines()).enumerate().find(|(_, (written, read))| written != read)
        {
            bail!("line {}: written `{written}`, reads back as `{read}`", line + 1);
        }
        if text.lines().count() != rendered.lines().count() {
            bail!("{} lines written, {} read back", text.lines().count(), rendered.lines().count());
        }
        Ok(format!("{markdown} reads back as written"))
    }
}

/// The tool: specs read from files, and its own spec.
pub struct Tool;

impl SpecFilesAlg for Tool {}

impl CliDocsAlg for Tool {
    fn cli_docs(&self) -> String {
        let spec = Describe::spec("notes-cli-md-tool", CliProgramBuilder.notes_cli_md_tool_cli::<Tool>());
        render(&spec.expect("the tool's command line states a spec"))
    }
}
