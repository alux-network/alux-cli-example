//! Folds an ALUX CLI program into [`CliSpec`], the spec it states as data.
//!
//! [`Describe`] is one more interpreter of `alux-cli`'s algebras, beside the direct and clap ones:
//! it applies nothing and answers what each command is. The spec is the meeting point of every
//! notation in this workspace: Markdown renders and reads it, and Rust declarations are generated
//! from it, so a program written either way folds to the same value.

mod describe;
mod spec;

pub use describe::*;
pub use spec::*;
