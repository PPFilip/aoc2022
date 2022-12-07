#![allow(dead_code)]
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::Add;

pub fn part1(input_file: &str) -> String {
    let file = File::open(input_file).unwrap();
    const NUM_STACKS:usize = 9;
    //TODO: remove fixed size

    const NEW_VEC: Vec<char> = Vec::new();
    let mut stacks= [NEW_VEC; NUM_STACKS];

    let mut lnum = 0;
    for l in io::BufReader::new(file).lines() {
        lnum += 1;
        let line = l.unwrap();

        if lnum <= NUM_STACKS {
            for c in line.chars() {
                stacks[lnum-1].push(c);
            }
        } else {
            let v: Vec<&str> = line.split(' ').collect();
            let move_count:usize = v.get(1).unwrap().parse().unwrap();
            let from_stack:usize = v.get(3).unwrap().parse().unwrap();
            let to_stack:usize = v.get(5).unwrap().parse().unwrap();

            for _ in 0 .. move_count {
                let cpop = stacks[from_stack-1].pop().unwrap();
                stacks[to_stack-1].push(cpop);
            }

        }

    }

    let mut result = "".to_string();
    for mut s in stacks {
        let top_crate = s.pop().unwrap();
        result.push(top_crate);
    }

    result
}

pub fn part2(input_file: &str) -> String {
    let file = File::open(input_file).unwrap();
    const NUM_STACKS:usize = 9;

    const NEW_VEC: Vec<char> = Vec::new();
    let mut stacks= [NEW_VEC; NUM_STACKS];

    let mut lnum = 0;
    for l in io::BufReader::new(file).lines() {
        lnum += 1;
        let line = l.unwrap();

        if lnum <= NUM_STACKS {
            for c in line.chars() {
                stacks[lnum-1].push(c);
            }
        } else {
            let v: Vec<&str> = line.split(' ').collect();
            let move_count:usize = v.get(1).unwrap().parse().unwrap();
            let from_stack:usize = v.get(3).unwrap().parse().unwrap();
            let to_stack:usize = v.get(5).unwrap().parse().unwrap();

            let mut backpop: Vec<char> = Default::default();
            for _ in 0 .. move_count {
                let cpop = stacks[from_stack-1].pop().unwrap();
                backpop.push(cpop);
            }

            backpop.reverse();
            for c in backpop {
                stacks[to_stack-1].push(c);
            }

        }

    }

    let mut result = "".to_string();
    for mut s in stacks {
        let top_crate = s.pop().unwrap();
        result.push(top_crate);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    // #[test]
    // fn test_part1_example() {
    //     assert_eq!(part1("input/day05_example_parsed.txt"), "CMZ".to_string())
    // }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input/day05_parsed.txt"), "CWMTGHBDW".to_string())
    }

    // #[test]
    // fn test_part2_example() {
    //     assert_eq!(part2("input/day05_example_parsed.txt"), "MCD".to_string())
    // }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input/day05_parsed.txt"), "SSCGWJCRB".to_string())
    }

}