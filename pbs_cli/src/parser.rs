use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::complete::{alphanumeric1, space1},
    combinator::eof,
    IResult,
};

#[derive(PartialEq, Eq, Debug)]
pub enum Command {
    Add { pn: String },
    List,
    Exit,
}

/// add <pn>
fn cmd_add(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("add")(input)?;
    let (input, _) = space1(input)?;
    let (input, pn) = alphanumeric1(input)?;
    let (input, _) = eof(input)?;
    Ok((input, Command::Add { pn: pn.to_string() }))
}

/// list
fn cmd_list(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("list")(input)?;
    let (input, _) = eof(input)?;
    Ok((input, Command::List))
}

/// exit
fn cmd_exit(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("exit")(input)?;
    let (input, _) = eof(input)?;
    Ok((input, Command::Exit))
}

pub fn get_command(input: &str) -> IResult<&str, Command> {
    alt((cmd_add, cmd_list, cmd_exit))(input.trim())
}

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
        assert_is(
            Command::Add {
                pn: "toto".to_string(),
            },
            " add \t toto  ",
        );
    }
}
