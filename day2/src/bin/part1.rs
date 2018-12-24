use std::io::{self, BufRead, BufReader};

fn checksum<R: BufRead>(reader: R) -> io::Result<u64> {
    let mut twos = 0;
    let mut threes = 0;

    for line in reader.lines() {
        let line = line?;
        let mut count = [0; 26];
        for c in line.chars() {
            let pos = (c as u32) - ('a' as u32);
            let pos = pos as usize;
            count[pos] += 1;
        }
        if count.iter().any(|n| *n == 2) {
            twos += 1;
        }
        if count.iter().any(|n| *n == 3) {
            threes += 1;
        }
    }

    Ok(twos * threes)
}

fn main() -> io::Result<()> {
    println!("{}", checksum(BufReader::new(io::stdin()))?);
    Ok(())
}
