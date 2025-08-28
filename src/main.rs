//! Ush is a new shell that provides unifying commands API's and easy to use.

#![deny(warnings)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(rustdoc::all)]
#![deny(missing_docs)]
#![allow(clippy::multiple_crate_versions)]

use std::{collections::HashMap, env, str::FromStr};

use anstyle::{AnsiColor, Style};

use crate::{
    builtins::echo::Echo,
    command::{CmdArg, CmdError, Command},
};

pub mod builtins;
pub mod command;

type ExecuteFn = fn(&[CmdArg]) -> Result<String, CmdError>;

fn main() {
    let error_style = Style::new().fg_color(Some(AnsiColor::Red.into())).bold();
    let log_style = Style::new().fg_color(Some(AnsiColor::Cyan.into())).bold();
    let pwd_style = Style::new().fg_color(Some(AnsiColor::BrightGreen.into()));
    let arrow_style = Style::new()
        .fg_color(Some(AnsiColor::Magenta.into()))
        .bold();

    let mut cmds: HashMap<&'static str, ExecuteFn> = HashMap::new();
    cmds.insert("echo", Echo::execute);

    println!("{log_style}Welcome to Ush! To list all builtins, type `ush -h`{log_style:#}");

    let mut buf = String::new();
    loop {
        if let Ok(pwd) = env::current_dir() {
            println!(
                "{pwd_style}{}{pwd_style:#}\n{arrow_style}>{arrow_style:#}",
                pwd.display()
            );
        }

        if let Ok(bytes_read) = std::io::stdin().read_line(&mut buf) {
            if bytes_read == 0 {
                println!("{log_style}Bye!{log_style:#}");
                break;
            }
            if buf.trim().is_empty() {
                continue;
            }
            let cmd_and_args: Vec<&str> = buf.split(' ').map(str::trim).collect();
            let mut args = vec![CmdArg::Help; cmd_and_args.len() - 1];
            for i in 1..cmd_and_args.len() {
                args[i - 1] = CmdArg::from_str(cmd_and_args[i])
                    .unwrap_or_else(|_| CmdArg::Literal(cmd_and_args[i].into()));
            }
            if let Some(f) = cmds.get(&cmd_and_args[0]) {
                match f(&args) {
                    Ok(res) => {
                        println!("{log_style}{res}{log_style:#}");
                    }
                    Err(e) => {
                        let err_text = match e {
                            CmdError::InvalidArg(x) => format!("Invalid arg: `{}`", x.as_ref()),
                        };
                        println!("{error_style}{err_text}{error_style:#}");
                    }
                }
            } else {
                println!(
                    "{error_style}Unknown cmd: `{}`{error_style:#}",
                    cmd_and_args[0]
                );
            }
        } else {
            println!("{error_style}Failed to read cmd{error_style:#}");
        }

        buf.clear();
    }
}
