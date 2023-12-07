use std::cmp::Ordering;

pub fn part2(input: &str) -> u64 {
    let mut lines = input.lines();

    let time = lines
        .next()
        .unwrap()
        .chars()
        .filter(|s| s.is_ascii_digit())
        .fold(0, |acc, s| acc * 10 + s.to_digit(10).unwrap() as u64);

    let distance = lines
        .next()
        .unwrap()
        .chars()
        .filter(|s| s.is_ascii_digit())
        .fold(0, |acc, s| acc * 10 + s.to_digit(10).unwrap() as u64);

    let (mut l, mut r) = (0, time);
    while l < r {
        let ms_button_held = l + ((r - l) / 2);
        let time_to_travel = time - ms_button_held;
        let distance_traveled = ms_button_held * time_to_travel;

        match distance_traveled.cmp(&distance) {
            Ordering::Less | Ordering::Equal => l = ms_button_held + 1,
            Ordering::Greater => r = ms_button_held,
        }
    }
    let low = l + ((r - l) / 2);

    let (mut l, mut r) = (0, time);
    while l < r {
        let ms_button_held = l + ((r - l) / 2);
        let time_to_travel = time - ms_button_held;
        let distance_traveled = ms_button_held * time_to_travel;

        match distance_traveled.cmp(&distance) {
            Ordering::Less => r = ms_button_held,
            Ordering::Equal | Ordering::Greater => l = ms_button_held + 1,
        }
    }
    let high = l + ((r - l) / 2);

    high - low
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_example() {
        let input = include_str!("../example.txt");
        let output = part2(input);
        let expected = 71503;
        assert_eq!(output, expected);
    }

    #[test]
    fn part2_input() {
        let input = include_str!("../input.txt");
        let output = part2(input);
        let expected = 45128024;
        assert_eq!(output, expected);
    }
}
