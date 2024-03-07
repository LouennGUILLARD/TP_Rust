use exquisite_corpse::make_reponse;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut text = String::from(""); // Change the type of `text` to `&mut String`
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;

    let response = make_reponse(&line, &mut text); // Change the argument type to `&mut text`

    io::stdout().write_all(response.as_bytes())?;
    Ok(())
}
