# notes

## `notes add`

Adds a note to the store.

- Operation: `add_note`
- Outcome: fallible

| Argument | Source           | Shape  | Variable      | Default    | Delimiter | Help                    |
| -------- | ---------------- | ------ | ------------- | ---------- | --------- | ----------------------- |
| `store`  | `--store`, `-s`  | one    | `NOTES_STORE` | `notes.db` |           | Store to read and write |
| `text`   | positional       | one    |               |            |           | Text of the note        |
| `tags`   | `--tag`, `-t`    | many   |               |            |           | Tag the note carries    |
| `pinned` | `--pinned`, `-p` | switch |               |            |           | Pin the note            |

## `notes list`

Lists the notes in the store, oldest first.

- Operation: `list_notes`
- Outcome: fallible

| Argument | Source          | Shape    | Variable      | Default    | Delimiter | Help                     |
| -------- | --------------- | -------- | ------------- | ---------- | --------- | ------------------------ |
| `store`  | `--store`, `-s` | one      | `NOTES_STORE` | `notes.db` |           | Store to read and write  |
| `tag`    | positional      | optional |               |            |           | Only notes with this tag |
| `limit`  | `--limit`, `-n` | one      |               | `10`       |           | How many notes           |

## `notes remove`

Removes a note from the store.

- Operation: `remove_note`
- Outcome: fallible

| Argument | Source          | Shape | Variable      | Default    | Delimiter | Help                    |
| -------- | --------------- | ----- | ------------- | ---------- | --------- | ----------------------- |
| `store`  | `--store`, `-s` | one   | `NOTES_STORE` | `notes.db` |           | Store to read and write |
| `id`     | positional      | one   |               |            |           | Number of the note      |

## `notes search`

Finds the notes holding every word, pinned first.

- Operation: `search_notes`
- Outcome: fallible

| Argument | Source          | Shape | Variable      | Default    | Delimiter | Help                    |
| -------- | --------------- | ----- | ------------- | ---------- | --------- | ----------------------- |
| `store`  | `--store`, `-s` | one   | `NOTES_STORE` | `notes.db` |           | Store to read and write |
| `words`  | positional      | many  |               |            |           | Words the note holds    |
| `limit`  | `--limit`, `-n` | one   |               | `10`       |           | How many notes          |

## `notes docs`

Prints this command line's spec as Markdown.

- Operation: `docs`
- Outcome: total
