use aoc_runner_derive::{aoc, aoc_generator};
use nom::character::complete::{multispace1, newline, u32 as parse_u32};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use std::iter::Zip;
use std::vec::IntoIter;

#[derive(Clone, Debug)]
struct List {
    left: Vec<u32>,
    right: Vec<u32>,
}

impl FromIterator<(u32, u32)> for List {
    fn from_iter<T: IntoIterator<Item = (u32, u32)>>(iter: T) -> Self {
        let iterator = iter.into_iter();
        let (min, maybe_max) = iterator.size_hint();
        let mut left = Vec::with_capacity(maybe_max.unwrap_or(min));
        let mut right = Vec::with_capacity(maybe_max.unwrap_or(min));

        for (a, b) in iterator {
            left.push(a);
            right.push(b);
        }

        Self { left, right }
    }
}

impl IntoIterator for List {
    type Item = (u32, u32);
    type IntoIter = Zip<IntoIter<u32>, IntoIter<u32>>;

    fn into_iter(self) -> Self::IntoIter {
        self.left.into_iter().zip(self.right)
    }
}

#[aoc_generator(day1)]
fn parse(input: &str) -> List {
    separated_list1(
        newline,
        map(
            separated_pair(parse_u32::<_, ()>, multispace1, parse_u32::<_, ()>),
            |(a, b)| (a, b),
        ),
    )(input)
    .unwrap()
    .1
    .into_iter()
    .collect()
}

#[aoc(day1, part1)]
fn part1(list: &List) -> u32 {
    let mut list = list.clone();
    list.left.sort();
    list.right.sort();

    list.into_iter()
        .map(|(a, b)| u32::max(a, b) - u32::min(a, b))
        .sum()
}

#[aoc(day1, part2)]
fn part2(input: &List) -> u32 {
    input
        .left
        .iter()
        .map(|left| left * input.right.iter().filter(|&right| left == right).count() as u32)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 11);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 31);
    }
}
