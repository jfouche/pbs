use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_while1},
    character::complete::{char, digit1, multispace0, space0, space1},
    combinator::{eof, map_res},
    error::ParseError,
    sequence::{delimited, pair, preceded, tuple},
    AsChar, Compare, IResult, InputLength, InputTake, InputTakeAtPosition, Parser,
};

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Command {
    Add(AddParams),
    AddChild(AddChildParams),
    List,
    Tree(TreeParams),
    WhereUsed(WhereUsedParams),
    Stock(StockParams),
    Help,
    Exit,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct AddParams {
    pub pn: String,
    pub name: String,
}

impl From<(&str, &str)> for AddParams {
    fn from(value: (&str, &str)) -> Self {
        AddParams {
            pn: value.0.to_string(),
            name: value.1.to_string(),
        }
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct AddChildParams {
    pub parent_pn: String,
    pub child_pn: String,
    pub quantity: usize,
}

impl From<(&str, &str, usize)> for AddChildParams {
    fn from(value: (&str, &str, usize)) -> Self {
        AddChildParams {
            parent_pn: value.0.to_string(),
            child_pn: value.1.to_string(),
            quantity: value.2,
        }
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct TreeParams {
    pub pn: String,
}

impl From<&str> for TreeParams {
    fn from(value: &str) -> Self {
        TreeParams {
            pn: value.to_string(),
        }
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct WhereUsedParams {
    pub pn: String,
}

impl From<&str> for WhereUsedParams {
    fn from(value: &str) -> Self {
        WhereUsedParams {
            pn: value.to_string(),
        }
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct StockParams {
    pub pn: String,
}

impl From<&str> for StockParams {
    fn from(value: &str) -> Self {
        StockParams {
            pn: value.to_string(),
        }
    }
}

/// Get the command of the input
pub fn get_command(input: &str) -> Result<Command, nom::Err<nom::error::Error<&str>>> {
    delimited(
        space0,
        alt((
            cmd_add,
            cmd_list,
            cmd_add_child,
            cmd_tree,
            cmd_help,
            cmd_exit,
            cmd_where_used,
            cmd_stock,
        )),
        eol,
    )(input)
    .map(|(_, cmd)| cmd)
}

// ====================================================================

/// parser for a cmd followed with params.
fn cmd<I, O, E: ParseError<I>, F, T>(cmd: T, parser: F) -> impl FnMut(I) -> IResult<I, O, E>
where
    I: InputTake + Compare<T>,
    T: InputLength + Clone,
    F: Parser<I, O, E>,
{
    preceded(tag(cmd), parser)
}

/// parser for a whitespace separated param
fn param<I, O, E: ParseError<I>, F>(mut parser: F) -> impl FnMut(I) -> IResult<I, O, E>
where
    I: InputTakeAtPosition,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
    F: Parser<I, O, E>,
{
    move |input: I| {
        let (input, _) = space1(input)?;
        parser.parse(input)
    }
}

/// A PN is alphanum and can contain `.`, `-` or `_`
fn pn(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphanumeric() || ".-_".find(c).is_some())(input)
}

#[allow(rustdoc::invalid_html_tags)]
/// get the <name> param, allowing direct
///
/// A name can start and end with `"`
fn name(input: &str) -> IResult<&str, &str> {
    alt((
        delimited(char('"'), take_till(|c| c == '"'), char('"')),
        take_while1(|c: char| c.is_alphanumeric() || ".-_".find(c).is_some()),
    ))(input)
}

/// Parser for a number
fn quantity(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

/// End of line parser
fn eol(input: &str) -> IResult<&str, ()> {
    pair(multispace0, eof)(input).map(|(i, (_, _))| (i, ()))
}

/// `add <pn> <name>`
fn cmd_add(input: &str) -> IResult<&str, Command> {
    cmd("add", pair(preceded(space1, pn), preceded(space1, name)))(input)
        .map(|(i, output)| (i, Command::Add(AddParams::from(output))))
}

/// `list`
fn cmd_list(input: &str) -> IResult<&str, Command> {
    tag("list")(input).map(|(i, _)| (i, Command::List))
}

/// `exit`
fn cmd_exit(input: &str) -> IResult<&str, Command> {
    tag("exit")(input).map(|(i, _)| (i, Command::Exit))
}

/// `help`
fn cmd_help(input: &str) -> IResult<&str, Command> {
    tag("help")(input).map(|(i, _)| (i, Command::Help))
}

/// `tree <pn>`
fn cmd_tree(input: &str) -> IResult<&str, Command> {
    cmd("tree", param(pn))(input).map(|(i, pn)| (i, Command::Tree(TreeParams::from(pn))))
}

/// `where-used <pn>`
fn cmd_where_used(input: &str) -> IResult<&str, Command> {
    cmd("where-used", param(pn))(input)
        .map(|(i, pn)| (i, Command::WhereUsed(WhereUsedParams::from(pn))))
}

/// `add-child <parent-pn> <child-pn> <quantity>`
fn cmd_add_child(input: &str) -> IResult<&str, Command> {
    cmd("add-child", tuple((param(pn), param(pn), param(quantity))))(input)
        .map(|(i, output)| (i, Command::AddChild(AddChildParams::from(output))))
}

/// `stock <pn>`
fn cmd_stock(input: &str) -> IResult<&str, Command> {
    preceded(tag("stock"), param(pn))(input)
        .map(|(i, pn)| (i, Command::Stock(StockParams::from(pn))))
}

/// =================================================================
/// Test
/// =================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pn() {
        assert_eq!(pn("PN"), Ok(("", "PN")));
        assert_eq!(pn("PN111"), Ok(("", "PN111")));
        assert_eq!(pn("PN111AA"), Ok(("", "PN111AA")));
        assert_eq!(pn("PN111-AA"), Ok(("", "PN111-AA")));
        assert_eq!(pn("PN111_AA"), Ok(("", "PN111_AA")));
        assert_eq!(pn("PN111.AA"), Ok(("", "PN111.AA")));
        assert_eq!(pn("PN111$AA"), Ok(("$AA", "PN111")));
    }

    #[test]
    fn test_name() {
        assert_eq!(name("NAME"), Ok(("", "NAME")));
        assert_eq!(name("\"NAME\""), Ok(("", "NAME")));
        assert_eq!(
            name("\"NAME WITH - _ CHARS\""),
            Ok(("", "NAME WITH - _ CHARS"))
        );
    }

    #[test]
    fn test_list_ok() {
        assert_eq!(Command::List, get_command("list").unwrap());
        assert_eq!(Command::List, get_command("  list").unwrap());
        assert_eq!(Command::List, get_command("  \tlist \t ").unwrap());
    }

    #[test]
    fn test_exit_ok() {
        assert_eq!(Command::Exit, get_command("exit").unwrap());
        assert_eq!(Command::Exit, get_command("  exit").unwrap());
        assert_eq!(Command::Exit, get_command("  exit  ").unwrap());
    }

    #[test]
    fn test_exit_err() {
        assert!(get_command("exi").is_err());
        assert!(get_command("exit4").is_err());
        assert!(get_command("exit 4").is_err());
    }

    #[test]
    fn test_add_ok() {
        let cmd = get_command("\t add \t PN \t   NAME  ").unwrap();
        assert_eq!(
            Command::Add(AddParams {
                pn: "PN".to_string(),
                name: "NAME".to_string()
            }),
            cmd
        );
    }

    #[test]
    fn test_tree_ok() {
        let cmd = get_command("\t tree \t PN \t  ").unwrap();
        assert_eq!(
            Command::Tree(TreeParams {
                pn: "PN".to_string(),
            }),
            cmd
        );
    }

    #[test]
    fn test_add_child() {
        let cmd = get_command("\t add-child \t PN1 \t   PN2\t  456 \t ").unwrap();
        assert_eq!(
            Command::AddChild(AddChildParams {
                parent_pn: "PN1".to_string(),
                child_pn: "PN2".to_string(),
                quantity: 456
            }),
            cmd
        );
    }
}
