use nom::{
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    IResult,
};

pub fn part1(input: &str) -> i32 {
    let (input, num_groups) = nums_parser(input).unwrap();
    debug_assert_eq!(input, "", "input should be consumed");

    num_groups
        .into_iter()
        .map(build_history)
        .map(calculate_next_value)
        .sum()
}

fn build_history(nums: Vec<i32>) -> History {
    let mut history = vec![nums];

    loop {
        let last_sequence = &history[history.len() - 1];

        if last_sequence.iter().all(|&num| num == 0) {
            break history;
        }

        let new_sequence = last_sequence
            .windows(2)
            .map(|nums| {
                if nums.len() < 2 {
                    nums[0]
                } else {
                    nums[1] - nums[0]
                }
            })
            .collect();
        history.push(new_sequence);
    }
}

fn calculate_next_value(history: History) -> i32 {
    history
        .into_iter()
        .rev()
        .skip(1)
        .fold(0, |prev, sequence| sequence[sequence.len() - 1] + prev)
}

fn nums_parser(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list1(line_ending, separated_list1(space1, complete::i32))(input)
}

type History = Vec<Vec<i32>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../example.txt");
        let output = part1(input);
        let expected = 114;
        assert_eq!(output, expected);
    }

    #[test]
    fn part1_input() {
        let input = include_str!("../input.txt");
        let output = part1(input);
        let expected = 1953784198;
        assert_eq!(output, expected);
    }
}
