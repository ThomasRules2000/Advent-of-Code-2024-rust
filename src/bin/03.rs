use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parser(input)
            .into_iter()
            .filter_map(|instr| match instr {
                Instruction::Mul(x, y) => Some(x * y),
                _ => None,
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let instrs = parser(input);

    let mut result = 0;
    let mut enabled = true;
    for instr in instrs {
        match instr {
            Instruction::Do => enabled = true,
            Instruction::Dont => enabled = false,
            Instruction::Mul(x, y) => if enabled {
                result += x * y;
            }
        }
    }

    Some(result)
}

#[derive(Debug)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

fn parser(input: &str) -> Vec<Instruction> {
    Regex::new(r"(mul\(\d{1,3}\,\d{1,3}\))|(do\(\))|(don't\(\))")
        .ok()
        .unwrap()
        .find_iter(input)
        .map(|x| {
            let x = x.as_str();
            match x.get(..3).unwrap() {
                "mul" => {
                    let x = x.get(4..x.len() - 1).unwrap();
                    let (x, y) = x.split(",").collect_tuple().unwrap();
                    Instruction::Mul(x.parse().unwrap(), y.parse().unwrap())
                }
                "do(" => Instruction::Do,
                "don" => Instruction::Dont,
                _ => unreachable!(),
            }
        })
        .collect()
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
