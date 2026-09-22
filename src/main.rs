mod command;
mod parser;
mod store;

use crate::command::Command;
use crate::parser::ParseError;
use crate::parser::get_command;
use crate::store::Store;

use std::io;
use std::io::Write;

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

fn execute_command<'a>(command: Result<Command<'a>, ParseError<'a>>, store: &mut Store) {
    match command {
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
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut store = Store::new();
    let mut stdout = io::stdout();

    loop {
        stdout.write_all(b"> ")?;
        stdout.flush()?;

        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n == 0 {
                    println!();
                    break;
                }
            }
            Err(error) => Err(error)?,
        }
        let res = input.trim();
        if res == "quit" {
            break;
        }
        let tokens: Vec<&str> = res.split_whitespace().collect();

        let command = get_command(&tokens);
        execute_command(command, &mut store);

        input.clear();
    }
    Ok(())
}
