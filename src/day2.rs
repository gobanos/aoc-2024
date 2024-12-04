use aoc_runner_derive::{aoc, aoc_generator};
use nom::bytes::complete::tag;
use nom::character::complete::{newline, u8 as parse_u8};
use nom::combinator::map;
use nom::multi::separated_list1;
use std::cmp::Ordering;

#[derive(Debug)]
struct Report {
    levels: Vec<u8>,
}

impl Report {
    fn is_safe(&self) -> bool {
        let filter = match self.levels[1].cmp(&self.levels[0]) {
            Ordering::Greater => all_increasing,
            Ordering::Less => all_decreasing,
            Ordering::Equal => return false,
        };

        self.levels
            .iter()
            .copied()
            .zip(self.levels.iter().skip(1).copied())
            .all(filter)
    }

    fn dampened(&self) -> impl Iterator<Item = Report> + use<'_> {
        (0..self.levels.len()).map(|i| {
            let mut levels = self.levels.clone();
            levels.remove(i);
            Report { levels }
        })
    }
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Report> {
    separated_list1(
        newline,
        map(separated_list1(tag(" "), parse_u8::<_, ()>), |levels| {
            Report { levels }
        }),
    )(input)
    .unwrap()
    .1
}

#[aoc(day2, part1)]
fn part1(reports: &[Report]) -> usize {
    reports.iter().filter(|report| report.is_safe()).count()
}

fn all_decreasing((a, b): (u8, u8)) -> bool {
    a > b && a - b <= 3
}

fn all_increasing((a, b): (u8, u8)) -> bool {
    b > a && b - a <= 3
}

#[aoc(day2, part2)]
fn part2(reports: &[Report]) -> usize {
    reports
        .iter()
        .filter(|report| report.is_safe() || report.dampened().any(|report| report.is_safe()))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 4);
    }
}
