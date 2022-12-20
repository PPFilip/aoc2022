use std::fs::File;
use std::{io};
use std::io::{BufRead};

const DECRYPTION_KEY:isize = 811589153;

fn parse_input(input_file: &str) -> Vec<isize> {
    let file = File::open(input_file).unwrap();
    // time to start using map consistently XD
    let res : Vec<isize> = io::BufReader::new(file).lines().map(|l| l.unwrap().parse().unwrap()).collect();
    res
}

fn mixer(arr: &Vec<isize>, count: usize) -> Vec<isize> {
    let mut positions: Vec<usize> = (0..arr.len()).collect();
    let wrap = arr.len()-1;

    for _ in 0..count {
        for (i, &element) in arr.iter().enumerate() {
            if element == 0 {
                continue;
            }

            let position = positions.iter().position(|x| *x == i).unwrap();
            let tmp = positions.remove(position);

            let new_position = (position as isize + element).rem_euclid(wrap as isize) as usize;
            positions.insert(new_position, tmp);
        }
    }

    positions.iter().map(|&p| arr[p]).collect()
}

fn grove_coordinate(arr: &Vec<isize>) -> isize {
    let p0 = arr.iter().position(|&x| x==0).unwrap();
    [1000,2000,3000].iter().map(|x| arr[(p0+x) % arr.len()]).sum()
}

pub fn part1(input_file: &str) -> isize {
    let arr = parse_input(input_file);
    let res = mixer(&arr, 1);
    grove_coordinate(&res)
}


pub fn part2(input_file: &str) -> isize {
    let arr:Vec<isize> = parse_input(input_file).iter().map(|&x| x * DECRYPTION_KEY).collect();
    let res = mixer(&arr, 10);
    grove_coordinate(&res)
}


fn main() {
    let p1 = part1("input/day20_example.txt");
    println!("Result 1 - {p1:?}");
    let p2 = part2("input/day20.txt");
    println!("Result 2 - {p2:?}");
}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1("input/day20_example.txt"), 3)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input/day20.txt"), 4578)
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2("input/day20_example.txt"), 1623178306)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input/day20.txt"), 2159638736133)
    }

}