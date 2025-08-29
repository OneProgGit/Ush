//! [`whoami`]

use crate::command::{CmdArg, CmdError};

/// Whoami gets a username
/// # Example
/// ```bash
/// > whoami
/// OneProg
/// ```
/// # Errors
/// Returns an error if not help arg provided if there are args
pub fn whoami(args: &[CmdArg]) -> Result<String, CmdError> {
    if args.is_empty() {
        Ok(whoami::username())
    } else if args[0] == CmdArg::Help {
        Ok("Whoami gets username.\nPossible args: [none], -h.".into())
    } else {
        Err(CmdError::InvalidArg(args[0].clone()))
    }
}
