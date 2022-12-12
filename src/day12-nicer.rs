// messing with pathfinding crate and writing nicer code (similar to what I used in day 9) to parse input ...
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;
use pathfinding::prelude::bfs;

const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (0, 1), (1, 0), (-1, 0)];

#[derive(Clone, Debug, Eq, Ord, Hash, PartialEq, PartialOrd, Copy)]
struct Pos(usize, usize);

fn parse_input(input_file: &str, start: &mut Pos, end: &mut Pos) -> Vec<Vec<u8>> {
    let mut array: Vec<Vec<u8>> = Vec::new();
    let file = File::open(input_file).unwrap();

    for (i, l) in io::BufReader::new(file).lines().enumerate() {
        let mut line:Vec<u8> = Vec::new();
        for (j, c) in l.unwrap().chars().enumerate() {
            let point = Pos(j, i);
            let height = match c {
                'S' => {
                    *start = point;
                    0
                },
                'E' => {
                    *end = point;
                    26
                },
                _ => {
                    (c as u8) - 96
                }
            };
            line.push(height);
        }
        array.push(line);
    }

    array
}


pub fn part1(input_file: &str) -> usize {
    let mut start = Pos(0,0);
    let mut end = Pos(0,0);
    let array: Vec<Vec<u8>> = parse_input(input_file, &mut start, &mut end);

    let mut edges: HashMap<Pos, HashSet<Pos>> = HashMap::new();

    let max_rows = array.len();
    let max_cols = array[0].len();

    for y in 0..max_rows {
        for x in 0..max_cols {
            let pos = Pos(x, y);
            let mut connections: HashSet<Pos> = HashSet::new();
            let val = array[y][x];

            for (dx, dy) in DIRECTIONS {
                let nx = dx + x as isize;
                let ny = dy + y as isize;
                if nx<0 || ny <0 {
                    continue;
                }

                let ux = nx as usize;
                let uy = ny as usize;
                if uy >= max_rows || ux >= max_cols {
                    continue;
                }

                let n = array[uy][ux];
                if val + 1 >= n {
                    connections.insert(Pos(ux, uy));
                }
            }

            edges.insert(pos, connections);
        }
    }

    let path = bfs(&start, |p| edges.get(p).unwrap().to_owned(), |&p| p == end);
    path.expect("").len()-1
}


pub fn part2(input_file: &str) -> usize {
    let mut start = Pos(0,0);
    let mut end = Pos(0,0);
    // start is irrelevant here, will be market later
    let array: Vec<Vec<u8>> = parse_input(input_file, &mut start, &mut end);

    let mut edges: HashMap<Pos, HashSet<Pos>> = HashMap::new();


    // flip connection conditions around
    let max_rows = array.len();
    let max_cols = array[0].len();

    for y in 0..max_rows {
        for x in 0..max_cols {
            let pos = Pos(x, y);
            let mut connections: HashSet<Pos> = HashSet::new();
            let val = array[y][x];

            for (dx, dy) in DIRECTIONS {
                let nx = dx + x as isize;
                let ny = dy + y as isize;
                if nx<0 || ny <0 {
                    continue;
                }

                let ux = nx as usize;
                let uy = ny as usize;
                if uy >= max_rows || ux >= max_cols {
                    continue;
                }

                let n = array[uy][ux];
                if n + 1 >= val {
                    connections.insert(Pos(ux, uy));
                }
            }

            edges.insert(pos, connections);
        }
    }

    let path = bfs(&end, |p| edges.get(p).unwrap().to_owned(), |&p| array[p.1][p.0] == 1);
    path.expect("").len()-1
}

fn main() {
    let p1 = part1("input/day12.txt");
    println!("Result 1 - {p1:?}");
    let p2 = part2("input/day12.txt");
    println!("Result 2 - {p2:?}");
}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1("input/day12_example.txt"), 31)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input/day12.txt"), 520)
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2("input/day12_example.txt"), 29)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input/day12.txt"), 508)
    }

}