use std::cmp;

#[derive(Debug)]
pub struct Game {
    pub power: i32,
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
}

impl Game {
    pub fn new(line: &str) -> Self {
        let (mut max_blue, mut max_red, mut max_green) = (0, 0, 0);

        let rounds = line.split(':').last().unwrap();
        rounds.split(';').for_each(|round| {
            round.split(',').map(Color::new).for_each(|c| match c {
                Color::Blue(n) => max_blue = cmp::max(max_blue, n),
                Color::Red(n) => max_red = cmp::max(max_red, n),
                Color::Green(n) => max_green = cmp::max(max_green, n),
            })
        });

        Self {
            power: max_blue * max_red * max_green,
        }
    }
}

pub fn part2(input: &str) -> i32 {
    input.lines().map(Game::new).map(|game| game.power).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_example() {
        let input = include_str!("../example.txt");
        let output = part2(input);
        let expected = 2286;
        assert_eq!(output, expected);
    }

    #[test]
    fn part2_input() {
        let input = include_str!("../input.txt");
        let output = part2(input);
        let expected = 67363;
        assert_eq!(output, expected);
    }
}
