const MAX_RED_CUBES: i32 = 12;
const MAX_GREEN_CUBES: i32 = 13;
const MAX_BLUE_CUBES: i32 = 14;

pub fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(Game::new)
        .filter(|game| game.is_valid)
        .map(|game| game.id)
        .sum()
}

struct Game {
    id: i32,
    is_valid: bool,
}

impl Game {
    pub fn new(line: &str) -> Self {
        let mut game_parts = line.split(':');

        Self {
            id: Game::parse_id(game_parts.next().unwrap()),
            is_valid: Game::get_is_valid(game_parts.next().unwrap()),
        }
    }

    fn parse_id(id_raw: &str) -> i32 {
        id_raw.split(' ').last().unwrap().parse::<i32>().unwrap()
    }

    fn get_is_valid(rounds: &str) -> bool {
        rounds
            .split(';')
            .all(|round| round.split(',').map(Color::new).all(|c| c.get_is_valid()))
    }
}

enum Color {
    Blue { num_seen: i32 },
    Red { num_seen: i32 },
    Green { num_seen: i32 },
}

impl Color {
    pub fn new(draw: &str) -> Self {
        let mut draw_parts = draw.trim().split(' ');
        let num_seen = draw_parts.next().unwrap().parse().unwrap();
        let color = draw_parts.next().unwrap();

        match color {
            "blue" => Self::Blue { num_seen },
            "red" => Self::Red { num_seen },
            "green" => Self::Green { num_seen },
            _ => unreachable!(),
        }
    }

    pub fn get_is_valid(&self) -> bool {
        match self {
            Color::Blue { num_seen } => *num_seen <= MAX_BLUE_CUBES,
            Color::Red { num_seen } => *num_seen <= MAX_RED_CUBES,
            Color::Green { num_seen } => *num_seen <= MAX_GREEN_CUBES,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../example.txt");
        let output = part1(input);
        let expected = 8;
        assert_eq!(output, expected);
    }

    #[test]
    fn part1_input() {
        let input = include_str!("../input.txt");
        let output = part1(input);
        let expected = 2528;
        assert_eq!(output, expected);
    }
}
