use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io;
use std::io::BufRead;

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


    for y in 0..array.len() {
        for x in 0..array[y].len() {
            let pos = Pos(x, y);
            let mut connections: HashSet<Pos> = HashSet::new();
            let val = array[y][x];

            if y>=1 {
                let up = array[y-1][x];
                if val + 1 >= up {
                    connections.insert(Pos(x, y-1));
                }
            }

            if y<=array.len()-2 {
                let down = array[y+1][x];
                if val + 1 >= down {
                    connections.insert(Pos(x, y+1));
                }
            }

            if x>=1 {
                let left = array[y][x-1];
                if val + 1 >= left {
                    connections.insert(Pos(x-1, y));
                }
            }

            if x<=array[y].len()-2 {
                let right = array[y][x+1];
                if val + 1 >= right {
                    connections.insert(Pos(x+1, y));
                }
            }

            edges.insert(pos, connections);
        }
    }

    // BFS
    let mut queue:VecDeque<Pos> = VecDeque::new();
    let mut explored:HashMap<Pos, Pos> = HashMap::new();

    queue.push_front(start);
    explored.insert(start, start);

    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();

        if node == end {
            break;
        }

        if let Some(neighbors) = edges.get(&node) {
            for n in neighbors {
                if explored.get(n) == None {
                    explored.insert(n.clone(), node);
                    queue.push_back(n.clone());
                }
            }
        }
    }

    let mut path:Vec<Pos> = Vec::new();
    let mut p = end.clone();
    path.push(p);
    while p != start {
        p = explored[&p];
        path.push(p);
    }

    path.len()-1
}


pub fn part2(input_file: &str) -> usize {
    let mut start = Pos(0,0);
    let mut end = Pos(0,0);
    // start is irrelevant here, will be market later
    let array: Vec<Vec<u8>> = parse_input(input_file, &mut start, &mut end);

    let mut edges: HashMap<Pos, HashSet<Pos>> = HashMap::new();


    // flip connection conditions around
    for y in 0..array.len() {
        for x in 0..array[y].len() {
            let pos = Pos(x, y);
            let mut connections: HashSet<Pos> = HashSet::new();
            let val = array[y][x];

            if y>=1 {
                let up = array[y-1][x];
                if up + 1 >= val {
                    connections.insert(Pos(x, y-1));
                }
            }

            if y<=array.len()-2 {
                let down = array[y+1][x];
                if down + 1 >= val {
                    connections.insert(Pos(x, y+1));
                }
            }

            if x>=1 {
                let left = array[y][x-1];
                if left + 1 >= val {
                    connections.insert(Pos(x-1, y));
                }
            }

            if x<=array[y].len()-2 {
                let right = array[y][x+1];
                if right + 1 >= val {
                    connections.insert(Pos(x+1, y));
                }
            }

            edges.insert(pos, connections);
        }
    }

    // BFS
    let mut queue:VecDeque<Pos> = VecDeque::new();
    let mut explored:HashMap<Pos, Pos> = HashMap::new();

    queue.push_front(end);
    explored.insert(end, end);

    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();

        // 1st 'a' we reach is the shortest path, mark start
        if array[node.1][node.0] == 1 { // 'a'
            start = node;
            break;
        }

        if let Some(neighbors) = edges.get(&node) {
            for n in neighbors {
                if explored.get(n) == None {
                    explored.insert(n.clone(), node);
                    queue.push_back(n.clone());
                }
            }
        }
    }

    let mut path:Vec<Pos> = Vec::new();
    let mut p = start.clone();
    path.push(p);
    while p != end {
        p = explored[&p];
        path.push(p);
    }

    path.len()-1
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