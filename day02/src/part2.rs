use std::cmp;

#[derive(Debug)]
pub struct Game {
    pub power: i32,
}

#[derive(Debug)]
struct ColorMaxes {
    blue: i32,
    red: i32,
    green: i32,
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
        let rounds = line.split(':').last().unwrap();
        let ColorMaxes { blue, red, green } = Game::get_color_maxes(rounds);
        Self {
            power: blue * red * green,
        }
    }

    fn get_color_maxes(rounds: &str) -> ColorMaxes {
        let (mut blue, mut red, mut green) = (0, 0, 0);

        rounds.split(';').for_each(|round| {
            round.split(',').map(Color::new).for_each(|c| match c {
                Color::Blue(n) => blue = cmp::max(blue, n),
                Color::Red(n) => red = cmp::max(red, n),
                Color::Green(n) => green = cmp::max(green, n),
            })
        });

        ColorMaxes { blue, red, green }
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
