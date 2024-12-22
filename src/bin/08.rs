use std::collections::{HashMap, HashSet};

use gcd::Gcd;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let (m, bounds) = parser(input);

    let antinodes: HashSet<(i32, i32)> = m
        .into_iter()
        .map(|nodes| calc_antinodes(nodes, &bounds))
        .flatten()
        .collect();

    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (m, bounds) = parser(input);

    let antinodes: HashSet<_> = m
        .into_iter()
        .map(|nodes| calc_antinodes_2(nodes, &bounds))
        .flatten()
        .collect();

    Some(antinodes.len() as u32)
}

fn parser(input: &str) -> (Vec<Vec<(i32, i32)>>, (i32, i32)) {
    let lines: Vec<_> = input.lines().collect();
    let max_x = lines.len();
    let max_y = lines[0].len();

    let mut m: HashMap<u8, Vec<(i32, i32)>> = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.as_bytes().into_iter().enumerate() {
            if *c == b'.' {
                continue;
            }
            if m.contains_key(c) {
                m.get_mut(c).unwrap().push((i as i32, j as i32));
            } else {
                m.insert(*c, vec![(i as i32, j as i32)]);
            }
        }
    }

    (m.into_values().collect(), (max_x as i32, max_y as i32))
}

fn calc_antinodes(nodes: Vec<(i32, i32)>, (max_x, max_y): &(i32, i32)) -> HashSet<(i32, i32)> {
    let mut result: HashSet<(i32, i32)> = HashSet::new();

    for i in 0..nodes.len() {
        let (x1, y1) = nodes[i];
        for j in i + 1..nodes.len() {
            let (x2, y2) = nodes[j];
            let diff_x = x1 - x2;
            let diff_y = y1 - y2;

            result.extend(
                [(x1 + diff_x, y1 + diff_y), (x2 - diff_x, y2 - diff_y)]
                    .into_iter()
                    .filter(|(x, y)| x >= &0 && y >= &0 && x < max_x && y < max_y),
            );
        }
    }

    result
}

fn calc_antinodes_2(nodes: Vec<(i32, i32)>, (max_x, max_y): &(i32, i32)) -> HashSet<(i32, i32)> {
    let mut result: HashSet<(i32, i32)> = HashSet::new();

    for i in 0..nodes.len() {
        let (x1, y1) = nodes[i];
        for j in i + 1..nodes.len() {
            let (x2, y2) = nodes[j];
            let raw_diff_x = x1 - x2;
            let raw_diff_y = y1 - y2;

            let gcd = (raw_diff_x.abs() as u32).gcd(raw_diff_y.abs() as u32) as i32;

            let diff_x = raw_diff_x / gcd;
            let diff_y = raw_diff_y / gcd;

            let mul_max = *max_x.max(max_y);

            for mul in -mul_max..mul_max {
                let x = x1 + (mul * diff_x);
                let y = y1 + (mul * diff_y);

                if x >= 0 && y >= 0 && &x < max_x && &y < max_y {
                    result.insert((x, y));
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
