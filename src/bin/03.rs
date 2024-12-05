#![feature(slice_split_once)]

use core::str;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let input = input.as_bytes();

    let mut result = 0;

    let mut i = 0;
    while i < input.len() - 3 {
        match &input[i..i + 4] {
            b"mul(" => {
                i += 4;
                let Some((num1_str, rest)) = input[i..].split_once(|x| *x == b',') else {
                    continue;
                };
                let Some((num2_str, _)) = rest.split_once(|x| *x == b')') else {
                    continue;
                };
                let Ok(num1) = str::from_utf8(num1_str).unwrap().parse::<u32>() else {
                    continue;
                };
                let Ok(num2) = str::from_utf8(num2_str).unwrap().parse::<u32>() else {
                    continue;
                };
                result += num1 * num2;
            }
            _ => i += 1,
        }
    }

    Some(result)
}

fn part_two(input: &str) -> Option<u32> {
    let input = input.as_bytes();

    let mut result = 0;
    let mut enabled = true;

    let mut i = 0;
    while i < input.len() - 3 {
        match &input[i..i + 4] {
            b"mul(" => {
                i += 4;
                if enabled {
                    let Some((num1_str, rest)) = input[i..].split_once(|x| *x == b',') else {
                        continue;
                    };
                    let Some((num2_str, _)) = rest.split_once(|x| *x == b')') else {
                        continue;
                    };
                    let Ok(num1) = str::from_utf8(num1_str).unwrap().parse::<u32>() else {
                        continue;
                    };
                    let Ok(num2) = str::from_utf8(num2_str).unwrap().parse::<u32>() else {
                        continue;
                    };
                    result += num1 * num2;
                }
            }
            b"do()" => {
                i += 4;
                enabled = true;
            }
            b"don'" => {
                i += 4;
                if &input[i..i + 3] == b"t()" {
                    i += 3;
                    enabled = false;
                }
            }
            _ => i += 1,
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
