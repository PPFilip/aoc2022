#![allow(dead_code)]
use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn part1(input_file: &str) -> Result<u32, ()> {
    let file = File::open(input_file).unwrap();

    for l in io::BufReader::new(file).lines() {
        let line = l.unwrap();
        let mut v: Vec<char> = Vec::new();
        let mut pos = 0_u32;
        for c in line.chars() {
            pos += 1;
            v.push(c);
            if v.len() >= 4 {
                if v.len() == 5 {
                    v.remove(0);
                }

                let mut vv = v.clone();
                vv.sort();
                vv.dedup();
                if vv.len() == 4 {
                    return Ok(pos);
                }
            }
        }
    }

    return Err(());
}

pub fn part2(input_file: &str) -> Result<u32, ()> {
        let file = File::open(input_file).unwrap();

    for l in io::BufReader::new(file).lines() {
        let line = l.unwrap();
        let mut v: Vec<char> = Vec::new();
        let mut pos = 0_u32;
        for c in line.chars() {
            pos += 1;
            v.push(c);
            if v.len() >= 14 {
                if v.len() == 15 {
                    v.remove(0);
                }

                let mut vv = v.clone();
                vv.sort();
                vv.dedup();
                if vv.len() == 14 {
                    return Ok(pos);
                }
            }
        }
    }

    return Err(());
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1("input/day06_example.txt"), Ok(7))
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input/day06.txt"), Ok(1651))
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2("input/day06_example.txt"), Ok(19))
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input/day06.txt"), Ok(3837))
    }

}