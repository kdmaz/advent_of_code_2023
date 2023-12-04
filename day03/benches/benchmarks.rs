fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    let input = include_str!("../input.txt");
    day03::part1(divan::black_box(input));
}

#[divan::bench]
fn part1_set() {
    let input = include_str!("../input.txt");
    day03::part1_set::part1(divan::black_box(input));
}

#[divan::bench]
fn part2() {
    let input = include_str!("../input.txt");
    day03::part2(divan::black_box(input));
}
