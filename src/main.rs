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

    pub fn names<'a>(input: &'a [u8], mut cb: impl FnMut(&'a [u8])) -> Result<(), Error> {
        let mut cursor = input;
        loop {
            let (name, ncursor) = consume_until(cursor, b'\n')?;
            match name.get(0) {
                Some(c) if *c == b'*' => return Ok(()),
                None => return Err(Error::Exhausted),
                _ => cb(name),
            }
            cursor = ncursor;
        }
    }
}

use parse::Error;
use std::io::{stdin, Read};
use std::collections::HashMap;
use std::str;

fn main() -> Result<(), Error> {
    let mut buf = Vec::with_capacity(1024 * 1024);
    stdin().read_to_end(&mut buf)?;

    let votes_to_win = {
        let mut num_votes = 0u32;
        parse::names(&buf, |_| num_votes += 1)?;
        num_votes / 2
    };

    let mut votes_per_name = HashMap::with_capacity(256);
    parse::names(&buf, |n| {
        let votes = votes_per_name.entry(n).or_insert(0_u32);
        *votes += 1;
        if *votes > votes_to_win {
            println!("{}", str::from_utf8(n).unwrap());
            std::process::exit(0);
        }
    })?;

    let mut highest_score = None;
    let mut second_best = None;
    for (name, votes) in votes_per_name.iter() {
        match highest_score {
            Some((lname, lvotes)) if votes > lvotes => {
                second_best = Some((lname, lvotes));
                highest_score = Some((name, votes))
            }
            Some((_, lvotes)) if votes == lvotes => {
                second_best = Some((name, votes));
            }
            Some(_) => {}
            None => highest_score = Some((name, votes)),
        }
    }

    match (highest_score, second_best) {
        (Some((hname, hvotes)), Some((_, svotes))) => {
            if hvotes == svotes {
                println!("Runoff!");
            } else {
                println!("{}", str::from_utf8(hname).unwrap());
            }
        }
        (Some((hname, _)), None) => println!("{}", str::from_utf8(hname).unwrap()),
        _ => unreachable!("should have at least two candidates"),
    }

    Ok(())
}
