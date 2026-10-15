use crate::command::Command;

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_command() {
        let command = get_command(&["SET", "asd", "sda"]);
        assert_eq!(command, Ok(Command::SET {
            key: "asd",
            value: "sda",
        }));
    }

    #[test]
    fn test_set_command_invalid() {
        let command = get_command(&["SET"]);
        assert_eq!(command, Err(ParseError::ArgumentCountMismatch {
            expected: 2,
            actual: 0,
        }));
        let command = get_command(&["SET", "asd"]);
        assert_eq!(command, Err(ParseError::ArgumentCountMismatch {
            expected: 2,
            actual: 1,
        }));
        let command = get_command(&["SET", "asd", "asd", "asd"]);
        assert_eq!(command, Err(ParseError::ArgumentCountMismatch {
            expected: 2,
            actual: 3,
        }));
    }

    #[test]
    fn test_get_command() {
        let command = get_command(&["GET", "asd"]);
        assert_eq!(command, Ok(Command::GET { key: "asd" }));
    }

    #[test]
    fn test_get_command_invalid() {
        let command = get_command(&["GET"]);
        assert_eq!(command, Err(ParseError::ArgumentCountMismatch {
            expected: 1,
            actual: 0,
        }));
        let command = get_command(&["GET", "asd", "asd"]);
        assert_eq!(command, Err(ParseError::ArgumentCountMismatch {
            expected: 1,
            actual: 2,
        }));
    }

    #[test]
    fn test_del_command() {
        let command = get_command(&["DEL", "asd"]);
        assert_eq!(command, Ok(Command::DEL { key: "asd" }));
    }

    #[test]
    fn test_del_command_invalid() {
        let command = get_command(&["DEL"]);
        assert_eq!(command, Err(ParseError::ArgumentCountMismatch {
            expected: 1,
            actual: 0,
        }));
        let command = get_command(&["DEL", "asd", "asd"]);
        assert_eq!(command, Err(ParseError::ArgumentCountMismatch {
            expected: 1,
            actual: 2,
        }));
    }

    #[test]
    fn test_exists_command() {
        let command = get_command(&["EXISTS", "asd"]);
        assert_eq!(command, Ok(Command::EXISTS { key: "asd" }));
    }

    #[test]
    fn test_exists_command_invalid() {
        let command = get_command(&["EXISTS"]);
        assert_eq!(command, Err(ParseError::ArgumentCountMismatch {
            expected: 1,
            actual: 0,
        }));
        let command = get_command(&["EXISTS", "asd", "asd"]);
        assert_eq!(command, Err(ParseError::ArgumentCountMismatch {
            expected: 1,
            actual: 2,
        }));
    }

    #[test]
    fn test_unknown_command() {
        let command = get_command(&["asd"]);
        assert_eq!(command, Err(ParseError::UnknownCommand { command: "asd" }));
        let command = get_command(&["ASD"]);
        assert_eq!(command, Err(ParseError::UnknownCommand { command: "ASD" }));
    }

    #[test]
    fn test_empty_command() {
        let command = get_command(&[]);
        assert_eq!(command, Err(ParseError::EmptyCommand));
    }
}
