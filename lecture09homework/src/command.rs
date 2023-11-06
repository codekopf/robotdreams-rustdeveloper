use std::io;
use std::str::FromStr;

#[derive(PartialEq)] // Java equals() annotation with Lombok ;)
pub enum Command {
    Lowercase,
    Uppercase,
    NoSpaces,
    Slugify,
    Sentence,
    Unicode,
    Csv,
}

impl FromStr for Command {
    type Err = io::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "lowercase" => Ok(Command::Lowercase),
            "uppercase" => Ok(Command::Uppercase),
            "no-spaces" => Ok(Command::NoSpaces),
            "slugify" => Ok(Command::Slugify),
            "sentence" => Ok(Command::Sentence),
            "unicode" => Ok(Command::Unicode),
            "csv" => Ok(Command::Csv),
            _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid command!")),
        }
    }
}