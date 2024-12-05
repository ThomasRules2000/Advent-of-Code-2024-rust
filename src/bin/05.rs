use std::{cmp::Ordering, collections::HashSet};

use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, reports) = parser(input);

    Some(
        reports
            .into_iter()
            .filter(|report| check_valid(&rules, report))
            .map(|report| report[report.len() / 2])
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, reports) = parser(input);

    Some(
        reports
            .into_iter()
            .filter(|report| !check_valid(&rules, report))
            .map(|mut report| {
                report.sort_unstable_by(|x, y| compare(&rules, x, y));
                report[report.len() / 2]
            })
            .sum(),
    )
}

fn parser(input: &str) -> (HashSet<(u32, u32)>, Vec<Vec<u32>>) {
    let (rules, reports) = input.split_once("\n\n").unwrap();

    let rules = rules
        .lines()
        .map(|rule| {
            rule.split("|")
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let reports = reports
        .lines()
        .map(|report| report.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();

    (rules, reports)
}

fn check_valid(rules: &HashSet<(u32, u32)>, report: &Vec<u32>) -> bool {
    let pairs = get_pairs(report);

    rules.is_disjoint(&pairs)
}

fn get_pairs(report: &Vec<u32>) -> HashSet<(u32, u32)> {
    report
        .into_iter()
        .enumerate()
        .map(|(i, x)| report[i..].into_iter().map(|y| (*y, *x)))
        .flatten()
        .collect()
}

fn compare(rules: &HashSet<(u32, u32)>, x: &u32, y: &u32) -> Ordering {
    if rules.contains(&(*x, *y)) {
        Ordering::Less
    } else if rules.contains(&(*y, *x)) {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
