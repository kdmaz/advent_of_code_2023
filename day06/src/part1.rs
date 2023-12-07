pub fn part1(input: &str) -> i32 {
    let mut lines = input.lines();

    let time = lines
        .next()
        .unwrap()
        .split(' ')
        .filter_map(|s| s.parse::<i32>().ok());

    let distance = lines
        .next()
        .unwrap()
        .split(' ')
        .filter_map(|s| s.parse::<i32>().ok());

    time.zip(distance).fold(1, |acc, (time, distance)| {
        acc * (0..=time)
            .filter(|ms_button_held| {
                let time_to_travel = time - ms_button_held;
                let distance_traveled = ms_button_held * time_to_travel;
                distance_traveled > distance
            })
            .count() as i32
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../example.txt");
        let output = part1(input);
        let expected = 288;
        assert_eq!(output, expected);
    }

    #[test]
    fn part1_input() {
        let input = include_str!("../input.txt");
        let output = part1(input);
        let expected = 800280;
        assert_eq!(output, expected);
    }
}
