use std::collections::HashSet;

use aoc2021::utils;
fn main() {
    let input = utils::string_from_file("src/bin/day05/input");

    println!("PART 1: {:?}", part1(input.clone()));
    println!("PART 2: {:?}", part2(input));
}

pub fn part1(input: String) -> usize {
    let mut all_coords: HashSet<(usize, usize)> = HashSet::new();
    let mut duplicate_coords: HashSet<(usize, usize)> = HashSet::new();
    input.lines().for_each(|l| {
        let line = l
            .split(" -> ")
            .flat_map(|p| p.split(","))
            .map(|s| s.trim().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let x1 = line[0];
        let y1 = line[1];
        let x2 = line[2];
        let y2 = line[3];

        // let mut coords: Vec<(usize, usize)> = vec![];

        if x1 == x2 || y1 == y2 {
            let (x1, x2) = if x1 >= x2 { (x2, x1) } else { (x1, x2) };
            let (y1, y2) = if y1 >= y2 { (y2, y1) } else { (y1, y2) };

            for x in x1..=x2 {
                for y in y1..=y2 {
                    let new_coord = (x, y);
                    if all_coords.contains(&new_coord) {
                        duplicate_coords.insert(new_coord);
                        // println!("duplicate: {:?}", new_coord);
                    } else {
                        all_coords.insert(new_coord);
                        // println!("adding new: {:?}", new_coord);
                    }
                }
            }
        }
    });

    // println!("duplicate_coords: {:?}", duplicate_coords);
    duplicate_coords.len()
}

pub fn part2(input: String) -> usize {
    let mut all_coords: HashSet<(usize, usize)> = HashSet::new();
    let mut duplicate_coords: HashSet<(usize, usize)> = HashSet::new();
    input.lines().for_each(|l| {
        let line = l
            .split(" -> ")
            .flat_map(|p| p.split(","))
            .map(|s| s.trim().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let x1 = line[0];
        let y1 = line[1];
        let x2 = line[2];
        let y2 = line[3];

        let mut x = x1;
        let mut y = y1;
        loop {
            let new_coord = (x, y);
            // println!("new_coord: {:?}", new_coord);
            if all_coords.contains(&new_coord) {
                duplicate_coords.insert(new_coord);
                // println!("duplicate: {:?}", new_coord);
            } else {
                all_coords.insert(new_coord);
                // println!("adding new: {:?}", new_coord);
            }

            if new_coord == (x2, y2) {
                break;
            }

            if x1 > x2 {
                x -= 1
            } else if x1 < x2 {
                x += 1
            }

            if y1 > y2 {
                y -= 1
            } else if y1 < y2 {
                y += 1
            }
        }
    });

    duplicate_coords.len()
}

#[test]
fn test_part_1() {
    let input = "0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2";

    assert_eq!(part1(input.to_string()), 5);
}

#[test]
fn test_part_2() {
    let input = "0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2";

    assert_eq!(part2(input.to_string()), 12);
}
