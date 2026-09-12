use std::io;

fn handle_error(length: usize, right_length: usize){
    println!("Expected {} arguments, got {}", length, right_length);
}

fn handle_set(command_params: &[&str]){
    if command_params.len() == 2 {
        for i in command_params {
            println!("{}", i);
        }
    } else {
        handle_error(2, command_params.len());
    }
}

fn handle_get(command_params: &[&str]){
    if command_params.len() == 1 {
        for i in command_params {
            println!("{}", i);
        }
    }  else {
        handle_error(1, command_params.len());
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();

    loop {
        io::stdin().read_line(&mut input)?;
        let res = input.trim();
        if res == "quit" {
            break;
        }
        let tokens: Vec<&str> = res.split_whitespace().collect();
        match tokens.first() {
            Some(&"SET") => handle_set(&tokens[1..]),
            Some(&"GET") => handle_get(&tokens[1..]),
            _ => ()
        }
        input.clear();
    }
    Ok(())
}
