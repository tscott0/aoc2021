use aoc2021::utils;

fn main() {
    let input = utils::string_from_file("src/bin/day02/input");

    println!("PART 1");
    println!("Answer: {:?}", part1(input.clone()));

    println!("PART 2");
    println!("Answer: {:?}", part2(input));
}

pub fn part1(input: String) -> u32 {
    let (position, depth) = input
        .lines()
        .map(|x| {
            let mut parts = x.split_whitespace();
            let instruction = parts.next().unwrap();
            let amount = parts.next().unwrap().parse::<u32>().unwrap();

            (instruction, amount)
        })
        .fold((0, 0), |(mut pos, mut depth), (i, a)| {
            match i {
                "forward" => pos += a,
                "down" => depth += a,
                "up" => depth -= a,
                _ => {}
            }

            (pos, depth)
        });

    position * depth
}

pub fn part2(input: String) -> u32 {
    let (position, depth, _) = input
        .lines()
        .map(|x| {
            let mut parts = x.split_whitespace();
            let instruction = parts.next().unwrap();
            let amount = parts.next().unwrap().parse::<u32>().unwrap();

            (instruction, amount)
        })
        .fold((0, 0, 0), |(mut pos, mut depth, mut aim), (i, amount)| {
            match i {
                "down" => aim += amount,
                "up" => aim -= amount,
                "forward" => {
                    pos += amount;
                    depth += aim * amount;
                }
                _ => {}
            }

            (pos, depth, aim)
        });

    position * depth
}

#[test]
fn test_part_1() {
    let input = "forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2";

    assert_eq!(part1(input.to_string()), 150);
}

#[test]
fn test_part_2() {
    let input = "forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2";

    assert_eq!(part2(input.to_string()), 900);
}
