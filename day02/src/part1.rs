const MAX_RED_CUBES: i32 = 12;
const MAX_GREEN_CUBES: i32 = 13;
const MAX_BLUE_CUBES: i32 = 14;

#[derive(Debug)]
pub struct Game {
    pub id: i32,
    pub is_possible: bool,
}

pub enum Color {
    Blue(i32),
    Red(i32),
    Green(i32),
}

impl Color {
    pub fn new(text: &str) -> Self {
        let mut split = text.trim().split(' ');
        let num = split.next().unwrap().parse().unwrap();
        let color = split.next().unwrap();

        match color {
            "blue" => Self::Blue(num),
            "red" => Self::Red(num),
            "green" => Self::Green(num),
            _ => unreachable!(),
        }
    }

    pub fn get_is_possible(&self) -> bool {
        match self {
            Color::Blue(n) => *n <= MAX_BLUE_CUBES,
            Color::Red(n) => *n <= MAX_RED_CUBES,
            Color::Green(n) => *n <= MAX_GREEN_CUBES,
        }
    }
}

impl Game {
    pub fn new(line: &str) -> Self {
        let mut split = line.split(':');

        Self {
            id: Game::parse_id(split.next().unwrap()),
            is_possible: Game::get_is_possible(split.next().unwrap()),
        }
    }

    fn parse_id(text: &str) -> i32 {
        text.split(' ').last().unwrap().parse::<i32>().unwrap()
    }

    fn get_is_possible(rounds: &str) -> bool {
        rounds.split(';').all(|round| {
            round
                .split(',')
                .map(Color::new)
                .all(|c| c.get_is_possible())
        })
    }
}

pub fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(Game::new)
        .filter(|game| game.is_possible)
        .map(|game| game.id)
        .sum()
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
