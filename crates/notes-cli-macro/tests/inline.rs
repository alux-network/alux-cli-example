//! A program declared from inline Markdown.

use notes_cli_macro::markdown_cli;
use notes_cli_markdown::Op_docs;

markdown_cli!(
    r"# tiny

## `tiny`

Prints this command line's spec as Markdown.

- Operation: `docs`
- Outcome: total
"
);

// The declaration exists as a bound.
const _: () = {
    fn _declared<This: TinyCliExt<Machine>, Machine>() {}
};
