/// A command line as data: its program name and every command, in declaration order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CliSpec {
    /// The program's name, the first word of its command line.
    pub name: String,
    /// Every command, the root command (empty path) included.
    pub commands: Vec<CommandSpec>,
}

/// One command: where it sits, the operation it applies, and where each argument comes from.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandSpec {
    /// The subcommand names leading to the command; empty for the program's own command.
    pub path: Vec<String>,
    /// The extension method the command applies, in snake case.
    pub operation: String,
    /// How the operation's output is read as the command's outcome.
    pub outcome: Outcome,
    /// The documentation of the operation, which is the command's help.
    pub doc: String,
    /// Every argument, in the operation's order.
    pub arguments: Vec<ArgumentSpec>,
}

/// How a command's output is read: `.fallible()`, `.fallible_as(status)`, `.fallible_by_error()`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// The output is the result; the command always succeeds.
    Total,
    /// The output is a `Result`; `Err` exits with this status.
    Fallible(u8),
    /// The output is a `Result`; `Err` exits with the status its error states.
    FallibleByError,
}

/// One argument and the source its words come from.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArgumentSpec {
    /// The operation's argument name.
    pub name: String,
    /// Where its words come from on the command line.
    pub source: Source,
    /// How many occurrences it takes.
    pub shape: Shape,
    /// The environment variable read when the command line gives none.
    pub env: Option<String>,
    /// The word used last.
    pub default: Option<String>,
    /// The character one word is split on, for a list.
    pub delimiter: Option<char>,
    /// What the argument means.
    pub doc: String,
}

/// Where an argument's words come from on the command line.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Source {
    /// The next positional words.
    Positional,
    /// The words after `--long` or `-short`.
    Named {
        /// The long spelling, without `--`.
        long: String,
        /// The short spelling, without `-`.
        short: Option<char>,
    },
}

/// How many occurrences an argument takes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shape {
    /// Exactly one value.
    One,
    /// At most one value.
    Optional,
    /// Any number of values.
    Many,
    /// Whether its spelling is given.
    Switch,
}

impl Source {
    /// The long spelling an argument gets when its declaration states none: its name in kebab case.
    #[must_use]
    pub fn default_long(name: &str) -> String {
        name.replace('_', "-")
    }
}
