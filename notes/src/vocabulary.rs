use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// A note: its number in its store, its text, the tags it carries, and whether it is pinned.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Note {
    /// The note's number, unique in its store.
    pub id: u64,
    /// The tags the note carries, without their `#`.
    pub tags: Vec<String>,
    /// Whether the note is pinned.
    pub pinned: bool,
    /// The note's text.
    pub text: String,
}

/// Shows a note on one line: its number, a `*` when pinned, its text, then its tags.
impl Display for Note {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)?;
        if self.pinned {
            write!(f, " *")?;
        }
        write!(f, " {}", self.text)?;
        self.tags.iter().try_for_each(|tag| write!(f, " #{tag}"))
    }
}

/// Why a store refused what was asked of it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Refusal {
    /// No note has the number.
    Missing(u64),
    /// A note needs text.
    Empty,
    /// The store could not be read or written, for the reason given.
    Store(String),
}

impl Display for Refusal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing(id) => write!(f, "no note {id}"),
            Self::Empty => write!(f, "a note needs text"),
            Self::Store(reason) => write!(f, "{reason}"),
        }
    }
}

impl Error for Refusal {}
