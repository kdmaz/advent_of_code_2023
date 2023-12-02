pub fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            let first = chars.iter().find(|c| c.is_ascii_digit()).unwrap();
            let last = chars.iter().rev().find(|c| c.is_ascii_digit()).unwrap();
            format!("{}{}", first, last).parse::<i32>().unwrap()
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
