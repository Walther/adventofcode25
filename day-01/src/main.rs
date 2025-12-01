fn main() {
    const INPUT: &str = include_str!("input.txt");
    let parsed = parse(INPUT);

    let value = part1(&parsed);
    println!("Part 1: {value}");

    let value = part2(&parsed);
    println!("Part 2: {value}");
}

const DIAL_SIZE: usize = 100;

#[derive(Debug)]
struct Dial {
    value: usize,
    zeroes: usize,
}

#[derive(Clone, Copy, Debug)]
enum Rotation {
    Left(usize),
    Right(usize),
}

impl Dial {
    pub fn new() -> Self {
        Self {
            value: 50,
            zeroes: 0,
        }
    }

    pub fn rotate(&mut self, rotation: Rotation) {
        let net = match rotation {
            Rotation::Left(count) => DIAL_SIZE - (count % DIAL_SIZE),
            Rotation::Right(count) => count % DIAL_SIZE,
        };
        self.value = (self.value + net) % DIAL_SIZE;
        if self.value == 0 {
            self.zeroes += 1;
        }
    }

    #[allow(non_snake_case)]
    pub fn method_0x434C49434B(&mut self, rotation: Rotation) {
        // FIXME: naive solution
        match rotation {
            Rotation::Left(count) => {
                for _ in 0..count {
                    if self.value == 0 {
                        self.zeroes += 1;
                        self.value = 99;
                    } else {
                        self.value -= 1;
                    }
                }
            }
            Rotation::Right(count) => {
                for _ in 0..count {
                    if self.value == 0 {
                        self.zeroes += 1;
                        self.value += 1;
                    } else if self.value == 99 {
                        self.value = 0;
                    } else {
                        self.value += 1;
                    }
                }
            }
        }
    }

    pub fn zeroes(&self) -> usize {
        self.zeroes
    }
}

type ParsedData = Vec<Rotation>;

fn parse(input: &str) -> ParsedData {
    let mut rotations = Vec::new();
    for line in input.lines() {
        let (direction, number) = line.split_at(1);
        let number: usize = number.parse().expect("Could not parse dial number");
        match direction {
            "L" => rotations.push(Rotation::Left(number)),
            "R" => rotations.push(Rotation::Right(number)),
            _ => panic!("Could not parse dial direction"),
        }
    }
    rotations
}

fn part1(data: &ParsedData) -> usize {
    let mut dial = Dial::new();
    for &rotation in data {
        dial.rotate(rotation);
    }
    dial.zeroes()
}

fn part2(data: &ParsedData) -> usize {
    let mut dial = Dial::new();
    for &rotation in data {
        dial.method_0x434C49434B(rotation);
    }
    dial.zeroes()
}

#[cfg(test)]
mod integration {
    const INPUT: &str = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    #[test]
    fn part1() {
        let parsed = crate::parse(INPUT);
        let value = crate::part1(&parsed);
        let expected = 3;
        assert_eq!(value, expected);
    }

    #[test]
    fn part2() {
        let parsed = crate::parse(INPUT);
        let value = crate::part2(&parsed);
        let expected = 6;
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
