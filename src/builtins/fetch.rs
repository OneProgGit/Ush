//! [`fetch`]

use std::env;

use crate::command::{CmdArg, CmdError};

/// Fetch gets system info
/// # Example
/// ```bash
/// > fetch
/// OneProg OS 1.0.0
/// ```
/// # Errors
/// Returns an error if not help arg provided if there are args
pub fn fetch(args: &[CmdArg]) -> Result<String, CmdError> {
    if args.is_empty() {
        Ok(env::consts::OS.into())
    } else if args[0] == CmdArg::Help {
        Ok("Fetch gets system info.\nPossible args: [none], -h.".into())
    } else {
        Err(CmdError::InvalidArg(args[0].clone()))
    }
}
