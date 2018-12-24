use day5::{read_polymer, simplify};
use std::io;

fn main() -> io::Result<()> {
    println!("{}", simplify(read_polymer()?).len());
    Ok(())
}
