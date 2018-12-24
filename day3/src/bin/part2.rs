use day3::Claim;
use failure::Fallible;
use std::io::{self, BufRead, BufReader};

fn main() -> Fallible<()> {
    let mut claims = BufReader::new(io::stdin())
        .lines()
        .map(|line| -> Fallible<Claim> { line?.parse().map_err(failure::Error::from) })
        .collect::<Result<Vec<_>, _>>()?;
    let mut checked = Vec::new();

    while let Some(a) = claims.pop() {
        if claims.iter().chain(checked.iter()).all(|b| !a.overlap(b)) {
            println!("{}", a.id);
        }
        checked.push(a);
    }

    Ok(())
}
