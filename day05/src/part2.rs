pub fn part2(input: &str) -> i64 {
    let mut lines = input.lines();

    let seed_ranges: Vec<&str> = lines
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split(' ')
        .collect();

    let seed_ranges: Vec<RangeWithStatus> = seed_ranges
        .chunks(2)
        .map(|seed_range| {
            let start = seed_range[0].parse::<i64>().unwrap();
            let range = seed_range[1].parse::<i64>().unwrap();
            RangeWithStatus::Unmapped(Range::new(start, start + range - 1))
        })
        .collect();

    let map_ranges_groups = lines.filter(|line| !line.is_empty()).fold(
        vec![] as Vec<Vec<MapRange>>,
        |mut groups, line| {
            if line.chars().peekable().peek().unwrap().is_ascii_digit() {
                let group_len = groups.len();
                groups[group_len - 1].push(MapRange::new(line));
            } else {
                groups.push(vec![]);
            }

            groups
        },
    );

    let locations_ranges = map_ranges_groups
        .into_iter()
        .fold(seed_ranges, |source_ranges, map_ranges| {
            get_ranges_with_statuses(source_ranges, map_ranges)
        });

    locations_ranges
        .into_iter()
        .map(|range| match range {
            RangeWithStatus::Mapped(r) => r.start,
            RangeWithStatus::Unmapped(r) => r.start,
        })
        .min()
        .unwrap()
}

fn get_ranges_with_statuses(
    source_ranges: Vec<RangeWithStatus>,
    map_ranges: Vec<MapRange>,
) -> Vec<RangeWithStatus> {
    let unmapped_source_ranges: Vec<RangeWithStatus> = source_ranges
        .iter()
        .map(|destination_range| {
            if let RangeWithStatus::Mapped(r) = destination_range {
                RangeWithStatus::Unmapped(*r)
            } else {
                *destination_range
            }
        })
        .collect();

    map_ranges
        .into_iter()
        .fold(unmapped_source_ranges, |source_ranges, map_range| {
            source_ranges
                .iter()
                .fold(vec![], |mut destination_ranges, source_range| {
                    if let RangeWithStatus::Unmapped(r) = &source_range {
                        let mut ranges = map_range.get_destination_ranges(r);
                        destination_ranges.append(&mut ranges);
                    } else {
                        destination_ranges.push(*source_range);
                    }

                    destination_ranges
                })
        })
}

#[derive(Debug, Clone, Copy)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }
}

#[derive(Debug, Clone, Copy)]
enum RangeWithStatus {
    Mapped(Range),
    Unmapped(Range),
}

#[derive(Debug)]
struct MapRange {
    source: Range,
    destination: Range,
}

impl MapRange {
    fn new(line: &str) -> Self {
        let mut mapping_parts = line.split(' ');
        let destination_start = mapping_parts.next().unwrap().parse::<i64>().unwrap();
        let source_start = mapping_parts.next().unwrap().parse::<i64>().unwrap();
        let range = mapping_parts.next().unwrap().parse::<i64>().unwrap();

        let source_end = source_start + range - 1;
        let destination_end = destination_start + range - 1;

        Self {
            source: Range::new(source_start, source_end),
            destination: Range::new(destination_start, destination_end),
        }
    }

    fn get_destination_ranges(&self, source: &Range) -> Vec<RangeWithStatus> {
        let diff = self.destination.start - self.source.start;

        if source.start >= self.source.start && source.end <= self.source.end {
            // range contained in map [map (source) map]

            let destination_range = Range::new(source.start + diff, source.end + diff);

            vec![RangeWithStatus::Mapped(destination_range)]
        } else if source.start < self.source.start && source.end >= self.source.start {
            // range overlapping before map (source [overlap) map]
            let unmapped =
                RangeWithStatus::Unmapped(Range::new(source.start, self.source.start - 1));
            let mapped =
                RangeWithStatus::Mapped(Range::new(self.source.start + diff, source.end + diff));

            vec![unmapped, mapped]
        } else if source.start <= self.source.end && source.end > self.source.end {
            // range overlapping after map [map (overlap] source)

            let mapped =
                RangeWithStatus::Mapped(Range::new(source.start + diff, self.source.end + diff));
            let unmapped = RangeWithStatus::Unmapped(Range::new(self.source.end + 1, source.end));

            vec![mapped, unmapped]
        } else {
            // range not in map (source) [map] or [map] (source)
            vec![RangeWithStatus::Unmapped(*source)]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_example() {
        let input = include_str!("../example.txt");
        let output = part2(input);
        let expected = 46;
        assert_eq!(output, expected);
    }

    #[test]
    fn part2_input() {
        let input = include_str!("../input.txt");
        let output = part2(input);
        let expected = 31161857;
        assert_eq!(output, expected);
    }
}
