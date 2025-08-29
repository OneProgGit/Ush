//! [`ush`]

use crate::command::{CmdArg, CmdError};

/// Ush defines commands for shell itself
/// # Example
/// ```bash
/// > ush -h
/// All builtins (try each command with -h arg):
/// - echo
/// - fetch
/// - whoami
/// - uvim
/// ```
/// # Errors
/// Returns an error when no args provided or not help arg
pub fn ush(args: &[CmdArg]) -> Result<String, CmdError> {
    if args.is_empty() {
        return Err(CmdError::InvalidArg(CmdArg::None));
    }
    match &args[0] {
        CmdArg::Help => Ok(
            "All builtins (try each command with -h arg):\n- echo\n- fetch\n- whoami\n- uvim"
                .into(),
        ),

        _ => Err(CmdError::InvalidArg(args[0].clone())),
    }
}
