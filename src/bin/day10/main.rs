use std::collections::{HashMap, HashSet};

use aoc2021::utils;
use itertools::Itertools;

fn main() {
    let input = utils::string_from_file("src/bin/day10/input");

    println!("PART 1: {:?}", part1(input.clone()));
    println!("PART 2: {:?}", part2(input));
}

pub fn part1(input: String) -> usize {
    let scores = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let opens = HashSet::from(['(', '[', '{', '<']);
    let pairs = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
    input
        .lines()
        .map(|l| l.trim())
        .filter_map(|l| {
            let mut stack: Vec<char> = vec![];
            let first_corrupted = l.chars().find(|&c| {
                if opens.contains(&c) {
                    stack.push(c);
                    return false;
                }
                if let Some(&last) = stack.last() {
                    if pairs.get(&c).unwrap().clone() == last {
                        stack.pop();
                        false
                    } else {
                        // println!("corrupted {:?} in line: {:?}", c, l);
                        true
                    }
                } else {
                    false
                }
            });

            first_corrupted
        })
        .map(|c| scores.get(&c).unwrap())
        .sum()
}

pub fn part2(input: String) -> usize {
    let scores = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);
    let opens = HashSet::from(['(', '[', '{', '<']);
    let close_to_open = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
    let line_scores: Vec<usize> = input
        .lines()
        .map(|l| l.trim())
        .filter_map(|l| {
            let mut stack: Vec<char> = vec![];
            let first_corrupted = l.chars().find(|&c| {
                if opens.contains(&c) {
                    stack.push(c);
                    return false;
                }
                if let Some(&last) = stack.last() {
                    if close_to_open.get(&c).unwrap().clone() == last {
                        stack.pop();
                        false
                    } else {
                        // println!("corrupted {:?} in line: {:?}", c, l);
                        true
                    }
                } else {
                    false
                }
            });

            if first_corrupted.is_none() {
                Some(stack)
            } else {
                None
            }
        })
        .map(|stack| {
            stack.iter().rev().fold(0, |mut acc, x| {
                acc *= 5;
                acc += scores.get(x).unwrap();
                acc
            })
        })
        .sorted()
        .collect::<Vec<_>>();

    println!("line_scores {:?}", line_scores);

    *line_scores.get(line_scores.len() / 2).unwrap()
}

#[test]
fn test_part_1() {
    let input = "[({(<(())[]>[[{[]{<()<>>
                 [(()[<>])]({[<{<<[]>>(
                 {([(<{}[<>[]}>{[]{[(<()>
                 (((({<>}<{<{<>}{[]{[]{}
                 [[<[([]))<([[{}[[()]]]
                 [{[{({}]{}}([{[{{{}}([]
                 {<[[]]>}<{[{[{[]{()[[[]
                 [<(<(<(<{}))><([]([]()
                 <{([([[(<>()){}]>(<<{{
                 <{([{{}}[<[[[<>{}]]]>[]]";

    assert_eq!(part1(input.to_string()), 26397);
}

#[test]
fn test_part_2() {
    let input = "[({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]";

    assert_eq!(part2(input.to_string()), 288957);
}
