use std::{cell::RefCell, collections::HashSet};

pub fn part1(input: &str) -> i32 {
    let grid = input.lines().map(|line| line.chars().collect()).collect();
    let system = System::new(grid);
    let mut part_num_total = 0;

    for (y, row) in system.grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c.is_ascii_digit() || c == '.' {
                continue;
            }

            let symbol = Position::new(x, y);
            part_num_total += system.collect_part_nums(symbol);
        }
    }

    part_num_total
}

struct System {
    grid: Vec<Vec<char>>,
    part_num_locations: RefCell<HashSet<Position>>,
}

impl System {
    fn new(grid: Vec<Vec<char>>) -> Self {
        System {
            grid,
            part_num_locations: RefCell::new(HashSet::new()),
        }
    }

    fn collect_part_nums(&self, pos: Position) -> i32 {
        let get_adjacent_part_num = |offset_x: i32, offset_y: i32| {
            let left_out_of_bounds = pos.x as i32 + offset_x < 0;
            let top_out_of_bounds = pos.y as i32 + offset_y < 0;
            let right_out_of_bounds = pos.x as i32 + offset_x > self.grid[pos.y].len() as i32 - 1;
            let bottom_out_of_bounds = pos.y as i32 + offset_y > self.grid.len() as i32 - 1;

            if left_out_of_bounds
                || top_out_of_bounds
                || right_out_of_bounds
                || bottom_out_of_bounds
            {
                0
            } else {
                let x = (pos.x as i32 + offset_x) as usize;
                let y = (pos.y as i32 + offset_y) as usize;
                let pos = Position::new(x, y);
                let c = self.grid[pos.y][pos.x];
                if c.is_ascii_digit() && !self.part_num_locations.borrow().contains(&pos) {
                    self.build_part_number(pos)
                } else {
                    0
                }
            }
        };

        let top_left = get_adjacent_part_num(-1, -1);
        let top = get_adjacent_part_num(0, -1);
        let top_right = get_adjacent_part_num(1, -1);
        let right = get_adjacent_part_num(1, 0);
        let bottom_right = get_adjacent_part_num(1, 1);
        let bottom = get_adjacent_part_num(0, 1);
        let bottom_left = get_adjacent_part_num(-1, 1);
        let left = get_adjacent_part_num(-1, 0);

        top_left + top + top_right + right + bottom_right + bottom + bottom_left + left
    }

    fn build_part_number(&self, pos: Position) -> i32 {
        let mut start = pos.x;
        while start > 0 && self.grid[pos.y][start - 1].is_ascii_digit() {
            start -= 1;
        }

        let mut end = pos.x;
        while end < self.grid[pos.y].len() - 1 && self.grid[pos.y][end + 1].is_ascii_digit() {
            end += 1;
        }

        let mut value = 0;
        for x in start..=end {
            self.update_part_num_location(Position::new(x, pos.y));
            value = value * 10 + self.grid[pos.y][x].to_digit(10).unwrap() as i32;
        }

        value
    }

    fn update_part_num_location(&self, pos: Position) {
        self.part_num_locations.borrow_mut().insert(pos);
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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
