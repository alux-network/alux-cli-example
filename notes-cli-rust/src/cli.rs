use alux_cli::{CliCommandsAlg, cli};
use alux_ext::ext;
use notes::{ArgNames_add_note, ArgNames_list_notes, ArgNames_remove_note, Op_add_note, Op_list_notes, Op_remove_note};
use notes_cli_markdown::Op_docs;
use notes_search::{ArgNames_search_notes, Op_search_notes};

/// Describes the `store` argument.
const STORE: &str = "Store to read and write";

/// Describes the `text` argument of `add`.
const TEXT: &str = "Text of the note";

/// Describes the `tags` argument of `add`.
const TAGS: &str = "Tag the note carries";

/// Describes the `pinned` argument of `add`.
const PINNED: &str = "Pin the note";

/// Describes the `tag` argument of `list`.
const TAG: &str = "Only notes with this tag";

/// Describes the `limit` argument of `list` and `search`.
const LIMIT: &str = "How many notes";

/// Describes the `id` argument of `remove`.
const ID: &str = "Number of the note";

/// Describes the `words` argument of `search`.
const WORDS: &str = "Words the note holds";

/// The notes command line, declared in Rust.
#[ext(name = NotesCliExt, defunc(via = cli))]
pub impl<This> This
where
    This: CliCommandsAlg,
{
    /// Declares the `notes` command line.
    #[rustfmt::skip]
    fn notes_cli<Machine>(&self) {
        // Each argument reads its flag, then its variable, then its default.
        //       argument   help                      shape       flag                     variable               default
        let add_note = self.op(Machine::add_note)
            .arg("store",   self.named(STORE)                     .short('s')              .env("NOTES_STORE")    .default("notes.db"))
            .arg("text",    self.positional(TEXT))
            .arg("tags",    self.named(TAGS)          .many()     .long("tag") .short('t'))
            .arg("pinned",  self.named(PINNED)        .switch()                .short('p'))
            .fallible();
        let list_notes = self.op(Machine::list_notes)
            .arg("store",   self.named(STORE)                     .short('s')              .env("NOTES_STORE")    .default("notes.db"))
            .arg("tag",     self.positional(TAG)      .optional())
            .arg("limit",   self.named(LIMIT)                     .short('n')                                     .default("10"))
            .fallible();
        let remove_note = self.op(Machine::remove_note)
            .arg("store",   self.named(STORE)                     .short('s')              .env("NOTES_STORE")    .default("notes.db"))
            .arg("id",      self.positional(ID))
            .fallible();
        let search_notes = self.op(Machine::search_notes)
            .arg("store",   self.named(STORE)                     .short('s')              .env("NOTES_STORE")    .default("notes.db"))
            .arg("words",   self.positional(WORDS)    .many())
            .arg("limit",   self.named(LIMIT)                     .short('n')                                     .default("10"))
            .fallible();
        let docs = self.op(Machine::docs);

        self.commands()
            .cmd("add",    add_note)
            .cmd("list",   list_notes)
            .cmd("remove", remove_note)
            .cmd("search", search_notes)
            .cmd("docs",   docs)
    }
}
