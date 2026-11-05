use alux_ext::ext;

/// States a program's own command line as Markdown.
pub trait CliDocsAlg {
    /// The program's spec as Markdown.
    fn cli_docs(&self) -> String;
}

/// The `docs` operation: a program printing its own spec.
#[ext(name = CliDocsExt, defunc)]
pub impl<This> This
where
    This: CliDocsAlg,
{
    /// Prints this command line's spec as Markdown.
    fn docs(&self) -> String {
        // The interpreter ends what it writes with the line the Markdown would end with.
        self.cli_docs().trim_end().to_owned()
    }
}
