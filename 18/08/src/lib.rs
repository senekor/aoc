use winnow::{
    ascii::dec_uint,
    combinator::{preceded, repeat},
    PResult, Parser,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn parse(input: &mut &str) -> PResult<Self> {
        let num_children: usize = dec_uint(input)?;
        ' '.parse_next(input)?;
        let num_metadata: usize = dec_uint(input)?;

        let children = repeat(num_children, preceded(' ', Node::parse)).parse_next(input)?;
        let metadata =
            repeat(num_metadata, preceded(' ', dec_uint::<_, usize, _>)).parse_next(input)?;

        Ok(Self { children, metadata })
    }

    fn metadata_sum(&self) -> usize {
        self.children
            .iter()
            .map(|c| c.metadata_sum())
            .chain(self.metadata.iter().copied())
            .sum()
    }

    fn value(&self) -> usize {
        if self.children.is_empty() {
            return self.metadata_sum();
        }
        self.metadata
            .iter()
            .flat_map(|m| self.children.get(m.wrapping_sub(1)))
            .map(|n| n.value())
            .sum()
    }
}

pub fn part1(mut input: &str) -> usize {
    let root = Node::parse(&mut input).unwrap();
    root.metadata_sum()
}

pub fn part2(mut input: &str) -> usize {
    let root = Node::parse(&mut input).unwrap();
    root.value()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_node() {
        let node = Node::parse(&mut "0 1 99").unwrap();
        let expected = Node {
            children: vec![],
            metadata: vec![99],
        };
        assert_eq!(node, expected);
    }
}
