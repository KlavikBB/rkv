use crate::command::Command;

pub enum ParseError<'a> {
    UnknownCommand { command: &'a str },
    ArgumentCountMismatch { expected: usize, actual: usize },
    EmptyCommand,
}

pub fn get_command<'a>(command: &[&'a str]) -> Result<Command<'a>, ParseError<'a>> {
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
