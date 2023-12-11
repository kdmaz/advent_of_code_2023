use nom::{
    character::complete::{line_ending, none_of},
    multi::{many1, separated_list1},
    IResult, Parser,
};

pub fn part1(input: &str) -> u32 {
    let (input, universe) = universe_parser(input).unwrap();
    debug_assert_eq!(input, "");
    let expansion_points = get_expansion_points(&universe);
    let universe = expand_universe(universe, expansion_points);
    let galaxy_positions = get_galaxy_positions(universe);
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

fn expand_universe(universe: Universe, expansion_points: Vec<ExpansionPoint>) -> Universe {
    expansion_points
        .into_iter()
        .rev()
        .fold(universe, |mut universe, expansion_point| {
            match expansion_point {
                ExpansionPoint::Row(y) => {
                    universe.insert(y, universe[y].clone());
                }
                ExpansionPoint::Col(x) => {
                    for y in (0..universe.len()).rev() {
                        universe[y].insert(x, DataPoint::Empty);
                    }
                }
            }
            universe
        })
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

fn get_shortest_path_sum(galaxy_positions: Vec<Position>) -> u32 {
    let mut count = 0;
    for (i, p1) in galaxy_positions.iter().enumerate() {
        for p2 in galaxy_positions.iter().skip(i) {
            let x = (p2.x as i32 - p1.x as i32).unsigned_abs();
            let y = (p2.y as i32 - p1.y as i32).unsigned_abs();
            count += x + y;
        }
    }
    count
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

    #[test]
    fn part1_example() {
        let input = include_str!("../example.txt");
        let output = part1(input);
        let expected = 374;
        assert_eq!(output, expected);
    }

    #[test]
    fn part1_input() {
        let input = include_str!("../input.txt");
        let output = part1(input);
        let expected = 9965032;
        assert_eq!(output, expected);
    }
}
