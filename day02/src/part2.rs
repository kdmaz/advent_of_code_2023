use std::cmp;

pub fn part2(input: &str) -> i32 {
    input.lines().map(Game::new).map(|game| game.power).sum()
}

struct Game {
    power: i32,
}

impl Game {
    pub fn new(line: &str) -> Self {
        let (mut max_blue, mut max_red, mut max_green) = (0, 0, 0);

        let rounds = line.split(':').last().unwrap();
        rounds.split(';').for_each(|round| {
            round.split(',').map(Color::new).for_each(|c| match c {
                Color::Blue { num_seen } => max_blue = cmp::max(max_blue, num_seen),
                Color::Red { num_seen } => max_red = cmp::max(max_red, num_seen),
                Color::Green { num_seen } => max_green = cmp::max(max_green, num_seen),
            })
        });

        Self {
            power: max_blue * max_red * max_green,
        }
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
