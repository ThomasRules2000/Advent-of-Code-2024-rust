use ndarray::Array2;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let chars = parser(input);

    let nrows = chars.nrows();
    let ncols = chars.ncols();

    let mut count = 0;
    for i in 0..nrows {
        for j in 0..ncols {
            if i < nrows - 3 {
                let col = [
                    chars[(i, j)],
                    chars[(i + 1, j)],
                    chars[(i + 2, j)],
                    chars[(i + 3, j)],
                ];
                count += is_xmas(&col) as u32;
            }

            if j < ncols - 3 {
                let row = [
                    chars[(i, j)],
                    chars[(i, j + 1)],
                    chars[(i, j + 2)],
                    chars[(i, j + 3)],
                ];
                count += is_xmas(&row) as u32;

                if i >= 3 {
                    let fw_diag = [
                        chars[(i, j)],
                        chars[(i - 1, j + 1)],
                        chars[(i - 2, j + 2)],
                        chars[(i - 3, j + 3)],
                    ];
                    count += is_xmas(&fw_diag) as u32;
                }

                if i < nrows - 3 {
                    let bw_diag = [
                        chars[(i, j)],
                        chars[(i + 1, j + 1)],
                        chars[(i + 2, j + 2)],
                        chars[(i + 3, j + 3)],
                    ];
                    count += is_xmas(&bw_diag) as u32;
                }
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        parser(input)
            .windows((3, 3))
            .into_iter()
            .filter(|w| {
                let c_valid = w[(1, 1)] == b'A';

                let tl = w[(0, 0)];
                let tr = w[(2, 0)];
                let bl = w[(0, 2)];
                let br = w[(2, 2)];

                let fw = [tl, br];
                let fw_valid = fw == "MS".as_bytes() || fw == "SM".as_bytes();

                let bw = [tr, bl];
                let bw_valid = bw == "MS".as_bytes() || bw == "SM".as_bytes();

                c_valid && fw_valid && bw_valid
            })
            .count() as u32,
    )
}

fn parser(input: &str) -> Array2<u8> {
    let lines: Vec<_> = input.lines().collect();
    Array2::from_shape_vec(
        (lines.len(), lines[0].len()),
        lines
            .into_iter()
            .map(|s| s.as_bytes().into_iter().cloned())
            .flatten()
            .collect(),
    )
    .unwrap()
}

fn is_xmas(input: &[u8]) -> bool {
    input == b"XMAS" || input == b"SAMX"
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
