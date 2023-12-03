pub fn part2(input: &str) -> i32 {
    let rows = input.lines().map(|line| line.chars().collect()).collect();
    let grid = Grid::new(rows);
    let mut gear_ratio_total = 0;

    for (y, row) in grid.rows.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c != '*' {
                continue;
            }

            if let Some((n1, n2)) = grid.get_two_adjacent_part_nums(Position { x, y }) {
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
        let get_is_new_num = |pos: Position, nums: &Vec<GridNumber>| {
            for &num in nums.iter() {
                let is_same_row = pos.y == num.y;
                let is_within_num_bounds = pos.x >= num.start && pos.x <= num.end;

                if is_same_row && is_within_num_bounds {
                    return false;
                }
            }

            true
        };

        let get_is_digit = |x: i32, y: i32| {
            let left_out_of_bounds = x < 0;
            let top_out_of_bounds = y < 0;

            if left_out_of_bounds || top_out_of_bounds {
                return false;
            }

            let pos = Position::new(x as usize, y as usize);

            let right_out_of_bounds = pos.x + 1 > self.rows[pos.y].len() - 1;
            let bottom_out_of_bounds = pos.y + 1 > self.rows.len();

            if right_out_of_bounds || bottom_out_of_bounds {
                return false;
            }

            self.rows[pos.y][pos.x].is_ascii_digit()
        };

        let offsets = [
            (-1, -1), // top left
            (0, -1),  // top
            (1, -1),  // top right
            (1, 0),   // right
            (1, 1),   // bottom right
            (0, 1),   // bottom
            (-1, 1),  // bottom left
            (-1, 0),  // left
        ];

        let nums = offsets
            .iter()
            .fold(Vec::with_capacity(3), |mut nums, (x_offset, y_offset)| {
                let x = pos.x as i32 + x_offset;
                let y = pos.y as i32 + y_offset;

                if get_is_digit(x, y)
                    && get_is_new_num(Position::new(x as usize, y as usize), &nums)
                {
                    nums.push(self.get_grid_num(Position::new(x as usize, y as usize)));
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
        let mut start = pos.x;
        while start > 0 && self.rows[pos.y][start - 1].is_ascii_digit() {
            start -= 1;
        }

        let mut end = pos.x;
        while end < self.rows[pos.y].len() - 1 && self.rows[pos.y][end + 1].is_ascii_digit() {
            end += 1;
        }

        let mut value = 0;
        for x in start..=end {
            value = value * 10 + self.rows[pos.y][x].to_digit(10).unwrap() as i32;
        }

        GridNumber::new(value, pos.y, start, end)
    }
}

#[derive(Clone, Copy, Debug)]
struct GridNumber {
    value: i32,
    y: usize,
    start: usize,
    end: usize,
}

impl GridNumber {
    fn new(value: i32, y: usize, start: usize, end: usize) -> Self {
        Self {
            value,
            y,
            start,
            end,
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
