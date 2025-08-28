//! [`parse_args`]

use std::str::FromStr;

use crate::command::CmdArg;

/// Parses cmd args to enum variants
#[must_use]
pub fn parse_args(cmd_and_args: &[&str]) -> Vec<CmdArg> {
    let mut args = vec![CmdArg::Help; cmd_and_args.len() - 1];
    for i in 1..cmd_and_args.len() {
        args[i - 1] = CmdArg::from_str(cmd_and_args[i])
            .unwrap_or_else(|_| CmdArg::Literal(cmd_and_args[i].into()));
    }
    args
}
