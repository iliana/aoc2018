use std::io::{self, Read};

pub fn read_polymer() -> io::Result<Vec<u8>> {
    io::stdin()
        .bytes()
        .filter_map(|b| match b {
            Ok(b) => {
                if b.is_ascii_alphabetic() {
                    Some(Ok(b))
                } else {
                    None
                }
            }
            Err(e) => Some(Err(e)),
        })
        .collect()
}

pub fn reacts(a: u8, b: u8) -> bool {
    a.is_ascii_uppercase() ^ b.is_ascii_uppercase()
        && a.to_ascii_lowercase() == b.to_ascii_lowercase()
}

pub fn simplify_once(s: &[u8]) -> Vec<u8> {
    let mut s = s.to_vec();
    let mut cursor = 0;
    while cursor < (s.len() - 1) {
        if reacts(s[cursor], s[cursor + 1]) {
            s.remove(cursor + 1);
            s.remove(cursor);
        } else {
            cursor += 1;
        }
    }
    s
}

pub fn simplify(s: Vec<u8>) -> Vec<u8> {
    let mut s = s;
    loop {
        let new = simplify_once(&s);
        if new == s {
            break;
        } else {
            s = new;
        }
    }
    s
}

#[cfg(test)]
#[test]
fn test() {
    assert_eq!(simplify(b"dabAcCaCBAcCcaDA".to_vec()).len(), 10);
}
