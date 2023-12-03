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
        let has_adjacent_symbol = |offset_x: i32, offset_y: i32| {
            let left_out_of_bounds = pos.x as i32 + offset_x < 0;
            let top_out_of_bounds = pos.y as i32 + offset_y < 0;
            let right_out_of_bounds = pos.x as i32 + offset_x > self.rows[pos.y].len() as i32 - 1;
            let bottom_out_of_bounds = pos.y as i32 + offset_y > self.rows.len() as i32 - 1;

            if left_out_of_bounds
                || top_out_of_bounds
                || right_out_of_bounds
                || bottom_out_of_bounds
            {
                false
            } else {
                let x = (pos.x as i32 + offset_x) as usize;
                let y = (pos.y as i32 + offset_y) as usize;
                let c = self.rows[y][x];
                !c.is_ascii_digit() && c != '.'
            }
        };

        // top left
        has_adjacent_symbol(-1, -1)
            // top
            || has_adjacent_symbol(0, -1)
            // top right
            || has_adjacent_symbol(1, -1)
            // right
            || has_adjacent_symbol(1, 0)
            // bottom right
            || has_adjacent_symbol(1, 1)
            // bottom
            || has_adjacent_symbol(0, 1)
            // bottom left
            || has_adjacent_symbol(-1, 1)
            // left
            || has_adjacent_symbol(-1, 0)
    }

    fn next_pos_is_digit(&self, pos: Position) -> bool {
        matches!(self.rows[pos.y].get(pos.x + 1), Some(c) if c.is_ascii_digit())
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
