use nom::{
    character::complete::{line_ending, none_of},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};
use std::{
    cell::{Cell, RefCell},
    cmp,
    collections::HashMap,
};

pub fn part1(input: &str) -> u32 {
    maze_parser(input).traverse().get_max_steps()
}

fn maze_parser(input: &str) -> Maze {
    let (input, grid) = grid_parser(input).unwrap();
    debug_assert_eq!(input, "");
    let start_position = find_start_position(&grid);
    Maze::new(grid, start_position)
}

fn grid_parser(input: &str) -> IResult<&str, Grid> {
    let non_line_ending = none_of("\r\n");
    let to_pipe = |c| Pipe::try_from(c).unwrap();
    let pipe_parser = map(non_line_ending, to_pipe);
    separated_list1(line_ending, many1(pipe_parser))(input)
}

fn find_start_position(grid: &Grid) -> Position {
    grid.iter()
        .enumerate()
        .find_map(|(y, pipes)| {
            pipes.iter().enumerate().find_map(|(x, pipe)| {
                if *pipe == Pipe::Start {
                    Some(Position::new(x, y))
                } else {
                    None
                }
            })
        })
        .unwrap()
}

#[derive(Debug)]
struct Maze {
    grid: Grid,
    start_position: Position,
    visited: RefCell<HashMap<Position, u32>>,
    max_steps: Cell<u32>,
}

impl Maze {
    fn new(grid: Grid, start_position: Position) -> Self {
        Self {
            grid,
            start_position,
            visited: RefCell::new(HashMap::new()),
            max_steps: Cell::new(0),
        }
    }

    fn traverse(self) -> Self {
        let start = self.start_position;
        let mut next_moves = vec![Move::new(start, start, 0)];

        while !next_moves.is_empty() {
            next_moves = self.get_next_moves(next_moves);
        }

        self
    }

    fn get_next_moves(&self, next_moves: Vec<Move>) -> Vec<Move> {
        next_moves
            .into_iter()
            .fold(vec![], |mut next_moves, r#move| {
                let Move {
                    current_position,
                    prev_position,
                    steps,
                } = r#move;
                self.update_visited(current_position, steps);
                let current_pipe = self.get_pipe_at(current_position);

                for direction in current_pipe.get_outbound_directions() {
                    if let Some(next_position) = self.try_next_position(current_position, direction)
                    {
                        let is_prev = next_position == prev_position;
                        let has_visit = self.get_has_visit(next_position, steps);
                        let is_next_pipe_connected = self
                            .get_pipe_at(next_position)
                            .get_is_connected_from(direction);

                        if !is_prev && !has_visit && is_next_pipe_connected {
                            next_moves.push(Move::new(next_position, current_position, steps + 1));
                        }
                    }
                }
                next_moves
            })
    }

    fn try_next_position(&self, position: Position, direction: Direction) -> Option<Position> {
        match direction {
            Direction::North => {
                let on_upper_edge = position.y == 0;
                if on_upper_edge {
                    None
                } else {
                    Some(Position::new(position.x, position.y - 1))
                }
            }
            Direction::East => {
                let on_right_edge = position.x == self.grid[0].len() - 1;
                if on_right_edge {
                    None
                } else {
                    Some(Position::new(position.x + 1, position.y))
                }
            }
            Direction::South => {
                let on_bottom_edge = position.y == self.grid.len() - 1;
                if on_bottom_edge {
                    None
                } else {
                    Some(Position::new(position.x, position.y + 1))
                }
            }
            Direction::West => {
                let on_left_edge = position.x == 0;
                if on_left_edge {
                    None
                } else {
                    Some(Position::new(position.x - 1, position.y))
                }
            }
        }
    }

    fn get_pipe_at(&self, position: Position) -> Pipe {
        self.grid[position.y][position.x]
    }

    fn update_visited(&self, position: Position, steps: u32) {
        let mut visited = self.visited.borrow_mut();
        let current_steps = visited.entry(position).or_insert(steps);
        *current_steps = cmp::min(*current_steps, steps);
        self.max_steps.replace(*current_steps);
    }

    fn get_has_visit(&self, position: Position, steps: u32) -> bool {
        self.visited
            .borrow()
            .get(&position)
            .map(|&current_steps| current_steps < steps)
            .unwrap_or(false)
    }

    fn get_max_steps(&self) -> u32 {
        self.max_steps.get()
    }
}

type Grid = Vec<Vec<Pipe>>;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pipe {
    Vertical,
    Horizontal,
    BottomLeftBend,
    BottomRightBend,
    TopRightBend,
    TopLeftBend,
    Ground,
    Start,
}

impl TryFrom<char> for Pipe {
    type Error = String;

    fn try_from(pipe: char) -> Result<Self, Self::Error> {
        Ok(match pipe {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::BottomLeftBend,
            'J' => Self::BottomRightBend,
            '7' => Self::TopRightBend,
            'F' => Self::TopLeftBend,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => return Err(format!("Could not convert symbol \"{pipe}\" into Pipe")),
        })
    }
}

impl Pipe {
    fn get_outbound_directions(&self) -> Vec<Direction> {
        use Direction::*;
        match self {
            Pipe::Vertical => vec![North, South],
            Pipe::Horizontal => vec![West, East],
            Pipe::BottomLeftBend => vec![North, East],
            Pipe::BottomRightBend => vec![North, West],
            Pipe::TopRightBend => vec![South, West],
            Pipe::TopLeftBend => vec![East, South],
            Pipe::Ground => vec![],
            Pipe::Start => vec![North, South, East, West],
        }
    }

    fn get_is_connected_from(&self, direction: Direction) -> bool {
        use Direction::*;
        match self {
            Pipe::Vertical => direction == North || direction == South,
            Pipe::Horizontal => direction == East || direction == West,
            Pipe::BottomLeftBend => direction == South || direction == West,
            Pipe::BottomRightBend => direction == South || direction == East,
            Pipe::TopRightBend => direction == North || direction == East,
            Pipe::TopLeftBend => direction == North || direction == West,
            Pipe::Ground | Pipe::Start => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
struct Move {
    current_position: Position,
    prev_position: Position,
    steps: u32,
}

impl Move {
    fn new(current_position: Position, prev_position: Position, steps: u32) -> Self {
        Self {
            current_position,
            prev_position,
            steps,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = include_str!("../example.txt");
        let output = part1(input);
        let expected = 4;
        assert_eq!(output, expected);
    }

    #[test]
    fn part1_example2() {
        let input = include_str!("../example2.txt");
        let output = part1(input);
        let expected = 8;
        assert_eq!(output, expected);
    }

    #[test]
    fn part1_input() {
        let input = include_str!("../input.txt");
        let output = part1(input);
        let expected = 6860;
        assert_eq!(output, expected);
    }
}
