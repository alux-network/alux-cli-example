use super::*;
use notes_conformance::NotesLawsExt;

/// Notes held in memory satisfy every law.
#[test]
fn satisfies_the_laws() {
    NotesMemory::default().laws(str::to_owned).expect("a law holds");
}
