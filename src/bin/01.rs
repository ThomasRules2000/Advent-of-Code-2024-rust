#![feature(try_blocks)]

use std::iter::zip;

use itertools::Itertools;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (list1, list2) = parse(input);
    Some(zip(list1, list2).map(|(x, y)| x.abs_diff(y)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (list1, list2) = parse(input);

    let mut i1 = list1.chunk_by(|x, y| x == y);
    let mut i2 = list2.chunk_by(|x, y| x == y);

    let mut res = 0;
    let _: Option<()> = try {
        loop {
            let mut x1 = i1.next()?;
            let mut x2 = i2.next()?;

            while x1[0] != x2[0] {
                if x1[0] < x2[0] {
                    x1 = i1.next()?;
                } else {
                    x2 = i2.next()?;
                }
            }
            res += x1[0] * x1.len() as u32 * x2.len() as u32;
        }
    };
    Some(res)
}

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    let (mut list1, mut list2): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip();

    list1.sort_unstable();
    list2.sort_unstable();

    (list1, list2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
