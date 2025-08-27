//! Command defines unifying interfaces for all commands

/// Defines common command's methods
pub trait Command {
    /// Executes a command with given arguments
    fn execute(args: &[CmdArg]) -> Result<String, CmdError>;
}

/// Defines common command's args
#[derive(PartialEq, Eq, Clone)]
pub enum CmdArg {
    /// Literal, like `Hello`
    Literal(String),
    /// Help command, usually used for listing all subcommands and args
    Help,
}

/// Defines command's errors
pub enum CmdError {
    /// Invalid arguments. For example, echo takes one literal arg, but user called it without args.
    InvalidArgs,
}
