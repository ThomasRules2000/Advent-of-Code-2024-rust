advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(parser(input).into_iter().filter(check_safe).count() as u32)
}

fn check_safe(report: &Vec<i32>) -> bool {
    let windows: Vec<_> = report.windows(2).collect();
    windows.iter().all(|w| safe_increasing(w[0], w[1]))
        || windows.into_iter().all(|w| safe_decreasing(w[0], w[1]))
}

fn safe_increasing(x1: i32, x2: i32) -> bool {
    let diff = x1 - x2;
    diff >= 1 && diff <= 3
}

fn safe_decreasing(x1: i32, x2: i32) -> bool {
    let diff = x2 - x1;
    diff >= 1 && diff <= 3
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(parser(input).into_iter().filter(check_safe_2).count() as u32)
}

fn check_safe_2(report: &Vec<i32>) -> bool {
    check_inc_with_remove(report) || check_dec_with_remove(report)
}

fn check_inc_with_remove(report: &Vec<i32>) -> bool {
    let mut removed = false;
    let mut i = 0;
    while i < report.len() - 1 {
        let diff = report[i] - report[i + 1];
        if diff < 1 || diff > 3 {
            if removed {
                return false;
            }
            if i == report.len() - 2 {
                break;
            }

            removed = true;

            // Remove element at i+1
            let diff2 = report[i] - report[i + 2];
            if diff2 >= 1 && diff2 <= 3 {
                i += 2;
                continue;
            }

            if i > 0 {
                // Remove element at i
                let diff1 = report[i - 1] - report[i + 1];
                if diff1 < 1 || diff1 > 3 {
                    return false;
                }
            }
        }
        i += 1;
    }
    true
}

fn check_dec_with_remove(report: &Vec<i32>) -> bool {
    let mut removed = false;
    let mut i = 0;
    while i < report.len() - 1 {
        let diff = report[i + 1] - report[i];
        if diff < 1 || diff > 3 {
            if removed {
                return false;
            }
            if i == report.len() - 2 {
                break;
            }

            removed = true;

            // Remove element at i+1
            let diff2 = report[i + 2] - report[i];
            if diff2 >= 1 && diff2 <= 3 {
                i += 2;
                continue;
            }

            if i > 0 {
                // Remove element at i
                let diff1 = report[i + 1] - report[i - 1];
                if diff1 < 1 || diff1 > 3 {
                    return false;
                }
            }
        }
        i += 1;
    }
    true
}

fn parser(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
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
