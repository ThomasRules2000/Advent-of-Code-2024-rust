use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        Regex::new(r"mul\(\d+\,\d+\)")
            .unwrap()
            .find_iter(input)
            .map(|x| {
                let (x, y): (u32, u32) = x.as_str()[4..x.len() - 1]
                    .split(",")
                    .map(|n| n.parse().unwrap())
                    .collect_tuple()
                    .unwrap();
                x * y
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result = 0;
    let mut enabled = true;
    for m in Regex::new(r"mul\(\d+\,\d+\)|do\(\)|don't\(\)")
        .unwrap()
        .find_iter(input)
    {
        let x = m.as_str();
        match &x[..3] {
            "mul" => {
                if enabled {
                    let (x, y) = x[4..x.len() - 1]
                        .split(",")
                        .map(|n| n.parse::<u32>().unwrap())
                        .collect_tuple()
                        .unwrap();
                    result += x * y;
                }
            }
            "do(" => enabled = true,
            "don" => enabled = false,
            _ => unreachable!(),
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
