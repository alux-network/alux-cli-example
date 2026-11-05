#![doc = include_str!("../README.md")]

use notes::{ArgNames_add_note, ArgNames_list_notes, ArgNames_remove_note, Op_add_note, Op_list_notes, Op_remove_note};
use notes_cli_macro::markdown_cli_src;
use notes_cli_markdown::Op_docs;
use notes_search::{ArgNames_search_notes, Op_search_notes};

// The generated declaration applies the imported operations as `Machine::<operation>`.
markdown_cli_src!("src/cli.md");
