use alux_ext::ext;
use anyhow::{Result, ensure};
use notes::{NotebookAlg, NotesExt, Refusal};
use notes_search::SearchExt;

/// Checks the laws every interpreter of notes satisfies, each on a store nobody wrote.
#[ext(name = NotesLawsExt)]
pub impl<This> This
where
    This: NotebookAlg,
{
    /// A store nobody wrote holds no notes.
    fn law_unwritten_store_is_empty(&self, store: &str) -> Result<()> {
        ensure!(self.list_notes(store.to_owned(), None, usize::MAX)?.is_empty(), "an unwritten store holds notes");
        Ok(())
    }

    /// Adding numbers notes from 1 up, and listing shows them oldest first, as they were written.
    fn law_added_notes_list_oldest_first(&self, store: &str) -> Result<()> {
        let first = self.add_note(store.to_owned(), "buy milk".to_owned(), vec!["home".to_owned()], false)?;
        let second = self.add_note(store.to_owned(), "call Ada".to_owned(), Vec::new(), true)?;
        ensure!((first, second) == (1, 2), "notes numbered {first} and {second}");
        let listed = self.list_notes(store.to_owned(), None, usize::MAX)?;
        ensure!(listed == ["1 buy milk #home", "2 * call Ada"], "listed {listed:?}");
        Ok(())
    }

    /// Listing by a tag shows only the notes carrying it, and never more than the limit.
    fn law_list_filters_by_tag_and_limit(&self, store: &str) -> Result<()> {
        for (text, tag) in [("a", "x"), ("b", "y"), ("c", "x"), ("d", "x")] {
            self.add_note(store.to_owned(), text.to_owned(), vec![tag.to_owned()], false)?;
        }
        let tagged = self.list_notes(store.to_owned(), Some("x".to_owned()), 2)?;
        ensure!(tagged == ["1 a #x", "3 c #x"], "listed {tagged:?}");
        Ok(())
    }

    /// A removed note is gone, removing it again is refused, and the next note's number follows
    /// the last one held.
    fn law_removed_notes_are_gone(&self, store: &str) -> Result<()> {
        self.add_note(store.to_owned(), "a".to_owned(), Vec::new(), false)?;
        self.add_note(store.to_owned(), "b".to_owned(), Vec::new(), false)?;
        self.remove_note(store.to_owned(), 1)?;
        ensure!(self.remove_note(store.to_owned(), 1) == Err(Refusal::Missing(1)), "removed a note twice");
        ensure!(self.list_notes(store.to_owned(), None, usize::MAX)? == ["2 b"], "the removed note is listed");
        ensure!(self.add_note(store.to_owned(), "c".to_owned(), Vec::new(), false)? == 3, "a number was reused");
        Ok(())
    }

    /// A note with no text is refused and leaves the store as it was.
    fn law_empty_notes_are_refused(&self, store: &str) -> Result<()> {
        ensure!(
            self.add_note(store.to_owned(), " ".to_owned(), Vec::new(), false) == Err(Refusal::Empty),
            "added an empty note"
        );
        ensure!(self.list_notes(store.to_owned(), None, usize::MAX)?.is_empty(), "an empty note is listed");
        Ok(())
    }

    /// Searching finds the notes holding every word, ignoring case, pinned first.
    fn law_search_finds_every_word_pinned_first(&self, store: &str) -> Result<()> {
        self.add_note(store.to_owned(), "Milk and bread".to_owned(), Vec::new(), false)?;
        self.add_note(store.to_owned(), "milk only".to_owned(), Vec::new(), false)?;
        self.add_note(store.to_owned(), "bread then milk".to_owned(), Vec::new(), true)?;
        let found = self.search_notes(store.to_owned(), vec!["MILK".to_owned(), "bread".to_owned()], usize::MAX)?;
        ensure!(found == ["3 * bread then milk", "1 Milk and bread"], "found {found:?}");
        Ok(())
    }

    /// Checks every law, each on its own store named from `store`.
    fn laws(&self, store: impl Fn(&str) -> String) -> Result<()> {
        self.law_unwritten_store_is_empty(&store("unwritten"))?;
        self.law_added_notes_list_oldest_first(&store("added"))?;
        self.law_list_filters_by_tag_and_limit(&store("tagged"))?;
        self.law_removed_notes_are_gone(&store("removed"))?;
        self.law_empty_notes_are_refused(&store("empty"))?;
        self.law_search_finds_every_word_pinned_first(&store("search"))
    }
}
