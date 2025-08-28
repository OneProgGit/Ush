//! [`Ush`]

use crate::command::{CmdArg, CmdError, Command};

/// Ush defines commands for shell itself
pub struct Ush;

impl Command for Ush {
    fn execute(args: &[crate::command::CmdArg]) -> Result<String, CmdError> {
        if args.is_empty() {
            return Err(CmdError::InvalidArg(CmdArg::None));
        }
        match &args[0] {
            CmdArg::Help => Ok("All builtins (try each command with -h arg):\n- echo".into()),

            _ => Err(CmdError::InvalidArg(args[0].clone())),
        }
    }
}
