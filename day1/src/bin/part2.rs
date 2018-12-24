use failure::Fallible;
use std::collections::HashSet;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;

fn read_values<R: BufRead>(reader: R) -> impl Iterator<Item = Fallible<isize>> {
    BufReader::new(reader).lines().map(|l| {
        l.map_err(failure::Error::from)
            .and_then(|l| isize::from_str(&l).map_err(failure::Error::from))
    })
}

fn main() -> Fallible<()> {
    let values = read_values(BufReader::new(io::stdin())).collect::<Result<Vec<_>, _>>()?;
    let mut sum = 0;
    let mut set = HashSet::new();
    for value in values.iter().cycle() {
        sum += value;
        if !set.insert(sum) {
            println!("{}", sum);
            break;
        }
    }
    Ok(())
}
