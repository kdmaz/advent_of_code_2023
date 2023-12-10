fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    let input = include_str!("../input.txt");
    day10::part1(divan::black_box(input));
}

#[divan::bench]
fn part2() {
    let input = include_str!("../input.txt");
    day10::part2(divan::black_box(input));
}
