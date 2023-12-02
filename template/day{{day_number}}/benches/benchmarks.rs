fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    let input = include_str!("../input.txt");
    day{{day_number}}::part1(input);
}

#[divan::bench]
fn part2() {
    let input = include_str!("../input.txt");
    day{{day_number}}::part2(input);
}
