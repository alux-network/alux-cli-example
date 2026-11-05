use crate::{ArgumentSpec, CliSpec, CommandSpec, Outcome, Shape, Source};
use alux_cli::{
    ArgumentInfo, Arity, CliCommandsAlg, CliNestAlg, CliOperationAlg, CliProgramAlg, CliSources, Conventions, Fallible,
    FallibleByError, SourceInfo, Total,
};
use alux_ext::OperationAlg;
use core::any::type_name;
use core::error::Error;
use core::fmt::{Display, Formatter, Result as FmtResult};

/// Describes a CLI program without applying any of it.
#[derive(Clone, Copy, Debug, Default)]
pub struct Describe;

/// A part of a program this spec has no words for yet, named by the command it is in.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unsupported {
    /// The operation whose declaration uses it.
    pub operation: String,
    /// What it is.
    pub what: &'static str,
}

impl Display for Unsupported {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "`{}` declares {}, which the spec cannot state yet", self.operation, self.what)
    }
}

impl Error for Unsupported {}

/// Every command described so far, or the first part no spec states.
pub type Described = Result<Vec<CommandSpec>, Unsupported>;

impl Describe {
    /// Describes `program` as the command line of `name`.
    ///
    /// # Errors
    ///
    /// Answers with the first part of the program the spec cannot state.
    pub fn spec<Program>(name: &str, program: Program) -> Result<CliSpec, Unsupported>
    where
        Program: CliProgramAlg<Self, Commands = Described>,
    {
        Ok(CliSpec { name: name.to_owned(), commands: program.compile_cli(&Self)? })
    }
}

impl CliCommandsAlg for Describe {
    type Commands = Described;

    fn cli_empty(&self) -> Described {
        Ok(Vec::new())
    }

    fn cli_merge(&self, left: Described, right: Described) -> Described {
        let mut left = left?;
        left.extend(right?);
        Ok(left)
    }
}

impl<Operation, Sources, Mode> CliOperationAlg<Operation, Sources, Mode> for Describe
where
    Operation: OperationAlg,
    Sources: CliSources<Operation::Args>,
    Mode: OutcomeDescribed,
{
    type Commands = Described;

    fn cli_operation(
        &self,
        name: &'static str,
        _: Operation,
        sources: Sources,
        mode: Mode,
        conventions: Conventions,
    ) -> Described {
        let operation = operation_name::<Operation>();
        let unsupported = |what| Unsupported { operation: operation.clone(), what };
        if conventions != Conventions::default() {
            return Err(unsupported("its own word conventions"));
        }
        let arguments = sources
            .cli_arguments(Operation::ARG_NAMES)
            .into_iter()
            .map(|argument| argument_spec(argument).map_err(unsupported))
            .collect::<Result<_, _>>()?;

        Ok(vec![CommandSpec {
            path: if name.is_empty() { Vec::new() } else { vec![name.to_owned()] },
            operation,
            outcome: mode.outcome(),
            doc: Operation::DOC.trim().to_owned(),
            arguments,
        }])
    }
}

impl CliNestAlg for Describe {
    type Commands = Described;

    fn cli_nest(&self, name: &'static str, commands: Described) -> Described {
        let mut commands = commands?;
        for command in &mut commands {
            command.path.insert(0, name.to_owned());
        }
        Ok(commands)
    }
}

/// States the outcome a declaration's mode reads its output as.
pub trait OutcomeDescribed {
    /// The outcome this mode reads.
    fn outcome(&self) -> Outcome;
}

impl OutcomeDescribed for Total {
    fn outcome(&self) -> Outcome {
        Outcome::Total
    }
}

impl OutcomeDescribed for FallibleByError {
    fn outcome(&self) -> Outcome {
        Outcome::FallibleByError
    }
}

/// `Fallible` keeps its status to itself, so the status is the one it equals.
impl OutcomeDescribed for Fallible {
    fn outcome(&self) -> Outcome {
        let status = (1..=u8::MAX).find(|status| Fallible::new(*status) == *self).unwrap_or(1);
        Outcome::Fallible(status)
    }
}

fn argument_spec(argument: ArgumentInfo) -> Result<ArgumentSpec, &'static str> {
    if argument.part_of.is_some() {
        return Err("a product or sum argument");
    }
    let source = match argument.source {
        SourceInfo::Positional { .. } => Source::Positional,
        SourceInfo::Named { short_aliases: [_, ..], .. } => return Err("short aliases"),
        SourceInfo::Named { long, short, .. } => Source::Named { long, short },
        SourceInfo::Assigned { .. } => return Err("an assigned operand"),
    };
    let shape = match argument.arity {
        Arity::One => Shape::One,
        Arity::Optional => Shape::Optional,
        Arity::Many => Shape::Many,
        Arity::Presence => Shape::Switch,
        Arity::OptionalValue => return Err("an optional value"),
    };
    Ok(ArgumentSpec {
        name: argument.name.to_owned(),
        source,
        shape,
        env: argument.fallback.env.map(str::to_owned),
        default: argument.fallback.default.map(str::to_owned),
        delimiter: argument.delimiter,
        doc: argument.doc.to_owned(),
    })
}

/// The extension method an operation reifies: `crate::Op_add_note<Notes>` is `add_note`.
fn operation_name<Operation>() -> String {
    let path = type_name::<Operation>();
    let path = path.split('<').next().unwrap_or(path);
    let ty = path.rsplit("::").next().unwrap_or(path);
    ty.strip_prefix("Op_").unwrap_or(ty).to_owned()
}
