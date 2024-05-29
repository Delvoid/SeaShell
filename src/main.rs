#[allow(unused_imports)]
use std::io::{self, Write};
use std::{
    env,
    path::PathBuf,
    process::{Command as OsCommand, Stdio},
};

use dirs::home_dir;

enum BuiltinCommand {
    Exit,
    Echo,
    Type,
    Pwd,
    Cd,
}

impl BuiltinCommand {
    fn from_str(s: &str) -> Option<BuiltinCommand> {
        match s {
            "exit" => Some(BuiltinCommand::Exit),
            "echo" => Some(BuiltinCommand::Echo),
            "type" => Some(BuiltinCommand::Type),
            "pwd" => Some(BuiltinCommand::Pwd),
            "cd" => Some(BuiltinCommand::Cd),
            _ => None,
        }
    }
}

enum Command {
    Builtin(BuiltinCommand, Vec<String>),
    External(String, Vec<String>),
}

impl Command {
    fn from_args(args: &[&str]) -> Self {
        if let Some(builtin) = BuiltinCommand::from_str(args[0]) {
            Command::Builtin(builtin, args[1..].iter().map(|s| s.to_string()).collect())
        } else {
            Command::External(
                args[0].to_string(),
                args[1..].iter().map(|s| s.to_string()).collect(),
            )
        }
    }

    fn execute(&self) {
        match self {
            Command::Builtin(builtin, args) => match builtin {
                BuiltinCommand::Exit => {
                    let code = args.first().and_then(|code| code.parse().ok()).unwrap_or(0);
                    std::process::exit(code);
                }
                BuiltinCommand::Echo => {
                    println!("{}", args.join(" "));
                }
                BuiltinCommand::Type => {
                    if let Some(command) = args.first() {
                        if BuiltinCommand::from_str(command).is_some() {
                            println!("{} is a shell builtin", command);
                        } else if let Some(executable) = find_executionable(command) {
                            println!("{} is {}", command, executable);
                        } else {
                            println!("{}: not found", command);
                        }
                    } else {
                        eprintln!("type: missing argument");
                    }
                }
                BuiltinCommand::Pwd => match std::env::current_dir() {
                    Ok(pwd) => println!("{}", pwd.display()),
                    Err(err) => eprintln!("pwd: failed to get current directory: {}", err),
                },
                BuiltinCommand::Cd => {
                    if let Some(path) = args.first() {
                        let new_dir = if path == "~" {
                            // Home directory
                            home_dir().unwrap_or_else(|| PathBuf::from("/"))
                        } else if path.starts_with('/') {
                            // Absolute path
                            PathBuf::from(path)
                        } else {
                            // Relative path
                            let mut current_dir =
                                env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
                            for component in path.split('/') {
                                match component {
                                    "." => {}
                                    ".." => {
                                        if let Some(parent) = current_dir.parent() {
                                            current_dir = parent.to_path_buf();
                                        }
                                    }
                                    other => {
                                        current_dir.push(other);
                                    }
                                }
                            }
                            current_dir
                        };

                        match env::set_current_dir(new_dir) {
                            Ok(()) => {}
                            Err(_) => eprintln!("{}: No such file or directory", path),
                        }
                    } else {
                        eprintln!("cd: missing argument");
                    }
                }
            },
            Command::External(command, args) => {
                if let Some(path) = find_executionable(command) {
                    match OsCommand::new(path)
                        .args(args)
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .output()
                    {
                        Ok(output) => {
                            if !output.status.success() {
                                match output.status.code() {
                                    Some(code) => eprintln!(
                                        "{}: command exited with status code {}",
                                        command, code
                                    ),
                                    None => eprintln!("{}: command terminated by signal", command),
                                }
                            }
                        }
                        Err(err) => {
                            eprintln!("{}: failed to execute command: {}", command, err);
                        }
                    }
                } else {
                    eprintln!("{}: not found", command);
                }
            }
        }
    }
}

fn find_executionable(command: &str) -> Option<String> {
    if command.starts_with('/') {
        if std::fs::metadata(command).is_ok() {
            return Some(command.to_string());
        }
    } else {
        let paths = std::env::var("PATH").unwrap_or_else(|_| "/bin:/usr/bin".to_string());
        for path in paths.split(':') {
            let full_path = format!("{}/{}", path, command);
            if std::fs::metadata(&full_path).is_ok() {
                return Some(full_path);
            }
        }
    }
    None
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let args: Vec<&str> = input.split_whitespace().collect();

        if !args.is_empty() {
            let command = Command::from_args(&args);
            command.execute();

            if matches!(command, Command::Builtin(BuiltinCommand::Exit, _)) {
                break;
            }
        }
    }
}
