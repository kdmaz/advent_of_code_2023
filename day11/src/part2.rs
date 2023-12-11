use nom::{
    character::complete::{line_ending, none_of},
    multi::{many1, separated_list1},
    IResult, Parser,
};

pub fn part2(input: &str, expansion_amount: u64) -> u64 {
    let (input, universe) = universe_parser(input).unwrap();
    debug_assert_eq!(input, "");
    let expansion_points = get_expansion_points(&universe);
    let mut galaxy_positions = get_galaxy_positions(universe);
    galaxy_positions =
        expand_galaxy_positions(galaxy_positions, expansion_points, expansion_amount);
    get_shortest_path_sum(galaxy_positions)
}

fn universe_parser(input: &str) -> IResult<&str, Universe> {
    separated_list1(
        line_ending,
        many1(none_of("\r\n").map(|c| DataPoint::try_from(c).unwrap())),
    )(input)
}

fn get_expansion_points(universe: &Universe) -> Vec<ExpansionPoint> {
    let (y_expand_points, x_expand_points) = universe.iter().enumerate().fold(
        (vec![true; universe.len()], vec![true; universe[0].len()]),
        |(mut y_expand_points, mut x_expand_points), (y, data_points)| {
            for (x, data_point) in data_points.iter().enumerate() {
                if data_point == &DataPoint::Galaxy {
                    y_expand_points[y] = false;
                    x_expand_points[x] = false;
                }
            }
            (y_expand_points, x_expand_points)
        },
    );

    let x_expand_points = x_expand_points
        .into_iter()
        .enumerate()
        .filter_map(|(x, can_expand)| can_expand.then_some(ExpansionPoint::Col(x)));

    let y_expand_points = y_expand_points
        .into_iter()
        .enumerate()
        .filter_map(|(y, can_expand)| can_expand.then_some(ExpansionPoint::Row(y)));

    y_expand_points.chain(x_expand_points).collect()
}

fn get_galaxy_positions(universe: Universe) -> Vec<Position> {
    universe
        .into_iter()
        .enumerate()
        .fold(vec![], |mut galaxy_positions, (y, data_points)| {
            for (x, data_point) in data_points.into_iter().enumerate() {
                if data_point == DataPoint::Galaxy {
                    galaxy_positions.push(Position::new(x, y));
                }
            }
            galaxy_positions
        })
}

fn expand_galaxy_positions(
    galaxy_positions: Vec<Position>,
    expansion_points: Vec<ExpansionPoint>,
    expansion_amount: u64,
) -> Vec<Position> {
    galaxy_positions
        .into_iter()
        .map(|mut pos| {
            let mut offset_x = 0;
            let mut offset_y = 0;

            for expansion_point in expansion_points.iter() {
                match expansion_point {
                    ExpansionPoint::Row(y) => {
                        if pos.y > *y + offset_y {
                            pos.y += expansion_amount as usize - 1;
                            offset_y += expansion_amount as usize - 1;
                        }
                    }
                    ExpansionPoint::Col(x) => {
                        if pos.x > *x + offset_x {
                            pos.x += expansion_amount as usize - 1;
                            offset_x += expansion_amount as usize - 1;
                        }
                    }
                }
            }
            pos
        })
        .collect()
}

fn get_shortest_path_sum(galaxy_positions: Vec<Position>) -> u64 {
    galaxy_positions
        .iter()
        .enumerate()
        .fold(0, |mut count, (i, p1)| {
            for p2 in galaxy_positions.iter().skip(i) {
                let x = (p2.x as i64 - p1.x as i64).unsigned_abs();
                let y = (p2.y as i64 - p1.y as i64).unsigned_abs();
                count += x + y;
            }

            count
        })
}

type Universe = Vec<Vec<DataPoint>>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum DataPoint {
    Empty,
    Galaxy,
}

impl TryFrom<char> for DataPoint {
    type Error = String;

    fn try_from(data_point: char) -> Result<Self, Self::Error> {
        Ok(match data_point {
            '.' => DataPoint::Empty,
            '#' => DataPoint::Galaxy,
            _ => return Err(format!("\"{data_point}\" is not a valid data point.")),
        })
    }
}

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum ExpansionPoint {
    Row(usize),
    Col(usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest::rstest]
    #[case(2, 374)]
    #[case(10, 1030)]
    #[case(100, 8410)]
    fn part2_example(#[case] expansion_amount: u64, #[case] expected: u64) {
        let input = include_str!("../example.txt");
        let output = part2(input, expansion_amount);
        assert_eq!(output, expected);
    }

    #[test]
    fn part2_input() {
        let input = include_str!("../input.txt");
        let output = part2(input, 1_000_000);
        let expected = 550358864332;
        assert_eq!(output, expected);
    }
}
