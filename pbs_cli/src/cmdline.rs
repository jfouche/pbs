use std::env::Args;

#[derive(Debug)]
pub enum Commands {
    Usage,
    Add(AddParams),
}

#[derive(Debug)]
pub enum CmdLineError {
    MissingCommand,
    UnknownCommand(String),
    MissingParams(Commands),
}

#[derive(Default, Debug)]
pub struct AddParams {
    pub pn: String,
    pub name: String,
}

/// Return the command
///
/// The args should remain untouched (the 1st elem shound be the executable name)
pub fn parse_cmd_line(mut args: Args) -> Result<Commands, CmdLineError> {
    args.next();
    if let Some(cmd) = args.next() {
        match cmd.as_str() {
            "-h" | "--help" => Ok(Commands::Usage),
            "add" => parse_add(args),
            cmd => Err(CmdLineError::UnknownCommand(cmd.to_string())),
        }
    } else {
        Err(CmdLineError::MissingCommand)
    }
}

/// Parse the [add] command
///
/// `add <part_number> <name>`
fn parse_add(mut args: Args) -> Result<Commands, CmdLineError> {
    let pn = args
        .next()
        .ok_or(CmdLineError::MissingParams(Commands::Add(
            AddParams::default(),
        )))?;
    let name = args
        .next()
        .ok_or(CmdLineError::MissingParams(Commands::Add(
            AddParams::default(),
        )))?;
    Ok(Commands::Add(AddParams { pn, name }))
}
