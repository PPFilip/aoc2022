#![allow(dead_code)]

use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn part1(input_file: &str) -> u32 {
    let file = File::open(input_file).unwrap();
    let mut points = 0_u32;
    let mut common_chars = Vec::new();

    for l in io::BufReader::new(file).lines() {
        let line = l.unwrap();
        let (half1, half2) = line.split_at(line.len()/2);

        for c in half1.chars() {
            if half2.contains(c) {
                common_chars.push(c);
                break;
            }
        }
    }

    for c in common_chars {
        let val = if c.is_lowercase() {
            c as u8 - 96
        } else {
            c as u8 - 64 + 26
        } as u32;

        points += val;
    }

    points
}

pub fn part2(input_file: &str) -> u32 {
    let file = File::open(input_file).unwrap();
    let mut points = 0_u32;
    let mut common_chars = Vec::new();

    let mut lc = 0_u8;
    let mut group = Vec::new();

    for l in io::BufReader::new(file).lines() {
        lc += 1;
        let line = l.unwrap();
        group.push(line);

        if lc == 3 {
            for c in group.get(0).unwrap().chars() {
                if group.get(1).unwrap().contains(c) && group.get(2).unwrap().contains(c) {
                    common_chars.push(c);
                    break;
                }
            }

            lc = 0;
            group.clear();
        }

    }

    for c in common_chars {
        let val = if c.is_lowercase() {
            c as u8 - 96
        } else {
            c as u8 - 64 + 26
        } as u32;

        points += val;
    }

    points
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1("input/day03_example.txt"), 157)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input/day03.txt"), 7597)
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2("input/day03_example.txt"), 70)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input/day03.txt"), 2607)
    }

}