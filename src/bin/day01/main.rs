use aoc2021::utils;

fn main() {
    let input = utils::string_from_file("src/bin/day01/input");

    println!("PART 1");
    part1(input.clone());

    println!("PART 2");
    part2(input);
}

pub fn part1(input: String) -> usize {
    let mut previous: Option<usize> = None;
    let answer = input
        .lines()
        .map(|x| x.trim().parse::<usize>().unwrap())
        .fold(0, |count, current| {
            if previous.is_none() || current <= previous.unwrap() {
                previous = Some(current);
                count
            } else {
                previous = Some(current);
                count + 1
            }
        });

    println!("Answer: {:?}", answer);

    answer
}

pub fn part2(input: String) -> usize {
    let mut previous: Option<usize> = None;
    let answer = input
        .lines()
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
        .windows(3)
        .map(|items| items.iter().sum::<usize>())
        .fold(0, |count, current| {
            if previous.is_none() || current <= previous.unwrap() {
                previous = Some(current);
                count
            } else {
                previous = Some(current);
                count + 1
            }
        });

    println!("Answer: {:?}", answer);

    answer
}

#[test]
fn test_part_1() {
    let input = "199
    200
    208
    210
    200
    207
    240
    269
    260
    263";

    assert_eq!(part1(input.to_string()), 7);
}

#[test]
fn test_part_2() {
    let input = "199
    200
    208
    210
    200
    207
    240
    269
    260
    263";

    assert_eq!(part2(input.to_string()), 5);
}
