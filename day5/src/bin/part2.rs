use day5::{read_polymer, simplify};
use std::io;

fn filter_polymer(polymer: &[u8], n: u8) -> Vec<u8> {
    let n = ('a' as u8) + n;
    polymer
        .iter()
        .filter(|c| c.to_ascii_lowercase() != n)
        .cloned()
        .collect()
}

fn shortest(polymer: &[u8]) -> usize {
    (0..26)
        .map(|n| simplify(filter_polymer(&polymer, n)).len())
        .min()
        .unwrap()
}

fn main() -> io::Result<()> {
    let polymer = read_polymer()?;
    println!("{}", shortest(&polymer));
    Ok(())
}

#[cfg(test)]
#[test]
fn test() {
    assert_eq!(shortest(b"dabAcCaCBAcCcaDA"), 4);
}
