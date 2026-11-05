use notes::{Note, NotebookAlg, Refusal};
use std::cell::RefCell;
use std::collections::BTreeMap;

/// Every store's notes, held in memory.
#[derive(Default)]
pub struct NotesMemory {
    stores: RefCell<BTreeMap<String, Vec<Note>>>,
}

impl NotebookAlg for NotesMemory {
    fn notebook_read(&self, store: &str) -> Result<Vec<Note>, Refusal> {
        Ok(self.stores.borrow().get(store).cloned().unwrap_or_default())
    }

    fn notebook_write(&self, store: &str, notes: &[Note]) -> Result<(), Refusal> {
        self.stores.borrow_mut().insert(store.to_owned(), notes.to_vec());
        Ok(())
    }
}
