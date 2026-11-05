# notes

Notes kept in a store, each with a number, its text, the tags it carries, and whether it is pinned.

- `NotebookAlg` reads and writes the notes a store holds; an interpreter decides what a store is.
- `NotesExt` adds, lists, and removes notes: numbers count up from 1 and are never reused while a later note is held, listing is oldest first, and a note with no text is refused.
