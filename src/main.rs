use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();

    loop {
        io::stdin().read_line(&mut input)?;
        let res = input.trim();
        if res == "quit" {
            break;
        }
        for i in res.split_whitespace() {
            println!("{}", i);
        }
        input.clear();
    }
    Ok(())
}
