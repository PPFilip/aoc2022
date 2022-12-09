use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn part1(input_file: &str) -> u32 {
    let file = File::open(input_file).unwrap();
    let mut points = 0_u32;

    for l in io::BufReader::new(file).lines() {
        let line = l.unwrap();
        let v: Vec<&str> = line.split(',').collect();
        let a: Vec<&str> = v.get(0).unwrap().split('-').collect();
        let b: Vec<&str> = v.get(1).unwrap().split('-').collect();
        let ax: u32 = a.get(0).unwrap().parse().unwrap();
        let ay: u32 = a.get(1).unwrap().parse().unwrap();
        let bx: u32 = b.get(0).unwrap().parse().unwrap();
        let by: u32 = b.get(1).unwrap().parse().unwrap();

        if (ax >= bx  &&  ay <= by) || (bx >= ax && by <= ay) {
            points += 1
        }

    }

    points
}

pub fn part2(input_file: &str) -> u32 {
    let file = File::open(input_file).unwrap();
    let mut points = 0_u32;

    for l in io::BufReader::new(file).lines() {
        let line = l.unwrap();
        let v: Vec<&str> = line.split(',').collect();
        let a: Vec<&str> = v.get(0).unwrap().split('-').collect();
        let b: Vec<&str> = v.get(1).unwrap().split('-').collect();
        let ax: u32 = a.get(0).unwrap().parse().unwrap();
        let ay: u32 = a.get(1).unwrap().parse().unwrap();
        let bx: u32 = b.get(0).unwrap().parse().unwrap();
        let by: u32 = b.get(1).unwrap().parse().unwrap();

        if (bx <= ax && ax <= by) || (ax <= bx && bx <= ay) {
            points += 1
        }

    }

    points
}

fn main() {
    let p1 = part1("input/day04.txt");
    println!("Result 1 - {p1:?}");
    let p2 = part2("input/day04.txt");
    println!("Result 2 - {p2:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1("input/day04_example.txt"), 2)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input/day04.txt"), 540)
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2("input/day04_example.txt"), 4)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input/day04.txt"), 872)
    }

}