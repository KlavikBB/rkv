pub enum Command<'a> {
    SET { key: &'a str, value: &'a str },
    GET { key: &'a str },
    DEL { key: &'a str },
    EXISTS { key: &'a str },
}
