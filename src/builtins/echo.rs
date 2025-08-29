//! [`echo`]

use crate::command::{CmdArg, CmdError};

/// Echo returns a literal itself
/// # Example
/// ```bash
/// > echo hi
/// hi
/// ```
/// # Errors
/// Returns an error when no args provided
pub fn echo(args: &[CmdArg]) -> Result<String, CmdError> {
    if args.is_empty() {
        return Err(CmdError::InvalidArgCount(0));
    }
    match &args[0] {
        CmdArg::Literal(x) => Ok(x.to_owned()),
        CmdArg::Help => Ok("Echo returns a literal itself.\nPossible args: [literal], -h".into()),
        CmdArg::None => Err(CmdError::InvalidArg(args[0].clone())),
    }
}
