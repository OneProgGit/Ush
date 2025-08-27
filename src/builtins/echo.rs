//! [`Echo`]

use crate::command::{CmdArg, CmdError, Command};

/// Echo returns argument itself.
/// # Example
/// ```bash
/// > echo hi
/// hi
/// ```
pub struct Echo;

impl Command for Echo {
    fn execute(args: &[CmdArg]) -> Result<String, CmdError> {
        if args.is_empty() {
            return Err(CmdError::InvalidArgs);
        }
        if let CmdArg::Literal(x) = &args[0] {
            return Ok(x.to_owned());
        }
        Err(CmdError::InvalidArgs)
    }
}
