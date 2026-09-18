use std::collections::HashMap;
use std::io;
use std::io::Write;

enum Command<'a> {
    SET { key: &'a str, value: &'a str },
    GET { key: &'a str },
    DEL { key: &'a str },
    EXISTS { key: &'a str },
}

fn get_command<'a>(command: &Vec<&'a str>) -> Option<Command<'a>> {
    match command.first() {
        Some(&"SET") => {
            if command.len() == 3 {
                Some(Command::SET {
                    key: command[1],
                    value: command[2],
                })
            } else {
                handle_error(2, command.len() - 1);
                None
            }
        }
        Some(&"GET") => {
            if command.len() == 2 {
                Some(Command::GET { key: command[1] })
            } else {
                handle_error(1, command.len() - 1);
                None
            }
        }
        Some(&"DEL") => {
            if command.len() == 2 {
                Some(Command::DEL { key: command[1] })
            } else {
                handle_error(1, command.len() - 1);
                None
            }
        }
        Some(&"EXISTS") => {
            if command.len() == 2 {
                Some(Command::EXISTS { key: command[1] })
            } else {
                handle_error(1, command.len() - 1);
                None
            }
        }
        _ => None,
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

fn handle_error(length: usize, right_length: usize) {
    if length == 1 {
        println!("Expected {} argument, got {}", length, right_length);
    } else {
        println!("Expected {} arguments, got {}", length, right_length);
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
            Some(Command::SET { key, value }) => store.set(key, value),
            Some(Command::GET { key }) => handle_get(store.get(key)),
            Some(Command::DEL { key }) => handle_del(store.del(key), key),
            Some(Command::EXISTS { key }) => handle_exists(store.exists(key), key),

            _ => (),
        }

        input.clear();
    }
    Ok(())
}
