use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

pub fn part2(input: &str) -> i32 {
    let card_copy_map = RefCell::new(HashMap::new());

    input
        .lines()
        .map(|line| {
            let mut line_parts = line.split(':');
            let card_num = line_parts
                .next()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse::<i32>()
                .unwrap();
            let mut card_parts = line_parts.last().unwrap().split('|');
            let winning_nums = HashSet::<i32>::from_iter(
                card_parts
                    .next()
                    .unwrap()
                    .split(' ')
                    .filter_map(|s| s.parse::<i32>().ok()),
            );

            let copies_won = card_parts
                .next()
                .unwrap()
                .split(' ')
                .filter_map(|s| s.parse::<i32>().ok())
                .filter(|n| winning_nums.contains(n))
                .count() as i32;

            let current_num_copies = card_copy_map.borrow().get(&card_num).copied().unwrap_or(1);

            for i in 1..=copies_won {
                *card_copy_map.borrow_mut().entry(card_num + i).or_insert(1) += current_num_copies;
            }

            card_copy_map.borrow_mut().remove(&card_num);
            current_num_copies
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_example() {
        let input = include_str!("../example.txt");
        let output = part2(input);
        let expected = 30;
        assert_eq!(output, expected);
    }

    #[test]
    fn part2_input() {
        let input = include_str!("../input.txt");
        let output = part2(input);
        let expected = 13114317;
        assert_eq!(output, expected);
    }
}
