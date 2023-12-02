fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    let input = include_str!("../input.txt");
    day01::part1(input);
}

#[divan::bench]
fn part2() {
    let input = include_str!("../input.txt");
    day01::part2(input);
}
