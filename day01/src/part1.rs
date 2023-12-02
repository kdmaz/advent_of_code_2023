pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let first = chars.find_map(|c| c.to_digit(10)).unwrap();
            let last = chars.rev().find_map(|c| c.to_digit(10)).unwrap_or(first);
            first * 10 + last
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
        let expected = 142;
        assert_eq!(output, expected);
    }

    #[test]
    fn part1_input() {
        let input = include_str!("../input.txt");
        let output = part1(input);
        let expected = 54667;
        assert_eq!(output, expected);
    }
}
