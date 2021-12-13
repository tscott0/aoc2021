use std::collections::HashSet;

use aoc2021::utils;

fn main() {
    let input = utils::string_from_file("src/bin/day13/input");

    println!("PART 1: {:?}", part1(input.clone()));
    part2(input);
}

pub fn part1(input: String) -> usize {
    let mut coordinates: HashSet<(usize, usize)> = HashSet::new();
    let mut folds: Vec<(char, usize)> = vec![];
    input.lines().map(|l| l.trim()).for_each(|l| {
        if l.starts_with("fold along ") {
            let l = l.replace("fold along ", "");
            let mut fold_iter = l.split("=");
            let axis = fold_iter.next().unwrap().chars().nth(0).unwrap();
            let fold_at = fold_iter.next().unwrap().parse::<usize>().unwrap();

            folds.push((axis, fold_at));
        } else if l.contains(",") {
            let mut coord_iter = l.split(',');
            let x = coord_iter.next().unwrap().parse::<usize>().unwrap();
            let y = coord_iter.next().unwrap().parse::<usize>().unwrap();

            coordinates.insert((x, y));
        }
    });

    let (axis, fold_at) = folds[0];
    fold_paper(&mut coordinates, axis, fold_at);
    // print(coordinates.clone());

    coordinates.len()
}

pub fn part2(input: String) {
    let mut coordinates: HashSet<(usize, usize)> = HashSet::new();
    let mut folds: Vec<(char, usize)> = vec![];
    input.lines().map(|l| l.trim()).for_each(|l| {
        if l.starts_with("fold along ") {
            let l = l.replace("fold along ", "");
            let mut fold_iter = l.split("=");
            let axis = fold_iter.next().unwrap().chars().nth(0).unwrap();
            let fold_at = fold_iter.next().unwrap().parse::<usize>().unwrap();

            folds.push((axis, fold_at));
        } else if l.contains(",") {
            let mut coord_iter = l.split(',');
            let x = coord_iter.next().unwrap().parse::<usize>().unwrap();
            let y = coord_iter.next().unwrap().parse::<usize>().unwrap();

            coordinates.insert((x, y));
        }
    });
    // print(coordinates.clone());

    folds.iter().for_each(|(axis, fold_at)| {
        fold_paper(&mut coordinates, *axis, *fold_at);
    });

    print(coordinates.clone());
}

fn fold_paper(coordinates: &mut HashSet<(usize, usize)>, axis: char, fold_at: usize) {
    coordinates
        .clone()
        .iter()
        .filter(|(x, y)| {
            if axis == 'x' && *x == fold_at {
                false
            } else if axis == 'y' && *y == fold_at {
                false
            } else {
                true
            }
        })
        .for_each(|coord| {
            let (x, y) = coord.clone();
            let new_x = if axis == 'x' && x > fold_at {
                fold_at - (x - fold_at)
            } else {
                x
            };

            let new_y = if axis == 'y' && y > fold_at {
                fold_at - (y - fold_at)
            } else {
                y
            };

            coordinates.remove(&coord);
            coordinates.insert((new_x, new_y));
        });
}

fn print(coordinates: HashSet<(usize, usize)>) {
    let mut mx = 0;
    let mut my = 0;

    for (x, y) in coordinates.clone() {
        mx = x.max(mx);
        my = y.max(my);
    }

    for y in 0..=my {
        for x in 0..=mx {
            let needle = (x, y);
            if coordinates.contains(&needle) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

#[test]
fn test_part_1() {
    let input = "6,10
    0,14
    9,10
    0,3
    10,4
    4,11
    6,0
    6,12
    4,1
    0,13
    10,12
    3,4
    3,0
    8,4
    1,10
    2,14
    8,10
    9,0
    
    fold along y=7
    fold along x=5";

    assert_eq!(part1(input.to_string()), 17);
}
