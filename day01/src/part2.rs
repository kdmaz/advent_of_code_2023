const NUMS: &'static [&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            let mut first = None;
            'outer: for (i, &c) in chars.iter().enumerate() {
                if c.is_ascii_digit() {
                    first = Some(c);
                    break;
                }

                let word: String = chars[i..chars.len()].iter().collect();
                for num in NUMS.iter() {
                    if word.starts_with(num) {
                        first = Some(get_digit_from_num(num));
                        break 'outer;
                    }
                }
            }
            let first = first.unwrap();

            let mut last = None;
            'outer: for (i, &c) in chars.iter().enumerate().rev() {
                if c.is_ascii_digit() {
                    last = Some(c);
                    break;
                }

                let word: String = chars[0..=i].iter().collect();
                for num in NUMS.iter() {
                    if word.ends_with(num) {
                        last = Some(get_digit_from_num(num));
                        break 'outer;
                    }
                }
            }
            let last = last.unwrap();

            format!("{}{}", first, last).parse::<i32>().unwrap()
        })
        .sum()
}

fn get_digit_from_num(num: &str) -> char {
    match num {
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        _ => unreachable!(),
    }
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
}
