use std::collections::HashMap;
use std::io;
use std::io::Write;

fn handle_error(length: usize, right_length: usize) {
    println!("Expected {} arguments, got {}", length, right_length);
}

fn handle_set(values: &mut HashMap<String, String>, command_params: &[&str]) {
    if command_params.len() == 2 {
        values.insert(command_params[0].to_string(), command_params[1].to_string());
    } else {
        handle_error(2, command_params.len());
    }
}

fn handle_get(values: &HashMap<String, String>, command_params: &[&str]) {
    if command_params.len() == 1 {
        match values.get(command_params[0]) {
            Some(value) => println!("{}", value),
            _ => (),
        }
    } else {
        handle_error(1, command_params.len());
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
        match tokens.first() {
            Some(&"SET") => handle_set(&mut values, &tokens[1..]),
            Some(&"GET") => handle_get(&values, &tokens[1..]),
            _ => (),
        }
        input.clear();
    }
    Ok(())
}
