//! Command defines unifying interfaces for all commands

use strum::{AsRefStr, EnumString};

/// Defines common command's args
#[derive(PartialEq, Eq, Clone, AsRefStr, EnumString)]
pub enum CmdArg {
    /// Empty argument
    #[strum(serialize = "none")]
    None,
    /// Literal, like `Hello`
    #[strum(serialize = "literal")]
    Literal(String),
    /// Help command, usually used for listing all subcommands and args
    #[strum(serialize = "-h")]
    Help,
}

/// Defines command's errors
pub enum CmdError {
    /// Invalid argument. For example, `echo` takes one literal arg, but user called it with -s arg.
    InvalidArg(CmdArg),
    /// Invalid arg count. For example, `echo` takes one literal arg, but no args were provided by user.
    InvalidArgCount(usize),
    /// Error, when failed to read or write to file
    File,
}
