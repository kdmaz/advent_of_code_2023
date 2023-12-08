use itertools::{FoldWhile, Itertools};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending, multispace1},
    combinator::eof,
    multi::{fold_many1, many1},
    sequence::{separated_pair, terminated},
    IResult, Parser,
};
use std::collections::HashMap;

pub fn part1(input: &str) -> i32 {
    let (input, instructions) =
        instructions_parser(input).expect("instructions should be parsable");
    let (input, nodes) = nodes_parser(input).expect("nodes should be parsable");
    debug_assert_eq!(input, "", "input should be consumed");

    instructions
        .iter()
        .cycle()
        .enumerate()
        .fold_while(
            (0, "AAA"),
            |(_, current_node_key), (steps_count, instruction)| {
                let node = nodes
                    .get(current_node_key)
                    .expect("node_key should always match a valid node");

                let next_node_key = match instruction {
                    Instruction::Left => node.left,
                    Instruction::Right => node.right,
                };

                let results = (steps_count as i32 + 1, next_node_key);
                match next_node_key {
                    "ZZZ" => FoldWhile::Done(results),
                    _ => FoldWhile::Continue(results),
                }
            },
        )
        .into_inner()
        .0
}

fn instructions_parser(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, instructions) = many1(alt((
        complete::char('L').map(|_| Instruction::Left),
        complete::char('R').map(|_| Instruction::Right),
    )))(input)?;
    let (input, _) = multispace1(input)?;
    Ok((input, instructions))
}

fn nodes_parser(input: &str) -> IResult<&str, HashMap<&str, Node>> {
    fold_many1(
        terminated(node_parser, alt((line_ending, eof))),
        HashMap::new,
        |mut nodes, (key, node)| {
            nodes.insert(key, node);
            nodes
        },
    )(input)
}

fn node_parser(input: &str) -> IResult<&str, (&str, Node)> {
    let (input, key) = alpha1(input)?;
    let (input, _) = tag(" = (")(input)?;
    let (input, (left, right)) = separated_pair(alpha1, tag(", "), alpha1)(input)?;
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
    fn part1_example1() {
        let input = include_str!("../example.txt");
        let output = part1(input);
        let expected = 2;
        assert_eq!(output, expected);
    }

    #[test]
    fn part1_example2() {
        let input = include_str!("../example2.txt");
        let output = part1(input);
        let expected = 6;
        assert_eq!(output, expected);
    }

    #[test]
    fn part1_input() {
        let input = include_str!("../input.txt");
        let output = part1(input);
        let expected = 13301;
        assert_eq!(output, expected);
    }
}
