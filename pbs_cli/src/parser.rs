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
    Create(CreateParams),
    Import(ImportParams),
    AddChild(AddChildParams),
    List,
    Tree(TreeParams),
    WhereUsed(WhereUsedParams),
    Stock(StockParams),
    Help,
    Exit,
}

/// trait to retrieve a command from it's params
trait ParamsCmd {
    fn cmd(self) -> Command;
}

/// trait to convert a parser result which returns params to a
/// `IResult<I, Command>`
trait CommandWithParamsResult<I, O> {
    fn cmd_n<P: ParamsCmd + From<O>>(self) -> IResult<I, Command>;
}

impl<I, O> CommandWithParamsResult<I, O> for IResult<I, O> {
    fn cmd_n<P>(self) -> IResult<I, Command>
    where
        P: ParamsCmd + From<O>,
    {
        self.map(|(i, o)| (i, P::from(o).cmd()))
    }
}

/// trait to convert a parser result which returns no param to a
/// `IResult<I, Command>`
trait CommandWithoutParamsResult<I, O> {
    fn cmd_0(self, command: Command) -> IResult<I, Command>;
}

impl<I, O> CommandWithoutParamsResult<I, O> for IResult<I, O> {
    fn cmd_0(self, command: Command) -> IResult<I, Command> {
        self.map(|(i, _)| (i, command))
    }
}

/// Params for the `create` command
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct CreateParams {
    pub name: String,
}

impl From<&str> for CreateParams {
    fn from(value: &str) -> Self {
        CreateParams {
            name: value.to_string(),
        }
    }
}

impl ParamsCmd for CreateParams {
    fn cmd(self) -> Command {
        Command::Create(self)
    }
}
/// Params for the `add` command
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct ImportParams {
    pub pn: String,
    pub name: String,
}

impl From<(&str, &str)> for ImportParams {
    fn from(value: (&str, &str)) -> Self {
        ImportParams {
            pn: value.0.to_string(),
            name: value.1.to_string(),
        }
    }
}

impl ParamsCmd for ImportParams {
    fn cmd(self) -> Command {
        Command::Import(self)
    }
}

/// Params for the `add-child` command
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct AddChildParams {
    pub parent_id: i64,
    pub child_id: i64,
    pub quantity: usize,
}

impl From<(i64, i64, usize)> for AddChildParams {
    fn from(value: (i64, i64, usize)) -> Self {
        AddChildParams {
            parent_id: value.0,
            child_id: value.1,
            quantity: value.2,
        }
    }
}

impl ParamsCmd for AddChildParams {
    fn cmd(self) -> Command {
        Command::AddChild(self)
    }
}

/// Params for the `tree` command
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct TreeParams {
    pub id: i64,
}

impl From<i64> for TreeParams {
    fn from(value: i64) -> Self {
        TreeParams { id: value }
    }
}

impl ParamsCmd for TreeParams {
    fn cmd(self) -> Command {
        Command::Tree(self)
    }
}

/// Params for the `where-used` command
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct WhereUsedParams {
    pub id: i64,
}

impl From<i64> for WhereUsedParams {
    fn from(value: i64) -> Self {
        WhereUsedParams { id: value }
    }
}

impl ParamsCmd for WhereUsedParams {
    fn cmd(self) -> Command {
        Command::WhereUsed(self)
    }
}

/// Params for the `stock` command
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct StockParams {
    pub id: i64,
}

impl From<i64> for StockParams {
    fn from(value: i64) -> Self {
        StockParams { id: value }
    }
}

impl ParamsCmd for StockParams {
    fn cmd(self) -> Command {
        Command::Stock(self)
    }
}

// ====================================================================
// parser helper functions
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

/// Parser for an [Item] id
fn id(input: &str) -> IResult<&str, i64> {
    map_res(digit1, |s: &str| s.parse::<i64>())(input)
}

/// End of line parser
fn eol(input: &str) -> IResult<&str, ()> {
    pair(multispace0, eof)(input).map(|(i, (_, _))| (i, ()))
}

// ====================================================================
// command parsers
// ====================================================================

/// `create <name>`
fn cmd_create(input: &str) -> IResult<&str, Command> {
    let params = param(pn);
    cmd("create", params)(input).cmd_n::<CreateParams>()
}

/// `add <pn> <name>`
fn import_add(input: &str) -> IResult<&str, Command> {
    let params = pair(param(pn), param(name));
    cmd("import", params)(input).cmd_n::<ImportParams>()
}

/// `list`
fn cmd_list(input: &str) -> IResult<&str, Command> {
    tag("list")(input).cmd_0(Command::List)
}

/// `exit`
fn cmd_exit(input: &str) -> IResult<&str, Command> {
    tag("exit")(input).cmd_0(Command::Exit)
}

/// `help`
fn cmd_help(input: &str) -> IResult<&str, Command> {
    tag("help")(input).cmd_0(Command::Help)
}

/// `tree <id>`
fn cmd_tree(input: &str) -> IResult<&str, Command> {
    let params = param(id);
    cmd("tree", params)(input).cmd_n::<TreeParams>()
}

/// `where-used <id>`
fn cmd_where_used(input: &str) -> IResult<&str, Command> {
    let params = param(id);
    cmd("where-used", params)(input).cmd_n::<WhereUsedParams>()
}

/// `add-child <parent-id> <child-id> <quantity>`
fn cmd_add_child(input: &str) -> IResult<&str, Command> {
    let params = tuple((param(id), param(id), param(quantity)));
    cmd("add-child", params)(input).cmd_n::<AddChildParams>()
}

/// `stock <pn>`
fn cmd_stock(input: &str) -> IResult<&str, Command> {
    let params = param(id);
    preceded(tag("stock"), params)(input).cmd_n::<StockParams>()
}

/// Get the command of the input
pub fn get_command(input: &str) -> Result<Command, nom::Err<nom::error::Error<&str>>> {
    delimited(
        space0,
        alt((
            cmd_create,
            import_add,
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
            Command::Import(ImportParams {
                pn: "PN".to_string(),
                name: "NAME".to_string()
            }),
            cmd
        );
    }

    #[test]
    fn test_tree_ok() {
        let cmd = get_command("\t tree \t 481 \t  ").unwrap();
        assert_eq!(Command::Tree(TreeParams { id: 481 }), cmd);
    }

    #[test]
    fn test_add_child() {
        let cmd = get_command("\t add-child \t 1 \t   17\t  456 \t ").unwrap();
        assert_eq!(
            Command::AddChild(AddChildParams {
                parent_id: 1,
                child_id: 17,
                quantity: 456
            }),
            cmd
        );
    }

    #[test]
    fn test_create() {
        let cmd = get_command("\t create \t   \t NAME ").unwrap();
        assert_eq!(
            Command::Create(CreateParams {
                name: "NAME".to_string(),
            }),
            cmd
        );
    }
}
