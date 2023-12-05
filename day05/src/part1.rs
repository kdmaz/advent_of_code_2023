pub fn part1(input: &str) -> i64 {
    let mut lines = input.lines();

    let seeds: Vec<i64> = lines
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split(' ')
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect();

    let maps = lines.filter(|line| !line.is_empty()).fold(
        vec![] as Vec<Vec<Range>>,
        |mut groups, line| {
            if line.chars().peekable().peek().unwrap().is_ascii_digit() {
                let group_len = groups.len();
                groups[group_len - 1].push(Range::new(line));
            } else {
                groups.push(vec![]);
            }

            groups
        },
    );

    let locations = maps.into_iter().fold(seeds, |sources, mapping_ranges| {
        sources
            .into_iter()
            .map(|source: i64| {
                for range in &mapping_ranges {
                    if let Some(destination) = range.get_destination_from_source(source) {
                        return destination;
                    }
                }

                source
            })
            .collect()
    });

    locations.into_iter().min().unwrap()
}

#[derive(Debug)]
struct Range {
    source_range_start: i64,
    destination_range_start: i64,
    range_length: i64,
}

impl Range {
    fn new(line: &str) -> Self {
        let mut mapping_parts = line.split(' ');
        let destination_range_start = mapping_parts.next().unwrap().parse::<i64>().unwrap();
        let source_range_start = mapping_parts.next().unwrap().parse::<i64>().unwrap();
        let range_length = mapping_parts.next().unwrap().parse::<i64>().unwrap();

        Self {
            source_range_start,
            destination_range_start,
            range_length,
        }
    }

    fn get_destination_from_source(&self, source: i64) -> Option<i64> {
        let is_source_in_range = source >= self.source_range_start
            && source < self.source_range_start + self.range_length;

        if is_source_in_range {
            let diff = self.destination_range_start - self.source_range_start;
            Some(source + diff)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../example.txt");
        let output = part1(input);
        let expected = 35;
        assert_eq!(output, expected);
    }

    #[test]
    fn part1_input() {
        let input = include_str!("../input.txt");
        let output = part1(input);
        let expected = 57075758;
        assert_eq!(output, expected);
    }
}
