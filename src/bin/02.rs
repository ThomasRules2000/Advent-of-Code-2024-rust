advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(parser(input).into_iter().filter(check_safe).count() as u32)
}

fn check_safe(report: &Vec<u32>) -> bool {
    let windows: Vec<_> = report.windows(2).collect();
    windows.iter().all(|w| safe_increasing(w[0], w[1]))
        || windows.iter().all(|w| safe_decreasing(w[0], w[1]))
}

fn safe_increasing(x1: u32, x2: u32) -> bool {
    let diff: i64 = x1 as i64 - x2 as i64;
    diff >= 1 && diff <= 3
}

fn safe_decreasing(x1: u32, x2: u32) -> bool {
    let diff = x2 as i64 - x1 as i64;
    diff >= 1 && diff <= 3
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(parser(input).into_iter().filter(check_safe_2).count() as u32)
}

fn check_safe_2(report: &Vec<u32>) -> bool {
    for i in 0..report.len() {
        let mut new_report = report.clone();
        new_report.remove(i);
        if check_safe(&new_report) {
            return true
        }
    }
    false
}

fn parser(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
