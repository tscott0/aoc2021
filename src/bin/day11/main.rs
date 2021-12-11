use std::collections::{HashMap, HashSet};

use aoc2021::utils;
use itertools::Itertools;

fn main() {
    let input = utils::string_from_file("src/bin/day11/input");

    println!("PART 1: {:?}", part1(input.clone(), 100));
    println!("PART 2: {:?}", part2(input));
}

pub fn part1(input: String, steps: usize) -> usize {
    let mut octopuses = input
        .lines()
        .map(|l| l.trim())
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut flash_count: usize = 0;
    for s in 0..steps {
        // First, the energy level of each octopus increases by 1.
        octopuses
            .iter_mut()
            .for_each(|row| row.iter_mut().for_each(|o| *o += 1));

        // Flashes
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        for x in 0..10 {
            for y in 0..10 {
                if octopuses[y][x] > 9 {
                    flash(&mut octopuses, x, y, &mut flash_count, &mut visited);
                }
            }
        }

        // Reset
        octopuses.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|o| {
                if *o > 9 {
                    *o = 0
                }
            })
        });

        for r in octopuses.iter() {
            println!("{:?}", r);
        }

        println!("step: {:?} flash_count: {:?}", s + 1, flash_count);
    }

    flash_count
}

fn flash(
    octopuses: &mut Vec<Vec<u32>>,
    x: usize,
    y: usize,
    flash_count: &mut usize,
    visited: &mut HashSet<(usize, usize)>,
) {
    if visited.contains(&(x, y)) {
        return;
    } else {
        visited.insert((x, y));
    }

    *flash_count += 1;
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
    ]
    .iter()
    .filter_map(|(x_off, y_off)| {
        let new_x = x as isize + *x_off;
        let new_y = y as isize + *y_off;
        if new_x >= 0 && new_x < 10 as isize && new_y >= 0 && new_y < 10 as isize {
            // println!("current: {:?}", ((new_x as usize), (new_y as usize)));
            Some(((new_x as usize), (new_y as usize)))
        } else {
            None
        }
    })
    .for_each(|(x, y)| {
        octopuses[y][x] += 1;
        let o = octopuses[y][x];
        if o > 9 {
            flash(octopuses, x, y, flash_count, visited);
        }
    });
}

pub fn part2(input: String) -> usize {
    let mut octopuses = input
        .lines()
        .map(|l| l.trim())
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut step = 1;
    loop {
        // First, the energy level of each octopus increases by 1.
        octopuses
            .iter_mut()
            .for_each(|row| row.iter_mut().for_each(|o| *o += 1));

        // Flashes
        let mut flash_count: usize = 0;
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        for x in 0..10 {
            for y in 0..10 {
                if octopuses[y][x] > 9 {
                    flash(&mut octopuses, x, y, &mut flash_count, &mut visited);
                }
            }
        }

        // Reset
        octopuses.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|o| {
                if *o > 9 {
                    *o = 0
                }
            })
        });

        println!("step: {:?} flash_count: {:?}", step, flash_count);
        if flash_count == 100 {
            return step;
        }

        step += 1;
    }
}

#[test]
fn test_part_1() {
    let input = "5483143223
    2745854711
    5264556173
    6141336146
    6357385478
    4167524645
    2176841721
    6882881134
    4846848554
    5283751526";

    assert_eq!(part1(input.to_string(), 10), 204);
    assert_eq!(part1(input.to_string(), 100), 1656);
}

#[test]
fn test_part_2() {
    let input = "5483143223
    2745854711
    5264556173
    6141336146
    6357385478
    4167524645
    2176841721
    6882881134
    4846848554
    5283751526";

    assert_eq!(part2(input.to_string()), 195);
}
