use aoc2021::utils;
use itertools::Itertools;

fn main() {
    let input = utils::string_from_file("src/bin/day04/input");

    println!("PART 1");
    part1(input.clone());

    println!("PART 2");
    part2(input);
}

pub fn part1(input: String) -> usize {
    let mut input_iterator = input.lines();
    let draw = input_iterator
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut boards = input_iterator
        .chunks(6)
        .into_iter()
        .map(|l| {
            l.skip(1)
                .map(|row| {
                    row.split_whitespace()
                        .map(|num| (num.parse::<usize>().unwrap(), false))
                        .collect::<Vec<(usize, bool)>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // println!("Boards: {:?}", boards);

    for i in draw.clone() {
        for b in &mut boards {
            for row in b {
                for val in row {
                    if val.0 == i {
                        val.1 = true
                    };
                }
            }
        }

        for b in &mut boards {
            if let Some(score) = winning_score(b, i) {
                println!("Answer: {:?}", score);
                return score;
            }
        }
    }

    println!("Boards: {:?}", boards);

    0
}

fn winning_score(board: &mut Vec<Vec<(usize, bool)>>, last_draw: usize) -> Option<usize> {
    let mut won = false;
    // Check for winning rows first
    for row in board.iter() {
        if row.iter().all(|v| v.1) {
            // println!("winning row: {:?}", row);
            won = true;
            break;
        }
    }

    // Check for winning column
    for x in 0..=4 {
        if board.iter().map(|row| row[x]).all(|val| val.1) {
            won = true;
            break;
        }
    }

    if won == true {
        let total = board
            .iter()
            .flatten()
            .filter(|(_, matched)| !*matched)
            .map(|(v, _)| *v)
            .sum::<usize>();
        Some(total * last_draw)
    } else {
        None
    }
}

pub fn part2(input: String) -> usize {
    let mut input_iterator = input.lines();
    let draw = input_iterator
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut boards = input_iterator
        .chunks(6)
        .into_iter()
        .map(|l| {
            l.skip(1)
                .map(|row| {
                    row.split_whitespace()
                        .map(|num| (num.parse::<usize>().unwrap(), false))
                        .collect::<Vec<(usize, bool)>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // println!("Boards: {:?}", boards);

    for d in draw.clone() {
        for b in &mut boards {
            for row in b {
                for val in row {
                    if val.0 == d {
                        val.1 = true
                    };
                }
            }
        }

        let mut indexes_to_remove: Vec<usize> = vec![];
        let mut board_count = boards.len();
        for (i, b) in boards.iter_mut().enumerate() {
            if let Some(score) = winning_score(b, d) {
                if board_count == 1 {
                    println!("Answer: {:?}", score);
                    return score;
                } else {
                    indexes_to_remove.push(i);
                    board_count -= 1;
                }
            }
        }

        indexes_to_remove.sort();
        indexes_to_remove.reverse() ;

        for i in indexes_to_remove {
            boards.remove(i);
        }
    }

    println!("Boards: {:?}", boards);

    0
}

#[test]
fn test_part_1() {
    let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19
    
     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6
    
    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7";

    assert_eq!(part1(input.to_string()), 4512);
}

#[test]
fn test_part_2() {
    let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19
    
     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6
    
    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7";

    assert_eq!(part2(input.to_string()), 1924);
}
