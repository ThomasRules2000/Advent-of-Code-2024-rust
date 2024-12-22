use std::collections::HashMap;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let m = parser(input);

    Some(
        m.iter()
            .filter(|(_, n)| **n == 0)
            .map(|(pos, _)| get_trails(&m, *pos).len() as u32)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let m = parser(input);

    Some(
        m.iter()
            .filter(|(_, n)| **n == 0)
            .map(|(pos, _)| get_trails(&m, *pos).values().sum::<u32>())
            .sum(),
    )
}

fn parser(input: &str) -> HashMap<(i32, i32), u8> {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.as_bytes()
                .into_iter()
                .enumerate()
                .map(move |(j, c)| ((i as i32, j as i32), c - b'0'))
        })
        .flatten()
        .collect()
}

fn get_trails(m: &HashMap<(i32, i32), u8>, start_pos: (i32, i32)) -> HashMap<(i32, i32), u32> {
    let mut frontier = HashMap::new();
    frontier.insert(start_pos, 1);

    for n in 1..10 {
        frontier = frontier
            .into_iter()
            .map(|((x, y), score)| {
                [(x - 1, y), (x, y - 1), (x, y + 1), (x + 1, y)]
                    .into_iter()
                    .filter(|pos| m.get(pos) == Some(&n))
                    .map(move |pos| (pos, score))
            })
            .flatten()
            .fold(
                HashMap::new(),
                |mut new_frontier: HashMap<(i32, i32), u32>, (k, v)| {
                    match new_frontier.get_mut(&k) {
                        Some(old_v) => {
                            *old_v += v;
                        }
                        None => {
                            new_frontier.insert(k, v);
                        }
                    }
                    new_frontier
                },
            )
    }
    frontier
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
