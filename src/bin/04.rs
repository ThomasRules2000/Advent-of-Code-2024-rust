use itertools::Itertools;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let chars = parser(input);

    let nrows = chars.len();
    let ncols = chars[0].len();

    let mut count = 0;
    for i in 0..nrows {
        for j in 0..ncols {
            if i < nrows - 3 {
                let col: &[u8; 4] = &chars[i..i + 4]
                    .into_iter()
                    .map(|row| row[j])
                    .collect_tuple::<(u8, u8, u8, u8)>().unwrap().into();
                count += is_xmas(col) as u32;
            }

            if j < ncols - 3 {
                let row = &chars[i][j..j + 4];
                count += is_xmas(row) as u32;

                if i >= 3 {
                    let fw_diag: &[u8; 4] = &chars[i - 3..i + 1]
                        .into_iter()
                        .rev()
                        .enumerate()
                        .map(|(n, row)| row[j + n])
                        .collect_tuple::<(u8, u8, u8, u8)>().unwrap().into();
                    count += is_xmas(fw_diag) as u32;
                }

                if i < nrows - 3 {
                    let bw_diag: &[u8; 4] = &chars[i..i + 4]
                        .into_iter()
                        .enumerate()
                        .map(|(n, row)| row[j + n])
                        .collect_tuple::<(u8, u8, u8, u8)>().unwrap().into();
                    count += is_xmas(bw_diag) as u32;
                }
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let chars = parser(input);

    let nrows = chars.len();
    let ncols = chars[0].len();

    let mut count = 0;
    for i in 1..nrows - 1 {
        for j in 1..ncols - 1 {
            let c_valid = chars[i][j] == b'A';

            let tl = chars[i - 1][j - 1];
            let tr = chars[i + 1][j - 1];
            let bl = chars[i - 1][j + 1];
            let br = chars[i + 1][j + 1];

            let fw = [tl, br];
            let fw_valid = fw == "MS".as_bytes() || fw == "SM".as_bytes();

            let bw = [tr, bl];
            let bw_valid = bw == "MS".as_bytes() || bw == "SM".as_bytes();

            count += (c_valid && fw_valid && bw_valid) as u32;
        }
    }

    Some(count)
}

fn parser(input: &str) -> Vec<&[u8]> {
    input.lines().map(|s| s.as_bytes()).collect()
}

fn is_xmas(input: &[u8]) -> bool {
    input == "XMAS".as_bytes() || input == "SAMX".as_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
