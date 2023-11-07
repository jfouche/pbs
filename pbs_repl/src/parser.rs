use std::fmt::Debug;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_while1},
    character::complete::{char, digit1, multispace0, space0, space1},
    combinator::{eof, map_res},
    error::ParseError,
    sequence::{delimited, pair, preceded, tuple},
    IResult, Parser,
};

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Command {
    ItemMake(ItemMakeParams),
    ItemBuy(ItemBuyParams),
    ItemRelease(ItemReleaseParams),
    ItemUpgrade(ItemUpgradeParams),
    ChildAdd(ChildAddParams),
    ChildDel(ChildDelParams),
    List,
    Tree(TreeParams),
    WhereUsed(WhereUsedParams),
    Report(ReportParams),
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

// Generic for a single <id> params
pub struct SingleIdParams {
    pub id: i64,
}

impl From<i64> for SingleIdParams {
    fn from(value: i64) -> Self {
        SingleIdParams { id: value }
    }
}

/// Params for the `make` command
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct ItemMakeParams {
    pub name: String,
}

impl From<&str> for ItemMakeParams {
    fn from(value: &str) -> Self {
        ItemMakeParams {
            name: value.to_string(),
        }
    }
}

impl ParamsCmd for ItemMakeParams {
    fn cmd(self) -> Command {
        Command::ItemMake(self)
    }
}
/// Params for the `add` command
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct ItemBuyParams {
    pub pn: String,
    pub name: String,
}

impl From<(&str, &str)> for ItemBuyParams {
    fn from(value: (&str, &str)) -> Self {
        ItemBuyParams {
            pn: value.0.to_string(),
            name: value.1.to_string(),
        }
    }
}

impl ParamsCmd for ItemBuyParams {
    fn cmd(self) -> Command {
        Command::ItemBuy(self)
    }
}

/// Params for the `item release` command
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct ItemReleaseParams {
    pub id: i64,
}

impl From<i64> for ItemReleaseParams {
    fn from(value: i64) -> Self {
        ItemReleaseParams { id: value }
    }
}

impl ParamsCmd for ItemReleaseParams {
    fn cmd(self) -> Command {
        Command::ItemRelease(self)
    }
}

/// Params for the `item upgrade` command
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct ItemUpgradeParams {
    pub id: i64,
}

impl From<i64> for ItemUpgradeParams {
    fn from(value: i64) -> Self {
        ItemUpgradeParams { id: value }
    }
}

impl ParamsCmd for ItemUpgradeParams {
    fn cmd(self) -> Command {
        Command::ItemUpgrade(self)
    }
}

/// Params for the `child add` command
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct ChildAddParams {
    pub parent_id: i64,
    pub child_id: i64,
    pub quantity: usize,
}

impl From<(i64, i64, usize)> for ChildAddParams {
    fn from(value: (i64, i64, usize)) -> Self {
        ChildAddParams {
            parent_id: value.0,
            child_id: value.1,
            quantity: value.2,
        }
    }
}

impl ParamsCmd for ChildAddParams {
    fn cmd(self) -> Command {
        Command::ChildAdd(self)
    }
}

/// Params for the `child del` command
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct ChildDelParams {
    pub parent_id: i64,
    pub child_id: i64,
}

impl From<(i64, i64)> for ChildDelParams {
    fn from(value: (i64, i64)) -> Self {
        ChildDelParams {
            parent_id: value.0,
            child_id: value.1,
        }
    }
}

impl ParamsCmd for ChildDelParams {
    fn cmd(self) -> Command {
        Command::ChildDel(self)
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

/// Params for the `report` command
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct ReportParams {
    pub id: i64,
}

impl From<i64> for ReportParams {
    fn from(value: i64) -> Self {
        ReportParams { id: value }
    }
}

impl ParamsCmd for ReportParams {
    fn cmd(self) -> Command {
        Command::Report(self)
    }
}

// ====================================================================
// parser helper functions
// ====================================================================

/// parser for a cmd followed with params.
fn cmd<'a, O, E, F>(cmd: &'a str, parser: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    E: ParseError<&'a str>,
    F: Parser<&'a str, O, E>,
{
    preceded(pair(space0, tag(cmd)), parser)
}

/// parser for a whitespace separated param
fn param<'a, O, E, F>(mut parser: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    E: ParseError<&'a str>,
    F: Parser<&'a str, O, E>,
{
    move |input: &'a str| {
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

fn cmd_item(input: &str) -> IResult<&str, Command> {
    let params = alt((
        cmd_item_make,
        cmd_item_buy,
        cmd_item_release,
        cmd_item_upgrade,
    ));
    cmd("item", params)(input)
}

fn cmd_item_make(input: &str) -> IResult<&str, Command> {
    let params = param(pn);
    cmd("make", params)(input).cmd_n::<ItemMakeParams>()
}

fn cmd_item_buy(input: &str) -> IResult<&str, Command> {
    let params = pair(param(pn), param(name));
    cmd("buy", params)(input).cmd_n::<ItemBuyParams>()
}

fn cmd_item_release(input: &str) -> IResult<&str, Command> {
    let params = param(id);
    cmd("release", params)(input).cmd_n::<ItemReleaseParams>()
}

fn cmd_item_upgrade(input: &str) -> IResult<&str, Command> {
    let params = param(id);
    cmd("upgrade", params)(input).cmd_n::<ItemUpgradeParams>()
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

/// `child ...`
fn cmd_child(input: &str) -> IResult<&str, Command> {
    let params = alt((cmd_child_add, cmd_child_del));
    cmd("child", params)(input)
}

/// `child add <parent-id> <child-id> <quantity>`
fn cmd_child_add(input: &str) -> IResult<&str, Command> {
    let params = tuple((param(id), param(id), param(quantity)));
    cmd("add", params)(input).cmd_n::<ChildAddParams>()
}
/// `child del <parent-id> <child-id>`
fn cmd_child_del(input: &str) -> IResult<&str, Command> {
    let params = tuple((param(id), param(id)));
    cmd("del", params)(input).cmd_n::<ChildDelParams>()
}

/// `report <pn>`
fn cmd_report(input: &str) -> IResult<&str, Command> {
    let params = param(id);
    preceded(tag("report"), params)(input).cmd_n::<ReportParams>()
}

/// Get the command of the input
pub fn get_command(input: &str) -> Result<Command, nom::Err<nom::error::Error<&str>>> {
    delimited(
        space0,
        alt((
            cmd_item,
            cmd_list,
            cmd_child,
            cmd_tree,
            cmd_help,
            cmd_exit,
            cmd_where_used,
            cmd_report,
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
    fn test_item_make() {
        let cmd = get_command("item make NAME").unwrap();
        assert_eq!(
            Command::ItemMake(ItemMakeParams {
                name: "NAME".to_string(),
            }),
            cmd
        );
    }

    #[test]
    fn test_item_buy() {
        let cmd = dbg!(get_command("item  buy  PN  NAME  ")).unwrap();
        assert_eq!(
            Command::ItemBuy(ItemBuyParams {
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
    fn test_child_add() {
        let cmd = get_command("\t child add \t 1 \t   17\t  456 \t ").unwrap();
        assert_eq!(
            Command::ChildAdd(ChildAddParams {
                parent_id: 1,
                child_id: 17,
                quantity: 456
            }),
            cmd
        );
    }

    #[test]
    fn test_child_del() {
        let cmd = get_command("\t child del \t 1 \t   17\t ").unwrap();
        assert_eq!(
            Command::ChildDel(ChildDelParams {
                parent_id: 1,
                child_id: 17,
            }),
            cmd
        );
    }
}
