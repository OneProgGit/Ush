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
            return Err(CmdError::InvalidArg(CmdArg::None));
        }
        match &args[0] {
            CmdArg::Literal(x) => Ok(x.to_owned()),
            CmdArg::Help => {
                Ok("Echo returns a literal itself.\nPossible args: [literal], -h".into())
            }
            CmdArg::None => Err(CmdError::InvalidArg(args[0].clone())),
        }
    }
}
