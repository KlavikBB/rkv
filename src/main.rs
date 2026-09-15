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

fn handle_error(length: usize, right_length: usize) {
    if length == 1 {
        println!("Expected {} argument, got {}", length, right_length);
    } else {
        println!("Expected {} arguments, got {}", length, right_length);
    }
}

fn handle_set(values: &mut HashMap<String, String>, key: &str, value: &str) {
    values.insert(key.to_string(), value.to_string());
}

fn handle_get(values: &HashMap<String, String>, key: &str) {
    match values.get(key) {
        Some(value) => println!("{}", value),
        _ => (),
    }
}

fn handle_del(values: &mut HashMap<String, String>, key: &str) {
    values.remove(key);
}

fn handle_exists(values: &HashMap<String, String>, key: &str) {
    if values.contains_key(key) {
        println!("The key '{}' exists", key);
    } else {
        println!("The key '{}' does not exist", key);
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut values = HashMap::new();
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
            Some(Command::SET { key, value }) => handle_set(&mut values, key, value),
            Some(Command::GET { key }) => handle_get(&values, key),
            Some(Command::DEL { key }) => handle_del(&mut values, key),
            Some(Command::EXISTS { key }) => handle_exists(&values, key),

            _ => (),
        }

        input.clear();
    }
    Ok(())
}
