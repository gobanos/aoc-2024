use aoc_runner_derive::{aoc, aoc_generator};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, digit1};
use nom::combinator::{map, verify};
use nom::multi::many1;
use nom::sequence::tuple;
use std::str::FromStr;

#[aoc_generator(day3, part1)]
fn parse_part1(input: &str) -> Vec<(u16, u16)> {
    let number_123 = || {
        map(
            verify(digit1::<_, ()>, |s: &str| s.len() <= 3),
            |s: &str| u16::from_str(s).unwrap(),
        )
    };

    let valid_operation = map(
        tuple((tag("mul("), number_123(), tag(","), number_123(), tag(")"))),
        |(_, a, _, b, _)| Some((a, b)),
    );
    many1(alt((valid_operation, map(anychar, |_| None))))(input)
        .unwrap()
        .1
        .into_iter()
        .flatten()
        .collect()
}

#[aoc(day3, part1)]
fn part1(multiplications: &[(u16, u16)]) -> u32 {
    multiplications
        .iter()
        .map(|&(n1, n2)| n1 as u32 * n2 as u32)
        .sum()
}

#[derive(Debug)]
enum Operation {
    Mul(u16, u16),
    Do,
    Dont,
}

#[aoc_generator(day3, part2)]
fn parse_part2(input: &str) -> Vec<Operation> {
    let number_123 = || {
        map(
            verify(digit1::<_, ()>, |s: &str| s.len() <= 3),
            |s: &str| u16::from_str(s).unwrap(),
        )
    };

    let valid_operation = map(
        tuple((tag("mul("), number_123(), tag(","), number_123(), tag(")"))),
        |(_, a, _, b, _)| Some(Operation::Mul(a, b)),
    );
    many1(alt((
        valid_operation,
        map(tag("do()"), |_| Some(Operation::Do)),
        map(tag("don't()"), |_| Some(Operation::Dont)),
        map(anychar, |_| None),
    )))(input)
    .unwrap()
    .1
    .into_iter()
    .flatten()
    .collect()
}

#[aoc(day3, part2)]
fn part2(operations: &[Operation]) -> u32 {
    operations
        .iter()
        .fold((true, 0), |(process, sum), operation| {
            match (process, operation) {
                (_, Operation::Do) => (true, sum),
                (_, Operation::Dont) => (false, sum),
                (true, &Operation::Mul(a, b)) => (process, sum + (a as u32 * b as u32)),
                (false, Operation::Mul(_, _)) => (process, sum),
            }
        })
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_part1(EXAMPLE_1)), 161);
    }

    const EXAMPLE_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_part2(EXAMPLE_2)), 48);
    }
}
