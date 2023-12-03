pub fn part2(input: &str) -> i32 {
    let rows = input.lines().map(|line| line.chars().collect()).collect();
    let grid = Grid::new(rows);
    let mut gear_ratio_total = 0;

    for (y, row) in grid.rows.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c != '*' {
                continue;
            }

            if let Some((n1, n2)) = grid.get_two_adjacent_part_nums(Position { y, x }) {
                gear_ratio_total += n1.value * n2.value;
            }
        }
    }

    gear_ratio_total
}

struct Grid {
    rows: Vec<Vec<char>>,
}

impl Grid {
    fn new(rows: Vec<Vec<char>>) -> Self {
        Grid { rows }
    }

    fn get_two_adjacent_part_nums(&self, pos: Position) -> Option<(GridNumber, GridNumber)> {
        let r = pos.y as i32;
        let c = pos.x as i32;

        let get_is_new_num = |r: i32, c: i32, nums: &Vec<GridNumber>| {
            let r = r as usize;
            let c = c as usize;
            for &num in nums.iter() {
                let is_same_row = r == num.row_index;
                let is_within_num_bounds = c >= num.start_index && c <= num.end_index;

                if is_same_row && is_within_num_bounds {
                    return false;
                }
            }

            true
        };

        let get_is_digit = |r: i32, c: i32| {
            let left_out_of_bounds = c < 0;
            let top_out_of_bounds = r < 0;

            if left_out_of_bounds || top_out_of_bounds {
                return false;
            }

            let r = r as usize;
            let c = c as usize;

            let right_out_of_bounds = c + 1 > self.rows[r].len() - 1;
            let bottom_out_of_bounds = r + 1 > self.rows.len();

            if right_out_of_bounds || bottom_out_of_bounds {
                return false;
            }

            self.rows[r][c].is_ascii_digit()
        };

        let top_left = (r - 1, c - 1);
        let top = (r - 1, c);
        let top_right = (r - 1, c + 1);
        let right = (r, c + 1);
        let bottom_right = (r + 1, c + 1);
        let bottom = (r + 1, c);
        let bottom_left = (r + 1, c - 1);
        let left = (r, c - 1);

        let nums = [
            top_left,
            top,
            top_right,
            right,
            bottom_right,
            bottom,
            bottom_left,
            left,
        ]
        .iter()
        .fold(Vec::with_capacity(3), |mut nums, (r, c)| {
            let r = *r;
            let c = *c;
            if get_is_digit(r, c) && get_is_new_num(r, c, &nums) {
                nums.push(self.get_grid_num(Position::new(c as usize, r as usize)));
            }

            nums
        });

        if nums.len() == 2 {
            let mut it = nums.into_iter();
            Some((it.next().unwrap(), it.next().unwrap()))
        } else {
            None
        }
    }

    fn get_grid_num(&self, pos: Position) -> GridNumber {
        let r = pos.y;
        let mut start = pos.x;
        while start > 0 && self.rows[r][start - 1].is_ascii_digit() {
            start -= 1;
        }

        let mut end = pos.x;
        while end < self.rows[r].len() - 1 && self.rows[r][end + 1].is_ascii_digit() {
            end += 1;
        }

        let mut value = 0;
        for i in start..=end {
            value = value * 10 + self.rows[r][i].to_digit(10).unwrap() as i32;
        }

        GridNumber::new(r, start, end, value)
    }
}

#[derive(Clone, Copy, Debug)]
struct GridNumber {
    value: i32,
    row_index: usize,
    start_index: usize,
    end_index: usize,
}

impl GridNumber {
    fn new(row_index: usize, start_index: usize, end_index: usize, value: i32) -> Self {
        Self {
            value,
            row_index,
            start_index,
            end_index,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_example() {
        let input = include_str!("../example.txt");
        let output = part2(input);
        let expected = 467835;
        assert_eq!(output, expected);
    }

    #[test]
    fn part2_input() {
        let input = include_str!("../input.txt");
        let output = part2(input);
        let expected = 81709807;
        assert_eq!(output, expected);
    }
}
