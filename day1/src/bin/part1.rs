use failure::Fallible;
use std::io::{self, BufRead, BufReader, Read};
use std::str::FromStr;

fn sum_lines<R: Read>(reader: R) -> Fallible<isize> {
    BufReader::new(reader)
        .lines()
        .map(|l| {
            l.map_err(failure::Error::from)
                .and_then(|l| isize::from_str(&l).map_err(failure::Error::from))
        })
        .sum()
}

fn main() -> Fallible<()> {
    println!("{}", sum_lines(io::stdin())?);
    Ok(())
}
