use aoc2021::utils;

fn main() {
    let input = utils::string_from_file("src/bin/day03/input");

    println!("PART 1");
    part1(input.clone(), 12);

    println!("PART 2");
    part2(input);
}

pub fn part1(input: String, width: usize) -> usize {
    let mut gamma_string: Vec<char> = vec![];
    let mut epsilon_string: Vec<char> = vec![];

    input
        .lines()
        .map(|x| x.trim().chars())
        .fold(vec![0; width], |mut acc, chars| {
            // println!("chars: {:?}", chars);
            chars.enumerate().for_each(|(i, value)| {
                if value == '0' {
                    acc[i] -= 1
                } else {
                    acc[i] += 1
                }
            });
            acc
        })
        .iter()
        .for_each(|&d| {
            gamma_string.push(if d > 0 { '1' } else { '0' });
            epsilon_string.push(if d <= 0 { '1' } else { '0' });
        });

    let gamma = usize::from_str_radix(gamma_string.iter().collect::<String>().as_str(), 2).unwrap();
    println!("gamma_string: {:?}", gamma_string);
    println!("gamma: {:?}", gamma);

    let epsilon =
        usize::from_str_radix(epsilon_string.iter().collect::<String>().as_str(), 2).unwrap();
    println!("epsilon_string: {:?}", epsilon_string);
    println!("epsilon: {:?}", epsilon);

    let answer = gamma * epsilon;
    println!("Answer: {:?}", answer);

    answer
}

pub fn part2(input: String) -> usize {
    let mut ogr_vec: Vec<String> = vec![];
    let mut csr_vec: Vec<String> = vec![];
    input.lines().map(|x| x.trim()).for_each(|s| {
        // let s = x.collect::<String>();
        ogr_vec.push(s.to_string());
        csr_vec.push(s.to_string());
    });

    let mut cursor: usize = 0;
    // println!("ogr_vec: {:?}", ogr_vec);
    let ogr = loop {
        reduce_ogr(&mut ogr_vec, cursor);
        // println!("ogr_vec: {:?}", ogr_vec);

        if ogr_vec.iter().count() == 1 {
            break usize::from_str_radix(ogr_vec[0].as_str(), 2).unwrap();
        }

        cursor += 1;
    };
    println!("ogr: {:?}", ogr);

    let mut cursor: usize = 0;
    // println!("csr_vec: {:?}", ogr_vec);
    let csr = loop {
        reduce_csr(&mut csr_vec, cursor);
        // println!("csr_vec: {:?}", ogr_vec);

        if csr_vec.iter().count() == 1 {
            break usize::from_str_radix(csr_vec[0].as_str(), 2).unwrap();
        }

        cursor += 1;
    };
    println!("csr: {:?}", csr);

    let answer = ogr * csr;
    println!("Answer: {:?}", answer);

    answer
}

pub fn reduce_ogr(ogr_vec: &mut Vec<String>, cursor: usize) {
    let mut count = 0;
    ogr_vec.iter().for_each(|s| {
        let value = s.chars().nth(cursor).unwrap();
        if value == '0' {
            count -= 1
        } else {
            count += 1
        }
    });

    let value = if count >= 0 { 1 } else { 0 };

    ogr_vec.retain(|row| row.chars().nth(cursor).unwrap().to_string() == value.to_string());
}

pub fn reduce_csr(csr_vec: &mut Vec<String>, cursor: usize) {
    let mut count = 0;
    csr_vec.iter().for_each(|s| {
        let value = s.chars().nth(cursor).unwrap();
        if value == '0' {
            count -= 1
        } else {
            count += 1
        }
    });

    let value = if count >= 0 { 0 } else { 1 };

    csr_vec.retain(|row| row.chars().nth(cursor).unwrap().to_string() == value.to_string());
}

#[test]
fn test_part_1() {
    let input = "00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010";

    assert_eq!(part1(input.to_string(), 5), 198);
}

#[test]
fn test_part_2() {
    let input = "00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010";

    assert_eq!(part2(input.to_string()), 230);
}
