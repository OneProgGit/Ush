//! [`uvim`]

use std::{fs::OpenOptions, io::Write};

use crate::command::{CmdArg, CmdError};

/// Uvim is a simple text appender
/// # Example
/// ```bash
/// > uvim Hi! example.txt
/// ```
/// # Errors
/// Returns an error if no args provided
/// # Panics
/// Panics when failed to read nor write to file
pub fn uvim(args: &[CmdArg]) -> Result<String, CmdError> {
    if !args.is_empty() && args[0] == CmdArg::Help {
        Ok("Uvim is a simple text appender\nPossible args: -h, [literal] [literal]".into())
    } else if args.len() != 2 {
        Err(CmdError::InvalidArgCount(args.len()))
    } else if let CmdArg::Literal(t) = &args[0] {
        if let CmdArg::Literal(f) = &args[1] {
            OpenOptions::new()
                .create(true)
                .truncate(false)
                .write(true)
                .open(f)
                .map_or(Err(CmdError::File), |mut buf| {
                    buf.write(t.as_bytes())
                        .map_or(Err(CmdError::File), |bytes| {
                            Ok(format!("Wrote {bytes} bytes"))
                        })
                })
        } else {
            Err(CmdError::InvalidArg(args[1].clone()))
        }
    } else {
        Err(CmdError::InvalidArg(args[0].clone()))
    }
}
