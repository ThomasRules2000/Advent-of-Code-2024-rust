#![feature(btree_cursors)]

use std::{collections::BTreeMap, ops::Bound};

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut m = parser(input);

    let mut pos = 0;

    loop {
        let c = m.upper_bound(Bound::Included(&pos));
        let Some((cur_pos, (cur_size, _))) = c.peek_prev() else {
            unreachable!()
        };
        let insertion_pos = cur_pos + cur_size;
        if pos < insertion_pos {
            pos = cur_pos + cur_size;
            continue;
        }
        let Some((next_pos, (next_size, _))) = c.peek_next() else {
            break;
        };
        let next_pos = *next_pos;

        pos = next_pos + next_size;

        let Some(mut last_entry) = m.last_entry() else {
            unreachable!()
        };
        let new_end = insertion_pos + last_entry.get().0;
        if new_end <= next_pos {
            pos = new_end;
            let last_size_num = last_entry.remove();
            m.insert(insertion_pos, last_size_num);
        } else {
            let insertion_size = next_pos - insertion_pos;

            let last_num = last_entry.get().1;
            last_entry.get_mut().0 -= insertion_size;
            m.insert(insertion_pos, (insertion_size, last_num));
        }
    }

    Some(checksum(m))
}

fn checksum(m: BTreeMap<u64, (u64, u64)>) -> u64 {
    m.into_iter()
        .skip(1)
        .map(|(pos, (size, val))| val * (((pos + size) * (pos + size - 1)) - (pos * (pos - 1))) / 2)
        .sum()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn parser(input: &str) -> BTreeMap<u64, (u64, u64)> {
    let mut block_no = 0;
    let mut pos = 0;
    let mut result = BTreeMap::new();
    let mut is_file = true;

    for num in input.trim().as_bytes().into_iter().map(|c| c - b'0') {
        if is_file {
            result.insert(pos, (num as u64, block_no));
            block_no += 1;
        }
        pos += num as u64;
        is_file = !is_file;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
