use std::collections::HashSet;

pub fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let line_parts = line.split(':');
            let mut card_parts = line_parts.last().unwrap().split('|');
            let winning_nums = HashSet::<i32>::from_iter(
                card_parts
                    .next()
                    .unwrap()
                    .split(' ')
                    .filter_map(|s| s.parse::<i32>().ok()),
            );

            card_parts
                .next()
                .unwrap()
                .split(' ')
                .filter_map(|s| s.parse::<i32>().ok())
                .filter(|n| winning_nums.contains(n))
                .fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../example.txt");
        let output = part1(input);
        let expected = 13;
        assert_eq!(output, expected);
    }

    #[test]
    fn part1_input() {
        let input = include_str!("../input.txt");
        let output = part1(input);
        let expected = 24706;
        assert_eq!(output, expected);
    }
}
