use std::{cmp::max, collections::HashSet};

use aoc2021::utils;
use regex::Regex;

fn main() {
    let input = utils::string_from_file("src/bin/day17/input");

    println!("PART 1: {:?}", part1(input.clone()));
    println!("PART 2: {:?}", part2(input.clone()));
}

fn part1(input: String) -> usize {
    let (max_y, _) = solve(input);
    max_y
}

fn part2(input: String) -> usize {
    let (_, valid_velocities) = solve(input);
    valid_velocities
}

fn solve(input: String) -> (usize, usize) {
    let re = Regex::new(r"target area: x=(.+)\.\.(.+), y=(.+)\.\.(.+)").unwrap();
    let cap = re.captures(input.as_str()).unwrap();

    let min_target_x = cap[1].parse::<i32>().unwrap();
    let max_target_x = cap[2].parse::<i32>().unwrap();
    let min_target_y = cap[3].parse::<i32>().unwrap();
    let max_target_y = cap[4].parse::<i32>().unwrap();
    println!(
        "{}..{} {}..{}",
        min_target_x, max_target_x, min_target_y, max_target_y
    );

    // Calculate the minimum X velocity to reach the target
    let mut min_initial_x = 0;
    loop {
        if x_position_after_steps(min_initial_x, min_initial_x) >= min_target_x {
            break;
        }

        min_initial_x += 1;
    }

    // Calculate the maximum X velocity before we overshoot
    let max_initial_x = max_target_x;

    let mut highest_y = 0;
    let mut velocities: HashSet<(i32, i32)> = HashSet::new();
    for y in 2 * min_target_y..500 {
        // calculate max steps using the initial y velocity only
        // y is always positive
        let min_steps = if y < 0 { 0 } else { 2 * y };
        let mut max_steps = min_steps;

        loop {
            if y_position_after_steps(y, max_steps) < min_target_y {
                max_steps -= 1;
                break;
            }

            max_steps += 1;
        }

        for x in min_initial_x..=max_initial_x {
            for s in min_steps..=max_steps {
                let x_attempt = x_position_after_steps(x, s);
                let y_attempt = y_position_after_steps(y, s);

                if (x_attempt >= min_target_x && x_attempt <= max_target_x)
                    && (y_attempt >= min_target_y && y_attempt <= max_target_y)
                {
                    // println!(
                    //     "velocity {},{} step {} coord {},{}",
                    //     x, y, s, x_attempt, y_attempt
                    // );

                    velocities.insert((x, y));

                    // Optimise to not check y after it reaches 0 (after y steps)
                    if y <= s {
                        highest_y = max(highest_y, max_y_after_steps(y, s));
                    }
                }
            }
        }
    }

    (highest_y as usize, velocities.len())
}

fn x_position_after_steps(initial_x: i32, steps: i32) -> i32 {
    (0..steps).fold(0, |acc, s| acc + max(initial_x - s, 0) as i32)
}

fn y_position_after_steps(initial_y: i32, steps: i32) -> i32 {
    (0..steps).fold(0, |acc, s| acc + initial_y - s as i32)
}

fn max_y_after_steps(initial_y: i32, steps: i32) -> i32 {
    (0..=steps)
        .map(|s| y_position_after_steps(initial_y, s))
        .max()
        .unwrap()
}

#[test]
fn test_part_1() {
    assert_eq!(part1("target area: x=20..30, y=-10..-5".to_string()), 45);
}

#[test]
fn test_part_2() {
    assert_eq!(part2("target area: x=20..30, y=-10..-5".to_string()), 112);
}
