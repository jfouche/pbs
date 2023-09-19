use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::complete::{alphanumeric1, digit1, space1},
    combinator::{eof, map_res},
    // error::ParseError,
    IResult,
};

#[derive(PartialEq, Eq, Debug)]
pub enum Command {
    Add(AddParams),
    AddChild(AddChildParams),
    List,
    Tree(TreeParams),
    Help,
    Exit,
}

#[derive(PartialEq, Eq, Debug)]
pub struct AddParams {
    pub pn: String,
    pub name: String,
}

#[derive(PartialEq, Eq, Debug)]
pub struct AddChildParams {
    pub parent_pn: String,
    pub child_pn: String,
    pub quantity: usize,
}

#[derive(PartialEq, Eq, Debug)]
pub struct TreeParams {
    pub pn: String,
}

// #[derive(PartialEq, Eq, Debug)]
// pub enum PbsParserError {
//     MissingPartNumber,
//     MissingName,
//     TooManyParams,
// }

// impl ParseError<&str> for PbsParserError {
//     fn from_error_kind(input: &str, kind: nom::error::ErrorKind) -> Self {}
//     fn append(input: &str, kind: nom::error::ErrorKind, other: Self) -> Self {}
// }

// //type ErrorTranslator = Fn(nom::error::Error<&str>) -> ParserError;

// fn missing_pn(_: nom::error::Error<&str>) -> impl Fn(nom::error::Error<&str>) -> ParserError {
//     |_: nom::error::Error<&str>| ParserError::MissingPartNumber
// }

// type CResult<'a> = IResult<&'a str, Command, ParserError>;

/// Get the command of the input
pub fn get_command(input: &str) -> IResult<&str, Command> {
    alt((
        cmd_add,
        cmd_list,
        cmd_add_child,
        cmd_tree,
        cmd_help,
        cmd_exit,
    ))(input.trim())
}

fn pn(input: &str) -> IResult<&str, &str> {
    alphanumeric1(input)
}

fn quantity(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

/// `add <pn> <name>`
fn cmd_add(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("add")(input)?;
    let (input, _) = space1(input)?;
    let (input, pn) = pn(input)?;
    let (input, _) = space1(input)?;
    let (input, name) = alphanumeric1(input)?;
    let (input, _) = eof(input)?;
    let params = AddParams {
        pn: pn.to_string(),
        name: name.to_string(),
    };
    Ok((input, Command::Add(params)))
}

/// `list`
fn cmd_list(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("list")(input)?;
    let (input, _) = eof(input)?;
    Ok((input, Command::List))
}

/// `exit`
fn cmd_exit(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("exit")(input)?;
    let (input, _) = eof(input)?;
    Ok((input, Command::Exit))
}

/// help
fn cmd_help(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("help")(input)?;
    let (input, _) = eof(input)?;
    Ok((input, Command::Help))
}

/// `tree <pn>`
fn cmd_tree(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("tree")(input)?;
    let (input, _) = space1(input)?;
    let (input, pn) = pn(input)?;
    let (input, _) = eof(input)?;
    let params = TreeParams { pn: pn.to_string() };
    Ok((input, Command::Tree(params)))
}

/// `add-child <parent-pn> <child-pn> <quantity>`
fn cmd_add_child(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("add-child")(input)?;
    let (input, _) = space1(input)?;
    let (input, parent_pn) = pn(input)?;
    let (input, _) = space1(input)?;
    let (input, child_pn) = pn(input)?;
    let (input, _) = space1(input)?;
    let (input, quantity) = quantity(input)?;
    let (input, _) = eof(input)?;
    let params = AddChildParams {
        parent_pn: parent_pn.to_string(),
        child_pn: child_pn.to_string(),
        quantity,
    };
    Ok((input, Command::AddChild(params)))
}

/// =================================================================
/// Test
/// =================================================================
#[cfg(test)]
mod tests {
    use super::*;

    fn assert_is(cmd: Command, input: &str) {
        let res = get_command(input);
        assert!(res.is_ok());
        assert_eq!(cmd, res.unwrap().1);
    }

    #[test]
    fn test_exit_ok() {
        assert_is(Command::Exit, "exit");
        assert_is(Command::Exit, "  exit");
        assert_is(Command::Exit, "  exit  ");
    }

    #[test]
    fn test_exit_err() {
        let res = get_command("exi");
        assert!(res.is_err());
        let res = get_command("exitt");
        assert!(res.is_err());
    }

    #[test]
    fn test_add_ok() {
        let res = get_command("\t add \t PN \t   NAME  ");
        assert!(res.is_ok(), "{res:?}");
        if let Ok((_, cmd)) = res {
            match cmd {
                Command::Add(params) => {
                    assert_eq!("PN".to_string(), params.pn);
                    assert_eq!("NAME".to_string(), params.name);
                }
                _ => panic!("Bad command : {cmd:?}"),
            }
        }
    }
}
