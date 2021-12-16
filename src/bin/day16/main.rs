use aoc2021::utils;

fn main() {
    let input = utils::string_from_file("src/bin/day16/input");

    println!("PART 1: {:?}", part1(input.clone()));
    println!("PART 2: {:?}", part2(input.clone()));
}

fn part1(input: String) -> usize {
    let bits: Vec<_> = bit_chars(input);

    let mut bit_iterator = bits.iter();
    let mut bit_count: usize = 0;

    let (top_packet, _) = parse_packet(&mut bit_iterator, &mut bit_count);
    println!("{:?}", top_packet);

    top_packet
}

fn part2(input: String) -> usize {
    let bits: Vec<_> = bit_chars(input);

    let mut bit_iterator = bits.iter();
    let mut bit_count: usize = 0;

    let (_, total) = parse_packet(&mut bit_iterator, &mut bit_count);
    println!("{:?}", total);

    total
}

fn bit_chars(input: String) -> Vec<char> {
    input
        .lines()
        .next()
        .unwrap()
        .trim()
        .chars()
        .flat_map(|h| match h {
            '0' => ['0', '0', '0', '0'],
            '1' => ['0', '0', '0', '1'],
            '2' => ['0', '0', '1', '0'],
            '3' => ['0', '0', '1', '1'],
            '4' => ['0', '1', '0', '0'],
            '5' => ['0', '1', '0', '1'],
            '6' => ['0', '1', '1', '0'],
            '7' => ['0', '1', '1', '1'],
            '8' => ['1', '0', '0', '0'],
            '9' => ['1', '0', '0', '1'],
            'A' => ['1', '0', '1', '0'],
            'B' => ['1', '0', '1', '1'],
            'C' => ['1', '1', '0', '0'],
            'D' => ['1', '1', '0', '1'],
            'E' => ['1', '1', '1', '0'],
            'F' => ['1', '1', '1', '1'],
            _ => panic!("Unexpected character"),
        })
        .collect::<Vec<_>>()
}

fn parse_packet(b: &mut std::slice::Iter<char>, bit_count: &mut usize) -> (usize, usize) {
    let v = parse_version(b, bit_count);
    let t = parse_type(b, bit_count);

    let mut version_sum = v as usize;

    let total = match t {
        4 => {
            let mut nibbles: Vec<usize> = vec![];
            loop {
                let last = parse_n(b, 1, bit_count) == 0;
                let current = parse_n(b, 4, bit_count) as usize;

                nibbles.push(current);

                if last {
                    break;
                }
            }

            // use bit-shifting to "grow" the 4-bit groups
            nibbles
                .iter()
                .rev()
                .enumerate()
                .fold(0, |acc, (i, r)| acc + (r << (i * 4)))
        }
        op => {
            let length_type = parse_n(b, 1, bit_count);

            let mut packets: Vec<usize> = vec![];
            match length_type {
                0 => {
                    let total_length = parse_n(b, 15, bit_count);
                    // println!("parsing {:?} bits", total_length);

                    let start_bit_count = bit_count.clone();
                    loop {
                        let (p, v) = parse_packet(b, bit_count);
                        version_sum += p;
                        packets.push(v);

                        if *bit_count - start_bit_count == total_length as usize {
                            break;
                        }
                    }
                }
                1 => {
                    let sub_packets = parse_n(b, 11, bit_count);
                    // println!("parsing {:?} packets", sub_packets);

                    (0..sub_packets)
                        .map(|_| parse_packet(b, bit_count))
                        .for_each(|(v, p)| {
                            version_sum += v;
                            packets.push(p);
                        });
                }
                a => panic!("Unexpected length type: {:?}", a),
            }

            match op {
                0 => packets.iter().sum::<usize>(),
                1 => packets.iter().product::<usize>(),
                2 => packets.iter().min().unwrap().clone(),
                3 => packets.iter().max().unwrap().clone(),
                5 => {
                    if packets[0] > packets[1] {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if packets[0] < packets[1] {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if packets[0] == packets[1] {
                        1
                    } else {
                        0
                    }
                }
                a => panic!("Unexpected op code: {:?}", a),
            }
        }
    };

    // println!("packet_sum: {:?}", packet_sum);
    (version_sum, total)
}

fn parse_version(bit_iterator: &mut std::slice::Iter<char>, bit_count: &mut usize) -> u32 {
    parse_n(bit_iterator, 3, bit_count)
}

fn parse_type(bit_iterator: &mut std::slice::Iter<char>, bit_count: &mut usize) -> u32 {
    parse_n(bit_iterator, 3, bit_count)
}

fn parse_n(bit_iterator: &mut std::slice::Iter<char>, count: usize, bit_count: &mut usize) -> u32 {
    // println!("{:?} bits", count);
    *bit_count += count;
    let s = bit_iterator.take(count).map(|c| *c).collect::<Vec<_>>();
    u32_from_binary(s)
}

fn u32_from_binary(s: Vec<char>) -> u32 {
    let s: String = s.into_iter().collect();
    let n = isize::from_str_radix(s.as_str(), 2).unwrap() as u32;
    // println!("{:} = {:?}", s, n);
    n
}

#[test]
fn test_part_1() {
    // Examples derived from the text
    assert_eq!(part1("D2FE28".to_string()), 6);
    assert_eq!(part1("38006F45291200".to_string()), 9);
    assert_eq!(part1("EE00D40C823060".to_string()), 14);

    // Proper examples
    assert_eq!(part1("8A004A801A8002F478".to_string()), 16);
    assert_eq!(part1("620080001611562C8802118E34".to_string()), 12);
    assert_eq!(part1("C0015000016115A2E0802F182340".to_string()), 23);
    assert_eq!(part1("A0016C880162017C3686B18A3D4780".to_string()), 31);
}

#[test]
fn test_part_2() {
    assert_eq!(part2("C200B40A82".to_string()), 3);
    assert_eq!(part2("04005AC33890".to_string()), 54);
    assert_eq!(part2("880086C3E88112".to_string()), 7);
    assert_eq!(part2("CE00C43D881120".to_string()), 9);
    assert_eq!(part2("D8005AC2A8F0".to_string()), 1);
    assert_eq!(part2("F600BC2D8F".to_string()), 0);
    assert_eq!(part2("9C005AC2F8F0".to_string()), 0);
    assert_eq!(part2("9C0141080250320F1802104A08".to_string()), 1);
}
