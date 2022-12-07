#![allow(dead_code)]

use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn part1(input_file: &str) -> u32 {
    let file = File::open(input_file).unwrap();
    let mut points = 0;

    let point_values = HashMap::from([
        ("X", 1),
        ("Y", 2),
        ("Z", 3)
    ]);

    for l in io::BufReader::new(file).lines() {
        let line = l.unwrap();
        let v: Vec<&str> = line.split(' ').collect();
        let our_hand = v.get(1).unwrap().to_owned();
        let _opp_hand = v.get(0).unwrap().to_owned();

        let we_win =  v == ["A", "Y"] || v == ["B", "Z"] || v == ["C", "X"];
        let we_draw =  v == ["A", "X"] || v == ["B", "Y"] || v == ["C", "Z"];

        points += point_values[our_hand];

        if we_win {
            points += 6
        } else if we_draw {
            points += 3
        }

    }

    points
}

pub fn part2(input_file: &str) -> u32 {
    let file = File::open(input_file).unwrap();
    let mut points = 0;

    let point_values = HashMap::from([
        ("X", 1),
        ("Y", 2),
        ("Z", 3)
    ]);

    let hand_map = HashMap::from([
        (("A", "X"), "Z"),
        (("A", "Y"), "X"),
        (("A", "Z"), "Y"),
        (("B", "X"), "X"),
        (("B", "Y"), "Y"),
        (("B", "Z"), "Z"),
        (("C", "X"), "Y"),
        (("C", "Y"), "Z"),
        (("C", "Z"), "X"),
    ]);

    for l in io::BufReader::new(file).lines() {
        let line = l.unwrap();
        let v: Vec<&str> = line.split(' ').collect();
        let our_choice = v.get(1).unwrap().to_owned();
        let opp_hand = v.get(0).unwrap().to_owned();

        let our_hand = hand_map[&(opp_hand, our_choice)];

        let we_win = our_choice == "Z";
        let we_draw = our_choice == "Y";

        points += point_values[our_hand];

        if we_win {
            points += 6
        } else if we_draw {
            points += 3
        }

    }

    points
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1("input/day02_example.txt"), 15)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input/day02.txt"), 11603)
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2("input/day02_example.txt"), 12)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input/day02.txt"), 12725)
    }

}
