use std::{fmt, io};

#[derive(Debug)]
pub enum InvalidInput {
    ExpectedUtf8Args,
    Empty,
    UnknownInputFormat,
}

impl fmt::Display for InvalidInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InvalidInput::ExpectedUtf8Args => write!(f, "Expected a UTF-8 arguments"),
            InvalidInput::Empty => write!(f, "Expected either a mnemonic or a mnemonic entropy"),
            InvalidInput::UnknownInputFormat => write!(f, "Unknown input format"),
        }
    }
}

impl From<InvalidInput> for io::Error {
    fn from(error: InvalidInput) -> Self {
        io::Error::new(io::ErrorKind::InvalidInput, error.to_string())
    }
}

pub enum WalletInput {
    Mnemonic(String),
    Entropy(String),
}

impl WalletInput {
    /// Takes command line arguments and tries to determine the input format:
    /// either a mnemonic or a mnemonic entropy.
    pub fn parse_cmd_args() -> Result<WalletInput, InvalidInput> {
        let args: Vec<_> = std::env::args_os()
            .map(|arg| arg.into_string())
            .collect::<Result<_, _>>()
            .map_err(|_| InvalidInput::ExpectedUtf8Args)?;

        match args.len() {
            0 | 1 => Err(InvalidInput::Empty),
            2 => Ok(WalletInput::Entropy(args[1].clone())),
            // Compile the words into a single string.
            _ => Ok(WalletInput::Mnemonic(args[1..].join(" "))),
        }
    }
}
