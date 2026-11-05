# ALUX CLI example guide

This file is the compact engineering authority for humans and coding agents working on this workspace. Apply it to the whole repository.

## Product

A notes app whose command line is one [`alux-cli`](https://docs.rs/alux-cli) spec stated in two notations, Rust and Markdown, each compiling to the other, and in both at once. It is an example: the layout and the rules are those of [`alux-rust`](https://github.com/alux-network/alux-rust). [`README.md`](README.md) says what each crate is for, and [`DENOTATIONAL_DESIGN.md`](DENOTATIONAL_DESIGN.md) explains the programming model used below.

## Workspace

A specification is named for what it means; `x-y` is `x` over `y`, a specification when `y` is one and an interpreter when `y` is a technology. Specifications sit at the top level, interpreters under `crates/`.

```text
notes-cli-*-direct -> notes-cli-rust, notes-cli-md -> notes-search -> notes
notes-cli-*-direct -> notes-files -> notes
notes-memory, notes-files -(tests)-> notes-conformance
notes-cli-md-tool, notes-cli-* -> notes-cli-macro -> notes-cli-rust-gen -> notes-cli-markdown -> notes-cli-describe -> alux-cli
```

| Crate | Kind | Responsibility |
| --- | --- | --- |
| `notes` | Spec | Notes in named stores: `NotebookAlg` is the one primitive, every operation an extension over it |
| `notes-search` | Spec | Finding notes by their words, over `notes` |
| `notes-cli-rust` | Spec | The notes command line, declared in Rust |
| `notes-cli-md` | Spec | The same command line, stated in Markdown in `src/cli.md` |
| `notes-cli-mixed` | Spec | The same command line, Rust and inline Markdown merged in code |
| `notes-conformance` | Laws | The laws every `NotebookAlg` interpreter passes |
| `notes-memory`, `notes-files` | Interpreter | `NotebookAlg` in memory and in a store file |
| `notes-cli-rust-direct`, `notes-cli-md-direct` | Binary | The notes app, run by `alux-cli-direct` over `notes-files` |
| `notes-cli-describe` | Interpreter | `Describe` and `CliSpec`, the value every notation meets at |
| `notes-cli-markdown` | Interpreter | `render` and `parse`, inverse on every spec, and the `docs` operation |
| `notes-cli-rust-gen` | Interpreter | `generate`, the Rust declaration of a `CliSpec` |
| `notes-cli-macro` | Interpreter | `markdown_cli_src!` and `markdown_cli!`, that declaration expanded from a file or inline Markdown |
| `notes-cli-md-tool` | Binary | A tool over Markdown specs whose own command line is one |

## Authority

Read authority in this order:

1. The documented meaning of an algebra, its methods, and its laws.
2. Public trait signatures and vocabulary types.
3. Extension bounds and bodies, which define derived meaning.
4. The laws in `notes-conformance` and the round-trip tests.
5. Interpreters, which witness the meaning.
6. Prose, which summarizes the above.

`CliSpec` is the authority over every notation. A notation that cannot state part of a program reports it by the operation that uses it, never drops it.

If these disagree, state the mismatch and fix the smallest authoritative layer that is wrong.

## Commands

```sh
just fmt
just build
just clippy
just doc
just test
just package
```

`just ci` runs all six in that order, as CI does. Run the narrowest relevant test during development (`just test -p notes-cli-mixed`) and `just ci` before finishing. The [`justfile`](justfile) holds the full command every recipe stands for.

## Design by Meaning

This follows the [ALUX programming guidelines](https://alux-network.github.io/alux-programming/) and the [`alux-rust` guide](https://github.com/alux-network/alux-rust/blob/main/AGENTS.md).

- Ask what the notes app means before asking how a file or a parser achieves it. The test for every operation: would it mean the same over a file, a database, or plain memory?
- Traits are algebras. Each names primitive observations and atomic transitions over carriers its interpreter chooses.
- Extensions are composition only. Their `where` clauses are their semantic dependencies; they carry no storage algorithm. The carrier is `This` with a `where` clause.
- A term built over an algebra (a command line, a law) is an extension method over `This`, never a free generic function taking the algebra as an argument.
- No trait inheritance, no supertraits on `#[ext]`, and no bounds in trait definitions. Bounds go on extension impls.
- A spec crate holds traits, `#[ext]` extensions, and data types, in that order. It holds no model implementation; a carrier lives under `crates/`.
- Laws are parameterized equivalences over the capability bounds, returning `anyhow::Result`, never enumerated value lists. Run each law against at least two interpreters.
- Interpreters are thin: carriers, storage, and one implementation per primitive. Derived behavior comes from the specification's extensions.

## Command-line notations

- A command line is an `alux-cli` program: an `#[ext(defunc(via = cli))]` extension over `This: CliCommandsAlg` whose `Machine::<operation>` names an operation `#[ext(defunc)]` generated as `Op_<operation>`, with its argument names as `ArgNames_<operation>`.
- Every notation states one spec. `notes-cli-rust`, `notes-cli-md`, and `notes-cli-mixed` each fold back to `notes-cli-md/src/cli.md`, and a test holds each to it byte for byte.
- A Markdown definition's `cli.md` is its source; change the Markdown, never the Rust the macro expands it to. A macro path is relative to the crate root (`markdown_cli_src!("src/cli.md")`).
- Programs in either notation compose: a program stated in Markdown merges with commands declared in Rust like any other program.
- When a renderer change moves the canonical Markdown, regenerate each `cli.md` with its binary's `docs` command and keep every round-trip test passing.

## Rust conventions

- Inherit edition, minimum Rust version, license, and common metadata from the workspace manifest. Declare dependencies in `[workspace.dependencies]`.
- Root modules define the product surface: `lib.rs` holds documentation, private modules, and intentional `pub use`.
- Import with module-top `use` and short names; no inline multi-segment paths in code.
- Order declarations and imports by visibility, with blank lines only between visibility groups.
- Public items require documentation describing meaning, in US English, beginning with a third-person singular verb. No em dashes.
- Wrap comments at 100 columns. Do not add section divider comments.
- A crate's `README.md` is its crate documentation.
- Unsafe code is forbidden.

## Tests and laws

- A law lives in `notes-conformance` as a `law_*` method of `NotesLawsExt`, taking what it is about as arguments, and `laws` runs them all. Each interpreter runs them in its tests: `notes-memory` in memory, `notes-files` in a temporary directory.
- A law drives an interpreter only through the specification, never through its representation.
- The notations are held by round trips: `parse(render(spec)) == spec`, `render(parse(markdown)) == markdown`, and every binary's `docs` equal to the `cli.md` it states.
- Tests are evidence, not proof. Add a law when a meaning is added.

## Change workflow

1. State the semantic change in one sentence.
2. Find the smallest algebra, vocabulary, or extension it needs.
3. Change the specification first.
4. Add or change the law.
5. Implement the primitive in every interpreter, and the command in every notation.
6. Run `just ci`.

Use Conventional Commits 1.0.0: `<type>(<scope>): <imperative summary>`.
