pub fn part1(input: &str) -> i32 {
    let rows = input.lines().map(|line| line.chars().collect()).collect();
    let grid = Grid::new(rows);
    let mut part_num_total = 0;

    for (y, row) in grid.rows.iter().enumerate() {
        let mut num = GridNumber::new();

        for (x, &c) in row.iter().enumerate() {
            if !c.is_ascii_digit() {
                continue;
            }
            if let Some(n) = c.to_digit(10) {
                num.add_digit(n as i32);
            } else {
                continue;
            }

            let char_pos = Position::new(x, y);

            if !num.is_part_number && grid.has_adjacent_symbol(char_pos) {
                num.is_part_number = true;
            }

            if grid.next_pos_is_digit(char_pos) {
                continue;
            }

            if num.is_part_number {
                part_num_total += num.value;
            }

            num = GridNumber::new();
        }
    }

    part_num_total
}

struct Grid {
    rows: Vec<Vec<char>>,
}

impl Grid {
    fn new(rows: Vec<Vec<char>>) -> Self {
        Grid { rows }
    }

    fn has_adjacent_symbol(&self, pos: Position) -> bool {
        let r = pos.y;
        let c = pos.x;

        let left_out_of_bounds = c == 0;
        let top_out_of_bounds = r == 0;
        let right_out_of_bounds = c + 1 > self.rows[r].len() - 1;
        let bottom_out_of_bounds = r + 1 > self.rows.len() - 1;

        let is_symbol = |char: char| !char.is_ascii_digit() && char != '.';

        // top left
        (!left_out_of_bounds && !top_out_of_bounds && is_symbol(self.rows[r - 1][c - 1]))
            // top
            || (!top_out_of_bounds && is_symbol(self.rows[r - 1][c]))
            // top right
            || (!right_out_of_bounds && !top_out_of_bounds && is_symbol(self.rows[r - 1][c + 1]))
            // right
            || (!right_out_of_bounds && is_symbol(self.rows[r][c + 1]))
            // bottom right
            || (!bottom_out_of_bounds
                && !right_out_of_bounds
                && is_symbol(self.rows[r + 1][c + 1]))
            // bottom
            || (!bottom_out_of_bounds && is_symbol(self.rows[r + 1][c]))
            // bottom left
            || (!bottom_out_of_bounds && !left_out_of_bounds && is_symbol(self.rows[r + 1][c - 1]))
            // left
            || (!left_out_of_bounds && is_symbol(self.rows[r][c - 1]))
    }

    fn next_pos_is_digit(&self, pos: Position) -> bool {
        let next_pos = Position::new(pos.x + 1, pos.y);
        if next_pos.x > self.rows.len() - 1 {
            false
        } else {
            let c = self.rows[next_pos.y][next_pos.x];
            c.is_ascii_digit()
        }
    }
}

struct GridNumber {
    value: i32,
    is_part_number: bool,
}

impl GridNumber {
    fn new() -> Self {
        Self {
            value: 0,
            is_part_number: false,
        }
    }

    fn add_digit(&mut self, num: i32) {
        self.value = self.value * 10 + num;
    }
}

#[derive(Clone, Copy, Debug)]
struct Position {
    y: usize,
    x: usize,
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
    fn part1_example() {
        let input = include_str!("../example.txt");
        let output = part1(input);
        let expected = 4361;
        assert_eq!(output, expected);
    }

    #[test]
    fn part1_input() {
        let input = include_str!("../input.txt");
        let output = part1(input);
        let expected = 538046;
        assert_eq!(output, expected);
    }
}
