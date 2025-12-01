fn main() {
    const INPUT: &str = include_str!("input.txt");
    let parsed = parse(INPUT);

    let value = part1(&parsed);
    println!("Part 1: {value}");

    let value = part2(&parsed);
    println!("Part 2: {value}");
}

type ParsedData = Vec<()>;

fn parse(_input: &str) -> ParsedData {
    Vec::new()
}

fn part1(_data: &ParsedData) -> usize {
    0
}

fn part2(_data: &ParsedData) -> usize {
    0
}

#[cfg(test)]
mod integration {
    const INPUT: &str = r"";

    #[test]
    fn part1() {
        let parsed = crate::parse(INPUT);
        let value = crate::part1(&parsed);
        let expected = 0;
        assert_eq!(value, expected);
    }

    #[test]
    fn part2() {
        let parsed = crate::parse(INPUT);
        let value = crate::part2(&parsed);
        let expected = 0;
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
