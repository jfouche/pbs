use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::complete::{alphanumeric1, digit1, space0, space1},
    combinator::{eof, map_res},
    sequence::{preceded, tuple},
    IResult,
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

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct AddChildParams {
    pub parent_pn: String,
    pub child_pn: String,
    pub quantity: usize,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct TreeParams {
    pub pn: String,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct WhereUsedParams {
    pub pn: String,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct StockParams {
    pub pn: String,
}

/// Get the command of the input
pub fn get_command(input: &str) -> Result<Command, nom::Err<nom::error::Error<&str>>> {
    alt((
        cmd_add,
        cmd_list,
        cmd_add_child,
        cmd_tree,
        cmd_help,
        cmd_exit,
        cmd_where_used,
        cmd_stock,
    ))(input.trim())
    .map(|(_, cmd)| cmd)
}

fn pn(input: &str) -> IResult<&str, &str> {
    alphanumeric1(input)
}

fn name(input: &str) -> IResult<&str, &str> {
    alphanumeric1(input)
}

fn quantity(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

fn eol(input: &str) -> IResult<&str, ()> {
    space0(input)?;
    eof(input)?;
    Ok((input, ()))
}

/// `add <pn> <name>`
fn cmd_add(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("add")(input)?;
    let (input, pn) = preceded(space1, pn)(input)?;
    let (input, name) = preceded(space1, name)(input)?;
    let _ = eol(input)?;
    let params = AddParams {
        pn: pn.to_string(),
        name: name.to_string(),
    };
    Ok((input, Command::Add(params)))
}

/// `list`
fn cmd_list(input: &str) -> IResult<&str, Command> {
    let _ = tuple((tag("list"), eol))(input)?;
    Ok((input, Command::List))
}

/// `exit`
fn cmd_exit(input: &str) -> IResult<&str, Command> {
    let _ = tuple((tag("exit"), eol))(input)?;
    Ok((input, Command::Exit))
}

/// help
fn cmd_help(input: &str) -> IResult<&str, Command> {
    let _ = tuple((tag("help"), eol))(input)?;
    Ok((input, Command::Help))
}

/// `tree <pn>`
fn cmd_tree(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("tree")(input)?;
    let (input, pn) = preceded(space1, pn)(input)?;
    let _ = eol(input)?;
    let params = TreeParams { pn: pn.to_string() };
    Ok((input, Command::Tree(params)))
}

/// `where-used <pn>`
fn cmd_where_used(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("where-used")(input)?;
    let (input, pn) = preceded(space1, pn)(input)?;
    let _ = eol(input)?;
    let params = WhereUsedParams { pn: pn.to_string() };
    Ok((input, Command::WhereUsed(params)))
}

/// `add-child <parent-pn> <child-pn> <quantity>`
fn cmd_add_child(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("add-child")(input)?;
    let (input, parent_pn) = preceded(space1, pn)(input)?;
    let (input, child_pn) = preceded(space1, pn)(input)?;
    let (input, quantity) = quantity(input)?;
    let _ = eol(input)?;
    let params = AddChildParams {
        parent_pn: parent_pn.to_string(),
        child_pn: child_pn.to_string(),
        quantity,
    };
    Ok((input, Command::AddChild(params)))
}

/// `stock <pn>`
fn cmd_stock(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("stock")(input)?;
    let (input, pn) = preceded(space1, pn)(input)?;
    let _ = eol(input)?;
    let params = StockParams { pn: pn.to_string() };
    Ok((input, Command::Stock(params)))
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
        assert_eq!(cmd, res.unwrap());
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
        if let Ok(cmd) = res {
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
