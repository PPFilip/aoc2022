use std::fs::File;
use std::{io};
use std::cmp::max;
use std::collections::{HashSet};
use std::io::{BufRead};

type Point = (isize, isize, isize);

fn parse_input(input_file: &str) -> (Vec<Point>, Point) {
    let file = File::open(input_file).unwrap();
    let mut res : Vec<Point> = Vec::new();

    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;

    for l in io::BufReader::new(file).lines() {
        let s = l.unwrap();
        let s:Vec<&str> = s.split(',').collect();
        let x : isize = s[0].parse().unwrap();
        let y : isize = s[1].parse().unwrap();
        let z : isize = s[2].parse().unwrap();

        max_x = max(max_x, x);
        max_y = max(max_y, y);
        max_z = max(max_z, z);
        res.push((x,y,z));
    }
    (res, (max_x+1, max_y+1, max_z+1))
}

fn get_neighbors(p1: Point) -> [Point;6] {
    [(p1.0, p1.1, p1.2 - 1), (p1.0, p1.1, p1.2 + 1),
    (p1.0, p1.1 - 1, p1.2), (p1.0, p1.1 + 1, p1.2),
    (p1.0 - 1, p1.1, p1.2), (p1.0 + 1, p1.1, p1.2)]
}

pub fn part1(input_file: &str) -> usize {
    let (drops, _p_max) = parse_input(input_file);

    let mut sides = 6 * drops.len();

    for i in 0 .. drops.len()-1 {
        for j in i+1..drops.len() {
            let p1 = drops[i];
            let p2 = drops[j];

            if get_neighbors(p1).contains(&p2) {
                sides -= 2;
            }

        }
    }

    sides
}


pub fn part2(input_file: &str) -> usize {
    let (drops, p_max) = parse_input(input_file);

    let mut visited : HashSet<Point> = HashSet::new();
    let mut queue : Vec<Point> = vec![(0,0,0)];

    let mut exterior_sides = 0;

    while let Some(p1) = queue.pop() {
        for p2 in get_neighbors(p1) {
            if visited.contains(&p2) {
                continue
            }

            if drops.contains(&p2) {
                exterior_sides += 1;
                continue
            }

            if ( (-1 <= p2.0) && (p2.0 <= p_max.0) ) &&
                ( (-1 <= p2.1) && (p2.1 <= p_max.1) ) &&
                ( (-1 <= p2.2) && (p2.2 <= p_max.2) ) {

                visited.insert(p2);
                queue.push(p2);
            }
        }
    }

    exterior_sides
}

fn main() {
    let p1 = part1("input/day18.txt");
    println!("Result 1 - {p1:?}");
    let p2 = part2("input/day18.txt");
    println!("Result 2 - {p2:?}");
}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1("input/day18_example.txt"), 64)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input/day18.txt"), 4288)
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2("input/day18_example.txt"), 58)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input/day18.txt"), 2494)
    }

}