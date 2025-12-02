use std::collections::HashSet;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let parsed = parse(INPUT);

    let value = part1(&parsed);
    println!("Part 1: {value}");

    let value = part2(&parsed);
    println!("Part 2: {value}");
}

struct ProductIDRange {
    start: usize,
    end: usize,
}

impl ProductIDRange {
    pub fn invalid_ids_part1(&self) -> HashSet<usize> {
        let mut invalid = HashSet::new();
        for id in self.start..=self.end {
            let str = id.to_string();
            let len = str.len();
            let (left, right) = str.split_at(len / 2);
            if left == right {
                invalid.insert(id);
            }
        }
        invalid
    }

    pub fn invalid_ids_part2(&self) -> HashSet<usize> {
        let mut invalid = HashSet::new();
        for id in self.start..=self.end {
            let str: Vec<char> = id.to_string().chars().collect();
            for length in 1..=(str.len() / 2) {
                let first: Vec<char> = str.iter().take(length).copied().collect();
                let mut chunks = str.chunks_exact(length);
                if !chunks.remainder().is_empty() {
                    continue;
                }
                if chunks.all(|s| s == first) {
                    invalid.insert(id);
                }
            }
        }
        invalid
    }
}

type ParsedData = Vec<ProductIDRange>;

fn parse(input: &str) -> ParsedData {
    let mut ranges = Vec::new();
    for line in input.lines() {
        for range in line.split(',') {
            let (left, right) = range
                .split_once('-')
                .expect("Unable to parse product ID range");
            let start: usize = left.parse().expect("Invalid start for product ID range");
            let end: usize = right.parse().expect("Invalid end for product ID range");
            ranges.push(ProductIDRange { start, end });
        }
    }
    ranges
}

fn part1(data: &ParsedData) -> usize {
    data.iter()
        .flat_map(ProductIDRange::invalid_ids_part1)
        .sum()
}

fn part2(data: &ParsedData) -> usize {
    data.iter()
        .flat_map(ProductIDRange::invalid_ids_part2)
        .sum()
}

#[cfg(test)]
mod integration {
    const INPUT: &str = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn part1() {
        let parsed = crate::parse(INPUT);
        let value = crate::part1(&parsed);
        let expected = 1_227_775_554;
        assert_eq!(value, expected);
    }

    #[test]
    fn part2() {
        let parsed = crate::parse(INPUT);
        let value = crate::part2(&parsed);
        let expected = 4_174_379_265;
        assert_eq!(value, expected);
    }
}

#[cfg(test)]
mod unit {
    #[test]
    fn unit() {
        let value = 0;
        let expected = 0;
        assert_eq!(value, expected);
    }
}
