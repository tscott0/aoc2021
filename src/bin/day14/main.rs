use std::collections::{HashMap, HashSet};

use aoc2021::utils;
use itertools::Itertools;

fn main() {
    let input = utils::string_from_file("src/bin/day14/input");

    println!("PART 1: {:?}", part1(input.clone(), 10));
    // println!("PART 2: {:?}", part2(input.clone(), 40));
}

pub fn part1(input: String, steps: usize) -> usize {
    let mut line_iter = input.lines().map(|l| l.trim());
    let mut template: Vec<_> = line_iter.next().unwrap().chars().collect();

    let rules: HashMap<(char, char), char> = line_iter
        .skip(1)
        .map(|l| {
            let mut rule_iter = l.split(" -> ");
            let pair = rule_iter.next().unwrap().chars().collect::<Vec<_>>();
            let replacement = rule_iter.next().unwrap().chars().nth(0).unwrap();
            ((pair[0], pair[1]), replacement)
        })
        .collect();

    println!("rules: {:?}", rules);

    for _ in 0..steps {
        let replacements = template
            .windows(2)
            .enumerate()
            .filter_map(|(i, c)| {
                let needle = (c[0], c[1]);
                if let Some(r) = rules.get(&needle) {
                    Some((i + 1, r))
                } else {
                    None
                }
            })
            .sorted_by(|a, b| Ord::cmp(&b.0, &a.0))
            .collect::<Vec<_>>();

        // println!("replacements: {:?}", replacements);

        for (i, r) in replacements {
            template.insert(i, *r)
        }

        // println!("template: {:?}", template);
        println!("template.len(): {:?}", template.len());
    }

    let counts = template
        .iter()
        .fold(HashMap::<char, usize>::new(), |mut acc, c| {
            let v = acc.entry(*c).or_insert(0);
            *v += 1;
            acc
        });

    println!("counts: {:?}", counts);

    let counts = counts.values().sorted().collect::<Vec<_>>();

    let least_common_quantity = **counts.first().unwrap();
    let most_common_quantity = **counts.last().unwrap();

    most_common_quantity - least_common_quantity
}

fn walk(
    a: char,
    b: char,
    step_depth: usize,
    max_depth: usize,
    counts: &mut HashMap<char, usize>,
    rules: &HashMap<(char, char), char>,
) {
    if step_depth >= max_depth {
        return;
    }
    let needle = (a, b);

    if let Some(r) = rules.get(&needle) {
        let v = counts.entry(*r).or_insert(0);
        *v += 1;

        walk(a, *r, step_depth + 1, max_depth, counts, rules);
        walk(*r, b, step_depth + 1, max_depth, counts, rules);
    }
}

pub fn part2(input: String, steps: usize) -> usize {
    let mut line_iter = input.lines().map(|l| l.trim());
    let template: Vec<_> = line_iter.next().unwrap().chars().collect();

    let rules: HashMap<(char, char), char> = line_iter
        .skip(1)
        .map(|l| {
            let mut rule_iter = l.split(" -> ");
            let pair = rule_iter.next().unwrap().chars().collect::<Vec<_>>();
            let replacement = rule_iter.next().unwrap().chars().nth(0).unwrap();
            ((pair[0], pair[1]), replacement)
        })
        .collect();

    println!("rules: {:?}", rules);

    let mut cache: HashMap<((char, char), usize), HashMap<(char, char), usize>> = HashMap::new();

    let count_map = template
        .windows(2)
        .map(|c| walk2((c[0], c[1]), 10, &mut cache, &rules))
        .fold(HashMap::new(), |mut merged, c| {
            c.iter().for_each(|(&k, v)| {
                *merged.entry(k).or_insert(0) += v;
            });

            merged
        });

    println!("count_map: {:?}", count_map);

    let char_counts = count_map
        .iter()
        .fold(HashMap::<char, usize>::new(), |mut acc, ((a, b), n)| {
            for c in [*a, *b] {
                let v = acc.entry(c).or_insert(0);
                *v += n;
            }
            acc
        });

    println!("char_counts: {:?}", count_map);


    let counts = char_counts.values().map(|n| n / 2).sorted().collect::<Vec<_>>();

    let least_common_quantity = *counts.first().unwrap();
    let most_common_quantity = *counts.last().unwrap();

    most_common_quantity - least_common_quantity
}

fn walk2(
    pair: (char, char),
    depth: usize,
    cache: &mut HashMap<((char, char), usize), HashMap<(char, char), usize>>,
    rules: &HashMap<(char, char), char>,
) -> HashMap<(char, char), usize> {
    println!("walk2 pair: {:?} depth: {:?}", pair, depth);

    if depth == 0 {
        return HashMap::new();
    }

    let needle = (pair, depth);

    if let Some(c) = cache.get(&needle) {
        return c.clone();
    }

    let merged_map = if let Some(r) = rules.get(&pair) {
        let (a, b) = pair;
        let first_map = walk2((a, *r), depth - 1, cache, rules);
        let mut combined = walk2((*r, b), depth - 1, cache, rules);
        first_map.iter().for_each(|(&k, v)| {
            *combined.entry(k).or_insert(0) += v;
        });

        combined
    } else {
        HashMap::new()
    };

    cache.insert(needle, merged_map.clone());

    merged_map
}

#[test]
fn test_part_1() {
    let input = "NNCB

    CH -> B
    HH -> N
    CB -> H
    NH -> C
    HB -> C
    HC -> B
    HN -> C
    NN -> C
    BH -> H
    NC -> B
    NB -> B
    BN -> B
    BB -> N
    BC -> B
    CC -> N
    CN -> C";

    assert_eq!(part1(input.to_string(), 10), 1588);
}

#[test]
fn test_part_2() {
    let input = "NNCB

    CH -> B
    HH -> N
    CB -> H
    NH -> C
    HB -> C
    HC -> B
    HN -> C
    NN -> C
    BH -> H
    NC -> B
    NB -> B
    BN -> B
    BB -> N
    BC -> B
    CC -> N
    CN -> C";

    assert_eq!(part2(input.to_string(), 10), 1588);
    // assert_eq!(part2(input.to_string(), 40), 2188189693529);
}
