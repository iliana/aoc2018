use day3::Claim;
use failure::Fallible;
use std::collections::HashMap;
use std::io::{self, BufRead, BufReader};

fn main() -> Fallible<()> {
    let mut fabric = HashMap::new();

    for line in BufReader::new(io::stdin()).lines() {
        let claim: Claim = line?.parse()?;
        for x in claim.x_range() {
            for y in claim.y_range() {
                *fabric.entry((x, y)).or_insert(0) += 1;
            }
        }
    }

    println!("{}", fabric.values().filter(|v| **v >= 2).count());

    Ok(())
}
