use failure::Fallible;
use std::io::{self, BufRead, BufReader};

fn read_int(v: io::Result<Vec<u8>>) -> Fallible<Option<u32>> {
    let s = String::from_utf8(v?)?;
    let s = s.trim().to_owned();
    Ok(if s.is_empty() { None } else { Some(s.parse()?) })
}

fn read_ints<R: BufRead>(reader: R) -> impl Iterator<Item = Fallible<u32>> {
    reader.split(b' ').filter_map(|v| match read_int(v) {
        Ok(Some(n)) => Some(Ok(n)),
        Ok(None) => None,
        Err(e) => Some(Err(e)),
    })
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
    children: Vec<Box<Node>>,
    metadata: Vec<u32>,
}

impl Node {
    fn read(iter: &mut impl Iterator<Item = Fallible<u32>>) -> Fallible<Node> {
        let num_children = iter.next().unwrap()?;
        let num_metadata = iter.next().unwrap()?;
        let mut children = Vec::with_capacity(num_children as usize);
        let mut metadata = Vec::with_capacity(num_metadata as usize);

        for _ in 0..num_children {
            children.push(Box::new(Node::read(iter)?));
        }

        for _ in 0..num_metadata {
            metadata.push(iter.next().unwrap()?);
        }

        Ok(Node { children, metadata })
    }

    fn sum_metadata(&self) -> u32 {
        self.children
            .iter()
            .map(|c| c.sum_metadata())
            .chain(self.metadata.iter().cloned())
            .sum()
    }

    fn value(&self) -> u32 {
        if self.children.is_empty() {
            self.metadata.iter().sum()
        } else {
            self.metadata
                .iter()
                .map(|x| match self.children.get((*x as usize) - 1) {
                    Some(node) => node.value(),
                    None => 0,
                })
                .sum()
        }
    }
}

fn main() -> Fallible<()> {
    let root = Node::read(&mut read_ints(BufReader::new(io::stdin())))?;
    println!("Part 1: {}", root.sum_metadata());
    println!("Part 2: {}", root.value());
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    static TEST_DATA: &[u8] = b"2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    fn read_test_data() -> impl Iterator<Item = Fallible<u32>> {
        read_ints(BufReader::new(&TEST_DATA[..]))
    }

    #[test]
    fn test_read_ints() {
        assert_eq!(
            read_test_data().collect::<Result<Vec<_>, _>>().unwrap(),
            vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2],
        );
    }

    #[test]
    fn test_node_from() {
        assert_eq!(
            Node::read(&mut read_test_data()).unwrap(),
            Node {
                children: vec![
                    Box::new(Node {
                        children: vec![],
                        metadata: vec![10, 11, 12],
                    }),
                    Box::new(Node {
                        children: vec![Box::new(Node {
                            children: vec![],
                            metadata: vec![99],
                        }),],
                        metadata: vec![2],
                    }),
                ],
                metadata: vec![1, 1, 2],
            }
        );
    }

    #[test]
    fn test_sum_metadata() {
        assert_eq!(
            Node::read(&mut read_test_data()).unwrap().sum_metadata(),
            138
        );
    }

    #[test]
    fn test_node_value() {
        let node = Node::read(&mut read_test_data()).unwrap();
        assert_eq!(node.children[0].value(), 33);
        assert_eq!(node.children[1].children[0].value(), 99);
        assert_eq!(node.children[1].value(), 0);
        assert_eq!(node.value(), 66);
    }
}
