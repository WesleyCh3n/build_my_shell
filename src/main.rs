use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Stdio, Command, Child};

fn main() {
    loop_sh();
}

fn loop_sh() {
    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split("|").peekable();
        let mut prev_cmd = None;

        while let Some(cmd) = commands.next() {
            let mut parts = cmd.trim().split_whitespace();
            let cmd = parts.next().unwrap();
            let args = parts;

            match cmd {
                "cd" => {
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }
                    prev_cmd = None;
                }
                "exit" => return,
                cmd => {
                    let stdin = prev_cmd.map_or(
                        Stdio::inherit(),
                        |output: Child| Stdio::from(output.stdout.unwrap())
                    );

                    let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                    let output = Command::new(cmd)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => {
                            prev_cmd = Some(output);
                        }
                        Err(e) => {
                            prev_cmd = None;
                            eprintln!("{}", e);
                        },
                    }
                }
            }
        }
        if let Some(mut final_cmd) = prev_cmd {
            final_cmd.wait().unwrap();
        };
    }
}
