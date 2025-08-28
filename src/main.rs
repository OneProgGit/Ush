//! Ush is a new shell that provides unifying commands API's and easy to use.

#![deny(warnings)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(rustdoc::all)]
#![deny(missing_docs)]
#![allow(clippy::multiple_crate_versions)]

use std::{
    collections::HashMap,
    env,
    io::{Write, stdin, stdout},
};

use anstyle::{AnsiColor, Style};

use crate::{
    args_parser::parse_args,
    builtins::{echo::Echo, ush::Ush},
    cmds_executor::execute_cmd,
    command::{CmdArg, CmdError, Command},
};

pub mod args_parser;
pub mod builtins;
pub mod cmds_executor;
pub mod command;

/// Defines cmd execture function
pub type ExecuteFn = fn(&[CmdArg]) -> Result<String, CmdError>;

fn main() {
    let error_style = Style::new().fg_color(Some(AnsiColor::Red.into())).bold();
    let log_style = Style::new().fg_color(Some(AnsiColor::Cyan.into())).bold();
    let pwd_style = Style::new().fg_color(Some(AnsiColor::BrightGreen.into()));
    let arrow_style = Style::new()
        .fg_color(Some(AnsiColor::Magenta.into()))
        .bold();

    let mut cmds: HashMap<&str, ExecuteFn> = HashMap::new();
    cmds.insert("echo", Echo::execute);
    cmds.insert("ush", Ush::execute);

    println!("{log_style}Welcome to Ush! To list all builtins, type `ush -h`{log_style:#}");

    let mut buf = String::new();
    loop {
        if let Ok(pwd) = env::current_dir() {
            print!(
                "{pwd_style}{}{pwd_style:#}\n{arrow_style}>{arrow_style:#} ",
                pwd.display()
            );
            stdout().flush().unwrap_or_else(|e| {
                println!("{error_style}Failed to flush stdout: `{e}`{error_style}");
            });
        }

        match stdin().read_line(&mut buf) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    println!("{log_style}Bye!{log_style:#}");
                    break;
                }
                if buf.trim().is_empty() {
                    continue;
                }
                let cmd_and_args: Vec<&str> = buf.split(' ').map(str::trim).collect();
                let args = parse_args(&cmd_and_args);

                match execute_cmd(cmd_and_args[0], &args, &cmds) {
                    Ok(res) => println!("{log_style}{res}{log_style:#}"),
                    Err(err) => println!("{error_style}{err}{error_style:#}"),
                }
            }
            Err(e) => {
                println!("{error_style}Failed to read stdin: `{e}`{error_style:#}");
            }
        }

        buf.clear();
    }
}
