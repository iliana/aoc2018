use std::io::{self, BufRead, BufReader};

fn similar(a: &str, b: &str) -> Option<String> {
    let c: String = a
        .chars()
        .zip(b.chars())
        .filter_map(|(a, b)| if a == b { Some(a) } else { None })
        .collect();
    if c.len() == a.len() - 1 {
        Some(c)
    } else {
        None
    }
}

fn main() -> io::Result<()> {
    let mut lines = BufReader::new(io::stdin())
        .lines()
        .collect::<Result<Vec<_>, _>>()?;
    while let Some(a) = lines.pop() {
        for b in lines.iter() {
            if let Some(c) = similar(&a, &b) {
                println!("{}", c);
            }
        }
    }

    Ok(())
}
