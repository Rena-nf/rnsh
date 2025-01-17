use std::{
    io::{stdin, stdout, Write},
    process::{Child, Command, Stdio},
};

fn main() {
    loop {
        print!("> ");
        let _ = stdout().flush();

        let mut input: String = String::new();

        stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split(" | ").peekable();
        let mut prev_parts: Option<Child> = None;

        while let Some(commands) = parts.next() {
            let mut part = commands.split_whitespace();
            let command = part.next().unwrap_or("Command Not Found");
            let args = part;

            match command {
                "cd" => {
                    commands::cd(args);
                    prev_parts = None;
                }
                "exit" => return,
                command => {
                    let stdin = prev_parts.map_or(Stdio::inherit(), |output: Child| {
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

                    let output = Command::new(command)
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

pub mod commands;
pub mod lexers;
pub mod shared;
