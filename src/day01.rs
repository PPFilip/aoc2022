use std::cmp::max;
use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn part1(input_file: &str) -> u32 {
    let file = File::open(input_file).unwrap();
    let mut elf_sum = 0_u32;
    let mut elf_max = 0_u32;
    for l in io::BufReader::new(file).lines() {
        let line = l.unwrap();
        if line == "" {
            elf_max = max(elf_sum, elf_max);
            elf_sum = 0;
        } else {
            elf_sum += line.parse::<u32>().unwrap();
        }
    }
    elf_max
}

pub fn part2(input_file: &str) -> u32 {
    let file = File::open(input_file).unwrap();
    let mut elf_sum = 0_u32;
    let mut elf_vec = Vec::new();

    for l in io::BufReader::new(file).lines() {
        let line = l.unwrap();
        if line == "" {
            elf_vec.push(elf_sum);
            elf_sum = 0;
        } else {
            elf_sum += line.parse::<u32>().unwrap();
        }
    }

    elf_vec.sort();
    elf_vec.reverse();

    let mut elf_sum = 0_u32;
    for v in &elf_vec[0..3] {
        elf_sum += v;
    }
    elf_sum
}

fn main() {
    let p1 = part1("input/day01.txt");
    println!("Result 1 - {p1:?}");
    let p2 = part2("input/day01.txt");
    println!("Result 2 - {p2:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1("input/day01_example.txt"), 24000)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input/day01.txt"), 67658)
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2("input/day01_example.txt"), 45000)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input/day01.txt"), 200158)
    }

}
