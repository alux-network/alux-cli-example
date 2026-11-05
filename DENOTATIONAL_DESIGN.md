# Design by Meaning

This workspace is written in the style described by [ALUX programming](https://alux-network.github.io/alux-programming/): it states what a program means before it chooses how the program runs. The useful unit of reuse is therefore a meaning, not a file format, parser, or command-line framework.

## The shape of a specification

```text
words and values
    -> capability algebras: primitive observations and atomic changes
    -> extensions: operations composed from those capabilities
    -> programs: first-order terms over the operations, such as a command line
    -> interpretations: a representation and an execution strategy
    -> laws: observable properties every interpretation must satisfy
```

An algebra is a Rust trait. Its methods state the smallest facts and state changes an interpretation must answer. It has no supertraits or implementation bounds: an interpreter is free to choose its carrier types and storage.

The notes app has one: a notebook reads and writes the notes of a named store.

```rust ignore
/// Holds the notes of every store, each store named.
pub trait NotebookAlg {
    /// Reads the notes `store` holds, in the order they were written; a store never written holds
    /// none.
    fn notebook_read(&self, store: &str) -> Result<Vec<Note>, Refusal>;
    /// Replaces the notes `store` holds with `notes`.
    fn notebook_write(&self, store: &str, notes: &[Note]) -> Result<(), Refusal>;
}
```

An extension is an `#[ext]` implementation. Its bounds say exactly what the derived operation means; its body combines those capabilities and has no representation algorithm. Adding, listing, removing, and searching notes are extensions over `NotebookAlg`, so a memory map and a store file make the same operations true without sharing any implementation code.

## Programs are terms

`#[ext(defunc)]` also reifies each operation as a first-order value, `Op_add_note` for `add_note`. A command line is then a term over those values: an `alux-cli` program, itself an extension over `This: CliCommandsAlg`, that says which command applies which operation and where each argument comes from.

Because the program is a term, it has several interpretations:

- `alux-cli-direct` runs it as a command line.
- `Describe` folds it into a `CliSpec`, a plain value.
- `render` writes that value as Markdown, and `parse` reads it back.
- `generate` writes the Rust declaration of that value, which the macros expand in place.

So one command line can be declared in Rust, stated in Markdown, or both merged, and each statement is checked against the others by folding it back to the same Markdown.

## Specifications and interpretations

A top-level crate states a specification. It may depend only on other specifications, and it names no file format, parser, or runtime. [`notes`](notes) states the notebook and its operations, [`notes-search`](notes-search) states finding notes over it, and the three `notes-cli-*` crates state the command line. [`crates/notes-files`](crates/notes-files) interprets the notebook over a file.

The name `x-y` reads "`x` over `y`." When `y` is another specification, `x-y` is a specification too. When `y` is a technology, it is an interpreter. This is why `notes-cli-md` is a specification but `notes-cli-md-direct` is an interpreter.

An interpreter should be deliberately boring: it chooses carriers, encodes values, and implements each primitive once. It delegates derived behavior back to the extension that defines it.

## Laws

Tests are evidence; laws state the portable evidence. [`notes-conformance`](crates/notes-conformance) exposes `law_*` methods generic over `NotebookAlg`, and every interpreter runs them. A new semantic promise therefore needs a law before an interpreter can claim to implement it. The notations have laws of their own: `render` and `parse` are inverse, and every statement of the command line folds back to one `cli.md`.

The rules that follow from it are in [AGENTS.md](AGENTS.md).
