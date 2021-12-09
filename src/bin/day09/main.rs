use std::collections::HashSet;

use aoc2021::utils;
use itertools::Itertools;

fn main() {
    let input = utils::string_from_file("src/bin/day09/input");

    println!("PART 1: {:?}", part1(input.clone()));
    println!("PART 2: {:?}", part2(input));
}

pub fn part1(input: String) -> usize {
    let grid = input
        .lines()
        .map(|l| {
            l.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let max_x = grid[0].len();
    let max_y = grid.len();

    let mut subtotal: usize = 0;
    grid.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, &current)| {
            let neighbours = [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .iter()
                .filter_map(|(x_off, y_off)| {
                    let new_x = x as isize + *x_off;
                    let new_y = y as isize + *y_off;
                    if new_x >= 0 && new_x < max_x as isize && new_y >= 0 && new_y < max_y as isize
                    {
                        // println!("current: {:?}", current);
                        Some(((new_x as usize), (new_y as usize)))
                    } else {
                        None
                    }
                })
                .map(|(fx, fy)| grid[fy][fx])
                .collect::<Vec<_>>();

            if neighbours.iter().all(|&n| current < n) {
                // println!("current: {:?}", current);
                // println!("neighbours: {:?}", neighbours);
                // println!("risk: {:?}", current as usize + 1);
                subtotal += current as usize + 1;
            }
        });
    });

    subtotal
}

pub fn part2(input: String) -> usize {
    let grid = input
        .lines()
        .map(|l| {
            l.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let max_x = grid[0].len();
    let max_y = grid.len();

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut basins: Vec<u8> = vec![];
    grid.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, _)| {
            let pos = (x, y);

            let basin_size = basin_walk(pos, &grid, max_x, max_y, &mut visited);
            if basin_size > 0 {
                basins.push(basin_size);
            }
        });
    });

    basins.sort();
    basins.reverse();
    basins
        .iter()
        .sorted()
        .rev()
        .take(3)
        .map(|&x| x as usize)
        .product::<usize>()
}

fn basin_walk(
    pos: (usize, usize),
    grid: &Vec<Vec<u8>>,
    max_x: usize,
    max_y: usize,
    visited: &mut HashSet<(usize, usize)>,
) -> u8 {
    if visited.contains(&pos) {
        return 0;
    }

    visited.insert(pos);
    let (x, y) = pos;
    let current = grid[y][x];

    if current == 9 {
        return 0;
    }

    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .filter_map(|(x_off, y_off)| {
            let new_x = x as isize + *x_off;
            let new_y = y as isize + *y_off;
            if new_x >= 0 && new_x < max_x as isize && new_y >= 0 && new_y < max_y as isize {
                // println!("current: {:?}", current);
                Some(((new_x as usize), (new_y as usize)))
            } else {
                None
            }
        })
        .map(|(fx, fy)| basin_walk((fx, fy), grid, max_x, max_y, visited))
        .sum::<u8>()
        + 1
}

#[test]
fn test_part_1() {
    let input = "2199943210
        3987894921
        9856789892
        8767896789
        9899965678";

    assert_eq!(part1(input.to_string()), 15);
}

#[test]
fn test_part_2() {
    let input = "2199943210
        3987894921
        9856789892
        8767896789
        9899965678";

    assert_eq!(part2(input.to_string()), 1134);
}
