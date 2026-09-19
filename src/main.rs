use std::collections::HashMap;
use std::io;
use std::io::Write;

enum ParseError<'a> {
    UnknownCommand { command: &'a str },
    ArgumentCountMismatch { expected: usize, actual: usize },
    EmptyCommand,
}

enum Command<'a> {
    SET { key: &'a str, value: &'a str },
    GET { key: &'a str },
    DEL { key: &'a str },
    EXISTS { key: &'a str },
}

fn get_command<'a>(command: &Vec<&'a str>) -> Result<Command<'a>, ParseError<'a>> {
    match command.first() {
        Some(&"SET") => {
            if command.len() == 3 {
                Ok(Command::SET {
                    key: command[1],
                    value: command[2],
                })
            } else {
                Err(ParseError::ArgumentCountMismatch {
                    expected: 2,
                    actual: command.len() - 1,
                })
            }
        }
        Some(&"GET") => {
            if command.len() == 2 {
                Ok(Command::GET { key: command[1] })
            } else {
                Err(ParseError::ArgumentCountMismatch {
                    expected: 1,
                    actual: command.len() - 1,
                })
            }
        }
        Some(&"DEL") => {
            if command.len() == 2 {
                Ok(Command::DEL { key: command[1] })
            } else {
                Err(ParseError::ArgumentCountMismatch {
                    expected: 1,
                    actual: command.len() - 1,
                })
            }
        }
        Some(&"EXISTS") => {
            if command.len() == 2 {
                Ok(Command::EXISTS { key: command[1] })
            } else {
                Err(ParseError::ArgumentCountMismatch {
                    expected: 1,
                    actual: command.len() - 1,
                })
            }
        }
        Some(&v) => Err(ParseError::UnknownCommand { command: v }),
        None => Err(ParseError::EmptyCommand),
    }
}

struct Store {
    hashmap: HashMap<String, String>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            hashmap: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.hashmap.insert(key.to_string(), value.to_string());
    }
    pub fn get(&self, key: &str) -> Option<&str> {
        self.hashmap.get(key).map(|s| s.as_str())
    }
    pub fn del(&mut self, key: &str) -> bool {
        self.hashmap.remove(key).is_some()
    }
    pub fn exists(&self, key: &str) -> bool {
        self.hashmap.contains_key(key)
    }
}

fn handle_command_error(command: &str) {
    println!("ERROR: Unknown command: '{}'", command);
}

fn handle_arg_error(length: usize, right_length: usize) {
    if length == 1 {
        println!("ERROR: Expected {} argument, got {}", length, right_length);
    } else {
        println!("ERROR: Expected {} arguments, got {}", length, right_length);
    }
}

fn handle_get(value: Option<&str>) {
    match value {
        Some(v) => println!("{}", v),
        _ => (),
    }
}

fn handle_del(deleted: bool, key: &str) {
    match deleted {
        true => println!("The key '{}' was deleted", key),
        false => println!("The key '{}' was not found", key),
    }
}

fn handle_exists(exists: bool, key: &str) {
    match exists {
        true => println!("The key '{}' exists", key),
        false => println!("The key '{}' does not exist", key),
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut store = Store::new();
    let mut stdout = io::stdout();

    loop {
        stdout.write_all(b"> ")?;
        stdout.flush()?;

        io::stdin().read_line(&mut input)?;
        let res = input.trim();
        if res == "quit" {
            break;
        }
        let tokens: Vec<&str> = res.split_whitespace().collect();

        match get_command(&tokens) {
            Ok(Command::SET { key, value }) => store.set(key, value),
            Ok(Command::GET { key }) => handle_get(store.get(key)),
            Ok(Command::DEL { key }) => handle_del(store.del(key), key),
            Ok(Command::EXISTS { key }) => handle_exists(store.exists(key), key),

            Err(ParseError::UnknownCommand { command }) => handle_command_error(command),
            Err(ParseError::ArgumentCountMismatch { expected, actual }) => {
                handle_arg_error(expected, actual)
            }
            Err(ParseError::EmptyCommand) => (),
        }

        input.clear();
    }
    Ok(())
}
