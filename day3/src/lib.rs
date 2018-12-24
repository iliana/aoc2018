use std::num::ParseIntError;
use std::ops::Range;
use std::str::FromStr;

trait RangeExt {
    fn overlap(&self, other: &Self) -> bool;
}

impl RangeExt for Range<u16> {
    fn overlap(&self, other: &Self) -> bool {
        !((self.end - 1) < other.start || self.start > (other.end - 1))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Claim {
    pub id: u16,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Claim {
    pub fn x_range(&self) -> Range<u16> {
        self.x..(self.x + self.width)
    }

    pub fn y_range(&self) -> Range<u16> {
        self.y..(self.y + self.height)
    }

    pub fn overlap(&self, other: &Claim) -> bool {
        self.x_range().overlap(&other.x_range()) && self.y_range().overlap(&other.y_range())
    }
}

impl FromStr for Claim {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Claim, ParseIntError> {
        let mut iter = s.split(' ');
        let id = iter.next().unwrap().trim_start_matches('#');
        iter.next();
        let (x, y) = {
            let mut iter = iter.next().unwrap().trim_end_matches(':').split(',');
            (iter.next().unwrap(), iter.next().unwrap())
        };
        let (width, height) = {
            let mut iter = iter.next().unwrap().split('x');
            (iter.next().unwrap(), iter.next().unwrap())
        };
        Ok(Claim {
            id: id.parse()?,
            x: x.parse()?,
            y: y.parse()?,
            width: width.parse()?,
            height: height.parse()?,
        })
    }
}

#[cfg(test)]
#[test]
fn test_claim_from_str() {
    assert_eq!(
        "#1 @ 1,3: 4x4".parse(),
        Ok(Claim {
            id: 1,
            x: 1,
            y: 3,
            width: 4,
            height: 4,
        })
    );
}

#[cfg(test)]
#[test]
fn test_range_overlap() {
    assert_eq!((3..8).overlap(&(1..3)), false);
    assert_eq!((3..8).overlap(&(2..4)), true);
    assert_eq!((3..8).overlap(&(4..7)), true);
    assert_eq!((3..8).overlap(&(7..9)), true);
    assert_eq!((3..8).overlap(&(8..10)), false);
    assert_eq!((1..3).overlap(&(3..8)), false);
    assert_eq!((2..4).overlap(&(3..8)), true);
    assert_eq!((4..7).overlap(&(3..8)), true);
    assert_eq!((7..9).overlap(&(3..8)), true);
    assert_eq!((8..10).overlap(&(3..8)), false);
}

#[cfg(test)]
#[test]
fn test_claim_overlap() {
    let a = Claim::from_str("#1 @ 1,3: 4x4").unwrap();
    let b = Claim::from_str("#2 @ 3,1: 4x4").unwrap();
    let c = Claim::from_str("#3 @ 5,5: 2x2").unwrap();

    assert_eq!(a.overlap(&b), true);
    assert_eq!(b.overlap(&a), true);
    assert_eq!(a.overlap(&c), false);
    assert_eq!(c.overlap(&a), false);
    assert_eq!(b.overlap(&c), false);
    assert_eq!(c.overlap(&b), false);
}
