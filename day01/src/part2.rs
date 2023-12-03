pub fn part2(input: &str) -> u32 {
    input.lines().map(get_num_from_line).sum()
}

fn get_num_from_line(line: &str) -> u32 {
    let mut it = (0..line.len()).filter_map(|index| {
        let line = &line[index..];

        if line.starts_with("one") {
            Some(1)
        } else if line.starts_with("two") {
            Some(2)
        } else if line.starts_with("three") {
            Some(3)
        } else if line.starts_with("four") {
            Some(4)
        } else if line.starts_with("five") {
            Some(5)
        } else if line.starts_with("six") {
            Some(6)
        } else if line.starts_with("seven") {
            Some(7)
        } else if line.starts_with("eight") {
            Some(8)
        } else if line.starts_with("nine") {
            Some(9)
        } else {
            line.chars().next().unwrap().to_digit(10)
        }
    });

    let first = it.next().unwrap();
    let last = it.last().unwrap_or(first);

    first * 10 + last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_example() {
        let input = include_str!("../example2.txt");
        let output = part2(input);
        let expected = 281;
        assert_eq!(output, expected);
    }

    #[test]
    fn part2_input() {
        let input = include_str!("../input.txt");
        let output = part2(input);
        let expected = 54203;
        assert_eq!(output, expected);
    }

    #[rstest::rstest]
    #[case("4sixnineone", 41)]
    #[case("twossevensseven3", 23)]
    #[case("eightwoeightwo", 82)]
    #[case("eigh938457938745893475897seve", 97)]
    #[case("blahblaheightblahblah", 88)]
    #[case("blahblah3blahblah", 33)]
    #[case("three46754645645645645645four", 34)]
    #[case("4fivefour5", 45)]
    fn line_test(#[case] line: &str, #[case] expected: u32) {
        assert_eq!(get_num_from_line(line), expected);
    }
}
