//! Markdown as a notation for [`CliSpec`]: [`render`] writes a spec, and [`parse`] reads the same
//! format back, so `parse(&render(spec)) == spec` for every spec.
//!
//! ```markdown
//! # notes
//!
//! ## `notes add`
//!
//! Adds a note to the store.
//!
//! - Operation: `add_note`
//! - Outcome: fallible
//!
//! | Argument | Source          | Shape | Variable      | Default    | Delimiter | Help              |
//! | -------- | --------------- | ----- | ------------- | ---------- | --------- | ----------------- |
//! | `store`  | `--store`, `-s` | one   | `NOTES_STORE` | `notes.db` |           | Store to write to |
//! | `text`   | positional      | one   |               |            |           | Text of the note  |
//! ```

mod docs;
mod parse;
mod render;

pub use docs::*;
pub use notes_cli_describe::CliSpec;
pub use parse::*;
pub use render::*;
