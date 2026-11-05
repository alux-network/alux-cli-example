# notes-cli-md-tool

## `notes-cli-md-tool rust`

Prints the Rust declaration a Markdown spec compiles to.

- Operation: `rust`
- Outcome: fallible

| Argument   | Source     | Shape | Variable | Default | Delimiter | Help                     |
| ---------- | ---------- | ----- | -------- | ------- | --------- | ------------------------ |
| `markdown` | positional | one   |          |         |           | Markdown spec to compile |

## `notes-cli-md-tool check`

Checks that a Markdown spec reads back exactly as it is written.

- Operation: `check`
- Outcome: fallible

| Argument   | Source     | Shape | Variable | Default | Delimiter | Help                   |
| ---------- | ---------- | ----- | -------- | ------- | --------- | ---------------------- |
| `markdown` | positional | one   |          |         |           | Markdown spec to check |

## `notes-cli-md-tool docs`

Prints this command line's spec as Markdown.

- Operation: `docs`
- Outcome: total
