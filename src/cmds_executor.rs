//! [`execute_cmd`]

use std::collections::HashMap;

use crate::{
    ExecuteFn,
    command::{CmdArg, CmdError},
};

/// Executes given command with args.
/// # Errors
/// Returns an error, if execution fails: invalid command or error from cmd
pub fn execute_cmd<S: std::hash::BuildHasher>(
    cmd: &str,
    args: &[CmdArg],
    cmds: &HashMap<&str, ExecuteFn, S>,
) -> Result<String, String> {
    cmds.get(cmd).map_or_else(
        || Err(format!("Unknown cmd: `{cmd}`")),
        |f| match f(args) {
            Ok(res) => Ok(res),
            Err(e) => {
                let err_text = match e {
                    CmdError::InvalidArg(x) => format!("Invalid arg: `{}`", x.as_ref()),
                };
                Err(err_text)
            }
        },
    )
}
