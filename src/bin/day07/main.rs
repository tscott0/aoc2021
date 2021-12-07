use aoc2021::utils;

fn main() {
    let input = utils::string_from_file("src/bin/day07/input");

    println!("PART 1: {:?}", part1(input.clone()));
    println!("PART 2: {:?}", part2(input));
}

pub fn part1(input: String) -> usize {
    let mut crabs = input
        .split(",")
        .map(|c| c.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    crabs.sort();

    let min = *crabs.first().unwrap();
    let max = *crabs.last().unwrap();

    (min..=max)
        .into_iter()
        .map(|pos| {
            crabs
                .iter()
                .map(|&c| (pos as isize - c as isize).abs() as usize)
                .sum()
        })
        .min()
        .unwrap()
}

pub fn part2(input: String) -> usize {
    let mut crabs = input
        .split(",")
        .map(|c| c.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    crabs.sort();

    let min = *crabs.first().unwrap();
    let max = *crabs.last().unwrap();

    (min..=max)
        .into_iter()
        .map(|pos| {
            crabs
                .iter()
                .map(|&c| (pos as isize - c as isize).abs() as usize)
                .map(|c| (c.pow(2) + c) / 2)
                .sum()
        })
        .min()
        .unwrap()
}

#[test]
fn test_part_1() {
    let input = "16,1,2,0,4,2,7,1,2,14";

    assert_eq!(part1(input.to_string()), 37);
}

#[test]
fn test_part_2() {
    let input = "16,1,2,0,4,2,7,1,2,14";

    assert_eq!(part2(input.to_string()), 168);
}
