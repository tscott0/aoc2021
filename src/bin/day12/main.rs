use std::collections::{HashMap, HashSet};

use aoc2021::utils;
use petgraph::{adj::NodeIndex, dot::Dot, graph::UnGraph};

fn main() {
    let input = utils::string_from_file("src/bin/day12/input");

    println!("PART 1: {:?}", part1(input.clone()));
    println!("PART 2: {:?}", part2(input));
}

pub fn part1(input: String) -> usize {
    let mut graph: UnGraph<String, usize> = UnGraph::new_undirected();
    let mut nodes: HashSet<String> = HashSet::new();
    let mut edges: Vec<(String, String)> = vec![];

    input.lines().for_each(|l| {
        let mut line = l.trim().split('-');
        let first = line.next().unwrap().to_string();
        let second = line.next().unwrap().to_string();

        nodes.insert(first.clone());
        nodes.insert(second.clone());

        edges.push((first, second));
    });

    let nodes = nodes
        .iter()
        .map(|n| {
            let i = graph.add_node(n.clone());
            (n.clone(), i)
        })
        .collect::<HashMap<String, NodeIndex<_>>>();

    edges.iter().for_each(|(a, b)| {
        let a_index = nodes.get(a).unwrap();
        let b_index = nodes.get(b).unwrap();
        graph.add_edge(*a_index, *b_index, 1);
    });

    // println!("{:?}", Dot::new(&graph));

    let start_string = "start".to_string();
    let end_string = "end".to_string();

    let start = nodes.get(&start_string).unwrap();
    let end = nodes.get(&end_string).unwrap();

    graph.neighbors(*start);

    let mut all_paths: Vec<Vec<usize>> = vec![];
    let current_path: Vec<usize> = vec![];
    let counts: HashMap<String, u32> = HashMap::new();

    walk(&graph, *start, *end, current_path, counts, &mut all_paths);
    // println!("all_paths: {:?}", all_paths);

    all_paths.len()
}

fn walk(
    graph: &UnGraph<String, usize>,
    current: petgraph::graph::NodeIndex,
    end: petgraph::graph::NodeIndex,
    current_path: Vec<usize>,
    visited_counts: HashMap<String, u32>,
    all_paths: &mut Vec<Vec<usize>>,
) {
    let w = graph.node_weight(current).unwrap();
    let is_lower = *w == w.to_lowercase();
    let mut visited_counts = visited_counts;
    let count = visited_counts.entry(w.clone()).or_insert(0);
    *count += 1;

    // println!("visited {:?} {:?} times", w, count);
    if is_lower && *count > 1 {
        return;
    }

    let mut current_path = current_path.clone();
    current_path.push(current.index());
    // println!(
    //     "walk current: {:?} current_path: {:?}",
    //     current, current_path
    // );

    if *w == "end" {
        all_paths.push(current_path.clone());
        // current_path.clear();
        return;
    }

    graph.neighbors(current).for_each(|n| {
        // Don't walk back to start
        let neighbour_name = graph.node_weight(n).unwrap();
        if *neighbour_name != "start" {
            walk(
                graph,
                n,
                end,
                current_path.clone(),
                visited_counts.clone(),
                all_paths,
            );
        }
    });
}

pub fn part2(input: String) -> usize {
    let mut graph: UnGraph<String, usize> = UnGraph::new_undirected();
    let mut nodes: HashSet<String> = HashSet::new();
    let mut edges: Vec<(String, String)> = vec![];

    input.lines().for_each(|l| {
        let mut line = l.trim().split('-');
        let first = line.next().unwrap().to_string();
        let second = line.next().unwrap().to_string();

        nodes.insert(first.clone());
        nodes.insert(second.clone());

        edges.push((first, second));
    });

    let nodes = nodes
        .iter()
        .map(|n| {
            let i = graph.add_node(n.clone());
            (n.clone(), i)
        })
        .collect::<HashMap<String, NodeIndex<_>>>();

    edges.iter().for_each(|(a, b)| {
        let a_index = nodes.get(a).unwrap();
        let b_index = nodes.get(b).unwrap();
        graph.add_edge(*a_index, *b_index, 1);
    });

    // println!("{:?}", Dot::new(&graph));

    let start_string = "start".to_string();
    let end_string = "end".to_string();

    let start = nodes.get(&start_string).unwrap();
    let end = nodes.get(&end_string).unwrap();

    graph.neighbors(*start);

    let mut all_paths: Vec<Vec<usize>> = vec![];
    let current_path: Vec<usize> = vec![];
    let counts: HashMap<String, u32> = HashMap::new();

    walk2(
        &graph,
        *start,
        *end,
        current_path,
        counts,
        &mut all_paths,
        false,
    );
    // println!("all_paths: {:?}", all_paths);

    all_paths.len()
}

fn walk2(
    graph: &UnGraph<String, usize>,
    current: petgraph::graph::NodeIndex,
    end: petgraph::graph::NodeIndex,
    current_path: Vec<usize>,
    visited_counts: HashMap<String, u32>,
    all_paths: &mut Vec<Vec<usize>>,
    twice_visited: bool,
) {
    let mut twice_visited_this_walk = twice_visited;
    let w = graph.node_weight(current).unwrap();
    let is_lower = *w == w.to_lowercase();

    let mut visited_counts = visited_counts;
    if is_lower {
        let count = visited_counts.entry(w.clone()).or_insert(0);
        *count += 1;

        // println!("visited {:?} {:?} times", w, count);
        if twice_visited_this_walk && *count > 1 {
            return;
        }

        if *count == 2 {
            twice_visited_this_walk = true;
        }
    }

    let mut current_path = current_path.clone();
    current_path.push(current.index());

    if *w == "end" {
        all_paths.push(current_path.clone());
        // current_path.clear();
        return;
    }

    graph.neighbors(current).for_each(|n| {
        // Don't walk back to start
        let neighbour_name = graph.node_weight(n).unwrap();
        if *neighbour_name != "start" {
            walk2(
                graph,
                n,
                end,
                current_path.clone(),
                visited_counts.clone(),
                all_paths,
                twice_visited_this_walk,
            );
        }
    });
}

#[test]
fn test_part_1_and_2_example_1() {
    let input = "start-A
    start-b
    A-c
    A-b
    b-d
    A-end
    b-end";

    assert_eq!(part1(input.to_string()), 10);
    assert_eq!(part2(input.to_string()), 36);
}

#[test]
fn test_part_1_and_2_example_2() {
    let input = "dc-end
    HN-start
    start-kj
    dc-start
    dc-HN
    LN-dc
    HN-end
    kj-sa
    kj-HN
    kj-dc";

    assert_eq!(part1(input.to_string()), 19);
    assert_eq!(part2(input.to_string()), 103);
}

#[test]
fn test_part_1_and_2_example_3() {
    let input = "fs-end
    he-DX
    fs-he
    start-DX
    pj-DX
    end-zg
    zg-sl
    zg-pj
    pj-he
    RW-he
    fs-DX
    pj-RW
    zg-RW
    start-pj
    he-WI
    zg-he
    pj-fs
    start-RW";

    assert_eq!(part1(input.to_string()), 226);
    assert_eq!(part2(input.to_string()), 3509);
}
