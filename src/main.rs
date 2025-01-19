use std::{
    env::current_dir,
    io::{stdin, stdout, Error, Write},
    path::PathBuf,
    process::{Child, Command, Stdio},
    str::SplitWhitespace,
};

use whoami::fallible;

fn start_loop() {
    let username: String = whoami::username();
    let hostname: String = match fallible::hostname() {
        Ok(x) => format!("@{}", x),
        Err(_) => String::from(""),
    };

    let combined: String = format!("[{0}{1}]", username, hostname);

    let current_folder: PathBuf = match current_dir() {
        Ok(x) => x,
        Err(_) => PathBuf::from("/"),
    };

    loop {
        print!("{0} {1} $ ", combined, current_folder.display());

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
                        // If there's a piped command
                        // Send the output to the next
                        Stdio::piped()
                    } else {
                        // If there's no more, then
                        // Send it to stdout
                        Stdio::inherit()
                    };

                    let output: anyhow::Result<Child, Error> = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
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
    start_loop();
}

pub mod commands;
pub mod dir;
pub mod parsers;
