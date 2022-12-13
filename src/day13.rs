use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::Read;
use std::str::FromStr;
use std::string::ParseError;
use itertools::Itertools;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
enum Packet {
    Packet(Vec<Packet>),
    Value(u8),
}

impl FromStr for Packet {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let packet: Packet = serde_json::from_str(line).unwrap();

        Ok(packet)
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Value(s), Packet::Value(o)) => s.cmp(o),
            (Packet::Value(n), Packet::Packet(_)) => Packet::Packet(vec![Packet::Value(*n)]).cmp(other),
            (Packet::Packet(_), Packet::Value(n)) => self.cmp(&Packet::Packet(vec![Packet::Value(*n)])),
            (Packet::Packet(left), Packet::Packet(right)) => {
                for i in 0.. left.len().min(right.len()) {
                    match left[i].cmp(&right[i]) {
                        Ordering::Less => {
                            return Ordering::Less
                        },
                        Ordering::Greater => {
                            return Ordering::Greater
                        }
                        _ => {}
                    }
                }
                left.len().cmp(&right.len())
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(input_file: &str) -> String {
    let file = File::open(input_file).unwrap();
    let mut result = "".to_string();
    let mut _lc = io::BufReader::new(file).read_to_string(&mut result).unwrap();

    result

    // TODO:  learn how to correctly pass this result
    // let signal_pairs: Vec<&str> = result.split("\n\n").collect();
    //
    // signal_pairs
}

pub fn part1(input_file: &str) -> usize {
    let input = parse_input(input_file);
    let signal_pairs: Vec<&str> = input.split("\n\n").collect();

    let mut res: usize = 0;
    for (i, sp) in signal_pairs.iter().enumerate() {
        let (left, right) = sp.split_once('\n').unwrap();
        let left: Packet = left.parse().unwrap();
        let right: Packet = right.parse().unwrap();


        if left < right {
            res += i + 1;
        }
    }

    res
}


pub fn part2(input_file: &str) -> usize {
    let input = parse_input(input_file);
    let signal_pairs: Vec<&str> = input.split("\n\n").collect();

    let mut packets:Vec<Packet> = Vec::new();
    let div_left: Packet = "[[2]]".parse().unwrap();
    let div_right: Packet = "[[6]]".parse().unwrap();
    packets.push(div_left.clone());
    packets.push(div_right.clone());

    for (_i, sp) in signal_pairs.iter().enumerate() {
        let (left, right) = sp.split_once('\n').unwrap();
        let left: Packet = left.parse().unwrap();
        let right: Packet = right.parse().unwrap();

        packets.push(left);
        packets.push(right);
    }

    packets.sort();

    let (pos_left, _ll) = packets.iter().find_position(|&p| p == &div_left).unwrap();
    let (pos_right, _rr) = packets.iter().find_position(|&p| p == &div_right).unwrap();

    (pos_left+1)*(pos_right+1)
}

fn main() {
    let p1 = part1("input/day13.txt");
    println!("Result 1 - {p1:?}");
    let p2 = part2("input/day13.txt");
    println!("Result 2 - {p2:?}");
}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1("input/day13_example.txt"), 13)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input/day13.txt"), 5340)
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2("input/day13_example.txt"), 140)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input/day13.txt"), 21276)
    }

}