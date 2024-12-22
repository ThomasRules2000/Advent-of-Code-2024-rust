use itertools::Itertools;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, false)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, true)
}

fn solve(input: &str, allow_concat: bool) -> Option<u64> {
    Some(
        parser(input)
            .into_iter()
            .filter(|(target, nums)| {
                check_equation(target, allow_concat, &nums[0], nums[1..].to_vec())
            })
            .map(|(target, _)| target)
            .sum(),
    )
}

fn parser(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|s| {
            let (target, nums) = s.split(": ").collect_tuple().unwrap();
            let target = target.parse().unwrap();
            let nums = nums
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            (target, nums)
        })
        .collect()
}

fn check_equation(target: &u64, allow_concat: bool, acc: &u64, nums: Vec<u64>) -> bool {
    if acc > target {
        return false;
    }
    if nums.is_empty() {
        return target == acc;
    }
    let add_result = acc + nums[0];
    let mul_result = acc * nums[0];
    let concat_result = acc * next_pow_10(nums[0]) + nums[0];
    if allow_concat && check_equation(target, allow_concat, &concat_result, nums[1..].to_vec()) {
        return true;
    }
    if check_equation(target, allow_concat, &mul_result, nums[1..].to_vec()) {
        return true;
    }
    check_equation(target, allow_concat, &add_result, nums[1..].to_vec())
}

fn next_pow_10(num: u64) -> u64 {
    if num < 10 {
        return 10;
    }
    10 * next_pow_10(num / 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
