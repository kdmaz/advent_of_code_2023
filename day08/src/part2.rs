use itertools::{FoldWhile, Itertools};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alphanumeric1, line_ending, multispace1},
    combinator::eof,
    multi::{fold_many1, many1},
    sequence::{separated_pair, terminated},
    IResult, Parser,
};
use std::collections::HashMap;

pub fn part2(input: &str) -> usize {
    let (input, instructions) =
        instructions_parser(input).expect("instructions should be parsable");
    let (input, (nodes, starting_node_keys)) =
        nodes_parser(input).expect("nodes should be parsable");
    debug_assert_eq!(input, "", "input should be consumed");

    let cycles: Vec<usize> = starting_node_keys
        .iter()
        .map(|&key| {
            instructions
                .iter()
                .cycle()
                .enumerate()
                .fold_while((0, key), |(_, current_key), (steps_count, instruction)| {
                    let node = nodes
                        .get(current_key)
                        .expect("node_key should always match a valid node");

                    let next_key = match instruction {
                        Instruction::Left => node.left,
                        Instruction::Right => node.right,
                    };

                    let result = (steps_count + 1, next_key);

                    if next_key.ends_with('Z') {
                        FoldWhile::Done(result)
                    } else {
                        FoldWhile::Continue(result)
                    }
                })
                .into_inner()
                .0
        })
        .collect();

    lcm(&cycles)
}

// https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn instructions_parser(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, instructions) = many1(alt((
        complete::char('L').map(|_| Instruction::Left),
        complete::char('R').map(|_| Instruction::Right),
    )))(input)?;
    let (input, _) = multispace1(input)?;
    Ok((input, instructions))
}

fn init<'a>() -> (HashMap<&'a str, Node<'a>>, Vec<&'a str>) {
    (HashMap::new(), vec![])
}

fn nodes_parser(input: &str) -> IResult<&str, (HashMap<&str, Node>, Vec<&str>)> {
    fold_many1(
        terminated(node_parser, alt((line_ending, eof))),
        init,
        |(mut nodes, mut starting_nodes), (key, node)| {
            if key.ends_with('A') {
                starting_nodes.push(key);
            }

            nodes.insert(key, node);
            (nodes, starting_nodes)
        },
    )(input)
}

fn node_parser(input: &str) -> IResult<&str, (&str, Node)> {
    let (input, key) = alphanumeric1(input)?;
    let (input, _) = tag(" = (")(input)?;
    let (input, (left, right)) = separated_pair(alphanumeric1, tag(", "), alphanumeric1)(input)?;
    let (input, _) = tag(")")(input)?;
    let node = Node { left, right };

    Ok((input, (key, node)))
}

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug)]
struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_example() {
        let input = include_str!("../example3.txt");
        let output = part2(input);
        let expected = 6;
        assert_eq!(output, expected);
    }

    #[test]
    fn part2_input() {
        let input = include_str!("../input.txt");
        let output = part2(input);
        let expected = 7309459565207;
        assert_eq!(output, expected);
    }
}
