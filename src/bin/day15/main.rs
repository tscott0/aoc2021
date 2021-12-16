use std::{collections::HashMap, time::Instant};

use aoc2021::utils;
use petgraph::{
    graph::{NodeIndex, UnGraph},
    visit::EdgeRef,
};

fn main() {
    let input = utils::string_from_file("src/bin/day15/input");

    println!("PART 1: {:?}", part1(input.clone()));

    let start = Instant::now();
    println!("PART 2: {:?}", part2(input.clone()));
    let duration = start.elapsed();
    println!("PART 2 solved in {:?}", duration);
}

fn part1(input: String) -> usize {
    let mut graph: UnGraph<u8, u8> = UnGraph::new_undirected();

    let grid = input
        .lines()
        .map(|l| l.trim())
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let width = grid.first().unwrap().len();
    let height = grid.len();

    let mut node_index_map: HashMap<(usize, usize), NodeIndex> = HashMap::new();
    for y in 0..width {
        for x in 0..height {
            let current = grid[y][x];
            let current_node = graph.add_node(current);

            node_index_map.insert((x, y), current_node);
        }
    }

    let start_node = node_index_map.get(&(0, 0)).unwrap();
    let destination_node = node_index_map.get(&(width - 1, height - 1)).unwrap();

    for y in 0..width {
        for x in 0..height {
            // let current = grid[y][x];
            let current_node = *node_index_map.get(&(x, y)).unwrap();
            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .iter()
                .for_each(|(x_off, y_off)| {
                    let n_x = x as isize + *x_off;
                    let n_y = y as isize + *y_off;
                    if n_x >= 0 && n_x < width as isize && n_y >= 0 && n_y < height as isize {
                        // let neighbour = grid[n_y as usize][n_x as usize];
                        let neighbour_node =
                            node_index_map.get(&(n_x as usize, n_y as usize)).unwrap();
                        graph.add_edge(current_node, *neighbour_node, 1);
                    }
                });
        }
    }

    let (cost, _) = petgraph::algo::astar(
        &graph,
        *start_node,
        |finish| finish == *destination_node,
        |e| graph.node_weight(e.target()).unwrap().clone() as usize,
        |_| 0,
    )
    .unwrap();

    // println!("path {:?}", path);
    cost
}

fn part2(input: String) -> usize {
    let mut graph: UnGraph<u8, u8> = UnGraph::new_undirected();

    let grid = input
        .lines()
        .map(|l| l.trim())
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let width = grid.first().unwrap().len();
    let height = grid.len();
    let new_width = width * 5;
    let new_height = height * 5;

    let mut new_grid: Vec<Vec<u8>> = vec![];

    for y in 0..new_height {
        let mut row: Vec<u8> = vec![];
        for x in 0..new_width {
            let x = x as usize;
            let y = y as usize;

            let x_offset = (x / width) as u8;
            let y_offset = (y / height) as u8;

            let old_x = x % width;
            let old_y = y % height;
            let old_risk = grid[old_y][old_x];

            let new_value = (old_risk as u32 + x_offset as u32 + y_offset as u32 - 1) % 9 + 1;
            row.push(new_value as u8);
        }
        new_grid.push(row)
    }

    // println!("new_grid {:?}", new_grid);

    let mut node_index_map: HashMap<(usize, usize), NodeIndex> = HashMap::new();
    for y in 0..new_height {
        for x in 0..new_width {
            let current = new_grid[y][x];
            let current_node = graph.add_node(current);

            node_index_map.insert((x, y), current_node);
        }
    }

    let start_node = node_index_map.get(&(0, 0)).unwrap();
    let destination_node = node_index_map
        .get(&(new_width - 1, new_height - 1))
        .unwrap();

    for y in 0..new_height {
        for x in 0..new_width {
            // let current = grid[y][x];
            let current_node = *node_index_map.get(&(x, y)).unwrap();
            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .iter()
                .for_each(|(x_off, y_off)| {
                    let n_x = x as isize + *x_off;
                    let n_y = y as isize + *y_off;
                    if n_x >= 0 && n_x < new_width as isize && n_y >= 0 && n_y < new_height as isize
                    {
                        // let neighbour = grid[n_y as usize][n_x as usize];
                        let neighbour_node =
                            node_index_map.get(&(n_x as usize, n_y as usize)).unwrap();
                        graph.add_edge(current_node, *neighbour_node, 1);
                    }
                });
        }
    }

    let (cost, _) = petgraph::algo::astar(
        &graph,
        *start_node,
        |finish| finish == *destination_node,
        |e| graph.node_weight(e.target()).unwrap().clone() as usize,
        |_| 0,
    )
    .unwrap();

    // for p in path {
    //     println!("p {:?}", graph.node_weight(p).unwrap());
    // }

    cost
}

#[test]
fn test_part_1() {
    let input = "1163751742
    1381373672
    2136511328
    3694931569
    7463417111
    1319128137
    1359912421
    3125421639
    1293138521
    2311944581";

    assert_eq!(part1(input.to_string()), 40);
    assert_eq!(part2(input.to_string()), 315);
}
