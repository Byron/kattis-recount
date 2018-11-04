mod parse {
    use std::io;

    #[derive(Debug)]
    pub enum Error {
        Exhausted,
        Io(io::Error),
    }

    impl From<io::Error> for Error {
        fn from(err: io::Error) -> Self {
            Error::Io(err)
        }
    }

    pub fn consume_until(input: &[u8], stop_byte: u8) -> Result<(&[u8], &[u8]), Error> {
        let (input, remainder) = input.split_at(input
            .iter()
            .position(|b| *b == stop_byte)
            .ok_or(Error::Exhausted)?);

        Ok((input, &remainder[1..]))
    }
}

use parse::Error;
use std::io::{stdin, stdout, BufWriter, Read, Write};
use std::str;

fn main() -> Result<(), Error> {
    let mut buf = Vec::with_capacity(1024 * 1024);
    stdin().read_to_end(&mut buf)?;

    Ok(())
}
