//! Markdown reads back every spec it renders, and renders back every Markdown it reads.

use notes_cli_describe::{ArgumentSpec, CliSpec, CommandSpec, Outcome, Shape, Source};
use notes_cli_markdown::{parse, render};

fn argument(name: &str, source: Source, shape: Shape) -> ArgumentSpec {
    ArgumentSpec {
        name: name.to_owned(),
        source,
        shape,
        env: None,
        default: None,
        delimiter: None,
        doc: format!("The {name}"),
    }
}

fn named(long: &str, short: Option<char>) -> Source {
    Source::Named { long: long.to_owned(), short }
}

/// A spec touching every cell the Markdown states.
fn spec() -> CliSpec {
    CliSpec {
        name: "shelf".to_owned(),
        commands: vec![
            CommandSpec {
                path: Vec::new(),
                operation: "open_shelf".to_owned(),
                outcome: Outcome::Total,
                doc: "Opens the shelf.\n\nA second paragraph, with a | bar.".to_owned(),
                arguments: vec![argument("path", Source::Positional, Shape::Optional)],
            },
            CommandSpec {
                path: vec!["put".to_owned()],
                operation: "put_book".to_owned(),
                outcome: Outcome::Fallible(3),
                doc: "Puts a book.".to_owned(),
                arguments: vec![
                    ArgumentSpec {
                        env: Some("SHELF_TITLES".to_owned()),
                        delimiter: Some(','),
                        doc: "Titles, a | b".to_owned(),
                        ..argument("titles", named("title", Some('t')), Shape::Many)
                    },
                    ArgumentSpec {
                        default: Some("1".to_owned()),
                        ..argument("copies", named("copies", None), Shape::One)
                    },
                    argument("dry_run", named("dry-run", Some('n')), Shape::Switch),
                ],
            },
            CommandSpec {
                path: vec!["count".to_owned()],
                operation: "count".to_owned(),
                outcome: Outcome::FallibleByError,
                doc: String::new(),
                arguments: Vec::new(),
            },
            CommandSpec {
                path: vec!["check".to_owned()],
                operation: "check".to_owned(),
                outcome: Outcome::Fallible(1),
                doc: "Checks.".to_owned(),
                arguments: Vec::new(),
            },
        ],
    }
}

#[test]
fn reads_back_every_spec_it_renders() {
    assert_eq!(parse(&render(&spec())), Ok(spec()));
}

#[test]
fn renders_back_every_markdown_it_reads() {
    let markdown = render(&spec());
    assert_eq!(render(&parse(&markdown).expect("rendered Markdown reads")), markdown);
}

#[test]
fn names_the_line_that_states_nothing() {
    let markdown = render(&spec()).replace("| many ", "| lots ");
    let error = parse(&markdown).expect_err("an unknown shape does not read");
    assert_eq!(markdown.lines().nth(error.line - 1).map(|line| line.contains("lots")), Some(true));
}
