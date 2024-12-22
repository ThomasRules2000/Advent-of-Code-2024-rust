use std::collections::HashSet;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let ((mut x, mut y), walls, (max_x, max_y)) = parser(input);

    let mut facing = Facing::North;
    let mut visited = HashSet::new();

    while x >= 0 && y >= 0 && x < max_x && y < max_y {
        visited.insert((x, y));
        ((x, y), facing) = do_move(&walls, (x, y), facing);
    }

    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn check_cycle(
    walls: &HashSet<(i32, i32)>,
    (mut x, mut y): (i32, i32),
    mut facing: Facing,
    (max_x, max_y): (i32, i32),
) -> bool {


    // let mut visited = HashSet::new();

    // while x >= 0 && y >= 0 && x < max_x && y < max_y {
    //     if visited.contains(&((x, y), facing)) {
    //         return true;
    //     }
    //     visited.insert(((x, y), facing));
    // }

    false
}

fn parser(input: &str) -> ((i32, i32), HashSet<(i32, i32)>, (i32, i32)) {
    let lines: Vec<_> = input.lines().collect();
    let (walls, guard): (Vec<Vec<_>>, Vec<Vec<_>>) = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.as_bytes()
                .into_iter()
                .enumerate()
                .filter(|(_, c)| **c != b'.')
                .map(|(j, c)| ((i, j), c))
                .partition(|(_, c)| **c == b'#')
        })
        .unzip();

    let walls = walls
        .into_iter()
        .flatten()
        .map(|((x, y), _)| (x as i32, y as i32))
        .collect();

    let (x, y) = guard.into_iter().flatten().next().unwrap().0;

    (
        (x as i32, y as i32),
        walls,
        (lines.len() as i32, lines[0].len() as i32),
    )
}

fn next_pos((x, y): (i32, i32), face: &Facing) -> (i32, i32) {
    match face {
        Facing::North => (x - 1, y),
        Facing::South => (x + 1, y),
        Facing::East => (x, y + 1),
        Facing::West => (x, y - 1),
    }
}

fn do_move(walls: &HashSet<(i32, i32)>, pos: (i32, i32), face: Facing) -> ((i32, i32), Facing) {
    let new_pos = next_pos(pos, &face);
    if walls.contains(&new_pos) {
        (pos, turn_right(&face))
    } else {
        (new_pos, face)
    }
}

#[derive(PartialEq, Eq, Hash)]
enum Facing {
    North,
    East,
    South,
    West,
}

fn turn_right(face: &Facing) -> Facing {
    match face {
        Facing::North => Facing::East,
        Facing::East => Facing::South,
        Facing::South => Facing::West,
        Facing::West => Facing::North,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
