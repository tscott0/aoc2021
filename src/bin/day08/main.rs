use std::collections::{HashMap, HashSet};

use aoc2021::utils;
use itertools::Itertools;

fn main() {
    let input = utils::string_from_file("src/bin/day08/input");

    println!("PART 1: {:?}", part1(input.clone()));
    println!("PART 2: {:?}", part2(input));
}

pub fn part1(input: String) -> usize {
    input
        .lines()
        .flat_map(|l| l.split(" | ").nth(1).unwrap().split_whitespace())
        .filter(|w| w.len() == 2 || w.len() == 3 || w.len() == 4 || w.len() == 7)
        .count()
}

pub fn part2(input: String) -> usize {
    let lines = input.lines().map(|l| {
        l.split(" | ")
            .flat_map(|p| p.split_whitespace().collect::<Vec<_>>())
            .collect::<Vec<_>>()
    });

    let letters = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    let mut valid_sets: HashMap<Vec<usize>, usize> = HashMap::new();
    valid_sets.insert(vec![0, 1, 2, 4, 5, 6], 0);
    valid_sets.insert(vec![2, 5], 1);
    valid_sets.insert(vec![0, 2, 3, 4, 6], 2);
    valid_sets.insert(vec![0, 2, 3, 5, 6], 3);
    valid_sets.insert(vec![1, 2, 3, 5], 4);
    valid_sets.insert(vec![0, 1, 3, 5, 6], 5);
    valid_sets.insert(vec![0, 1, 3, 4, 5, 6], 6);
    valid_sets.insert(vec![0, 2, 5], 7);
    valid_sets.insert(vec![0, 1, 2, 3, 4, 5, 6], 8);
    valid_sets.insert(vec![0, 1, 2, 3, 5, 6], 9);

    lines
        .map(|l| {
            // println!("l: {:?}", l);

            let mut letter_map: HashMap<char, HashSet<usize>> = HashMap::new();
            for c in ['a', 'b', 'c', 'd', 'e', 'f', 'g'] {
                letter_map.insert(c, HashSet::from([0, 1, 2, 3, 4, 5, 6, 7]));
            }

            letters
                .iter()
                .permutations(7)
                .map(|perm| {
                    perm.iter()
                        .enumerate()
                        .map(|(i, &c)| (*c, i))
                        .collect::<HashMap<char, usize>>()
                })
                .find_map(|key_map| {
                    let mut numbers: Vec<usize> = vec![];
                    let all_good = l.clone().into_iter().all(|w| {
                        if let Some(v) = valid(&valid_sets, &key_map, w) {
                            numbers.push(v);
                            true
                        } else {
                            false
                        }
                    });

                    if all_good {
                        Some(
                            numbers
                                .iter()
                                .rev()
                                .take(4)
                                .rev()
                                .map(|&x| x.to_string())
                                .collect::<String>()
                                .parse::<usize>()
                                .unwrap(),
                        )
                    } else {
                        None
                    }
                })
                .unwrap()
        })
        .sum()
}

// Returns Some(usize) if applying the cipher to the word is a valid arrangement of
// segments. Returns None otherwise.
pub fn valid(
    valid_segments: &HashMap<Vec<usize>, usize>,
    cipher_key: &HashMap<char, usize>,
    word: &str,
) -> Option<usize> {
    let needle = word
        .chars()
        .map(|c| cipher_key.get(&c).unwrap().clone())
        .sorted()
        .collect::<Vec<_>>();

    if let Some(n) = valid_segments.get(&needle) {
        Some(*n)
    } else {
        None
    }
}

#[test]
fn test_part_1_and_2() {
    let input =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    assert_eq!(part1(input.to_string()), 26);
    assert_eq!(part2(input.to_string()), 61229);
}

#[test]
fn test_part_2_lines() {
    assert_eq!(
        part2(
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"
                .to_string()
        ),
        8394
    );
    assert_eq!(
        part2(
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc"
                .to_string()
        ),
        9781
    );
}
