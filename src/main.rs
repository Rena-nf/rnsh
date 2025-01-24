use std::{
    io::{stdin, stdout, Write},
    process::{Child, Command, Stdio},
    str::SplitWhitespace,
};

use anyhow::Context;
use clap::Parser;
use users::Users;

fn start_loop(user: Users) {
    let combined: String = format!("[{0}{1}]", user.username, user.hostname);

    loop {
        print!("{0} {1} $ ", combined, user.home_dir.display());

        let _ = stdout().flush();

        let mut input: String = String::new();

        stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split(" | ").peekable();
        let mut prev_parts: Option<Child> = None;

        while let Some(commands) = parts.next() {
            let mut part: SplitWhitespace<'_> = commands.split_whitespace();
            let command: &str = part.next().unwrap_or("Command Not Found");
            let args: SplitWhitespace<'_> = part;

            match command {
                "cd" => {
                    commands::cd(args);
                    prev_parts = None;
                }
                "exit" => return,

                command => {
                    let stdin: Stdio = prev_parts.map_or(Stdio::inherit(), |output: Child| {
                        Stdio::from(output.stdout.unwrap())
                    });

                    let stdout = if parts.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                    let output: anyhow::Result<Child, anyhow::Error> = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn()
                        .context("Failed to create command spawn");

                    match output.with_context(|| format!("Failed to run command :{}", command)) {
                        Ok(x) => prev_parts = Some(x),
                        Err(e) => {
                            prev_parts = None;
                            eprintln!("{}", e);
                        }
                    }
                }
            }
        }

        if let Some(mut final_command) = prev_parts {
            let _ = final_command.wait();
        }
    }
}

fn main() {
    #[allow(unused_variables)]
    let args: args::Args = args::Args::parse();
    let user: Users = Users::new();

    start_loop(user);
}

pub mod args;
pub mod commands;
pub mod users;
