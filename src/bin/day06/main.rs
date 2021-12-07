use std::collections::HashMap;

use aoc2021::utils;

fn main() {
    let input = utils::string_from_file("src/bin/day06/input");

    println!("PART 1: {:?}", solve(input.clone(), 80));
    println!("PART 2: {:?}", solve(input, 256));
}

pub fn solve(input: String, day: isize) -> usize {
    let fish = input
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    println!("fish: {:?}", fish);

    let mut computed: HashMap<isize, usize> = HashMap::new();
    println!("walk2(18): {:?}", walk(18, &mut computed));

    fish.into_iter()
        .map(|f| {
            walk(day - f as isize, &mut computed)
        })
        .sum()
}

// delta is the number of days remaining for this anglerfish
fn walk(delta: isize, computed: &mut HashMap<isize, usize>) -> usize {
    return if let Some(val) = computed.get(&delta) {
        *val
    } else {
        if delta > 0 {
            let sum = walk(delta - 7, computed) + walk(delta - 9, computed);
            computed.insert(delta, sum);
            sum
        } else {
            1
        }
    };
}

#[test]
fn test_part_1_ex_1() {
    let input = "3,4,3,1,2";

    assert_eq!(solve(input.to_string(), 18), 26);
}

#[test]
fn test_part_1_ex_2() {
    let input = "3,4,3,1,2";

    assert_eq!(solve(input.to_string(), 80), 5934);
}

#[test]
fn test_part_2() {
    let input = "3,4,3,1,2";

    assert_eq!(solve(input.to_string(), 256), 26984457539);
}
