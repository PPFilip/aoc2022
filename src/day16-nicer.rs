use std::cmp::{min};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::iter::zip;
use itertools::{Itertools};


type ValveMap = HashMap<String, (usize, HashSet<String>)>;

// Since my own solution was really slow and I couldn't come up with something faster (it got me to
// sub 2000 leaderboard, but was based on random traversal), I reimplemented a **much** better one
// in Rust as learning exercise. I based the code it on this python notebook:
// https://github.com/bjmorgan/advent-of-code-2022/blob/main/solutions/day%2016.ipynb
// https://www.reddit.com/r/adventofcode/comments/zn6k1l/comment/j0kds9w/
// TODO: do more polish and make it feel more like Rust code


fn parse_input(input_file: &str) -> ValveMap {
    let file = File::open(input_file).unwrap();
    let mut res : ValveMap = HashMap::new();

    let re = regex::Regex::new(r"Valve ([A-Z]{2}) has flow rate=([0-9]+); tunnels? leads? to valves? (.*)").unwrap();

    for l in io::BufReader::new(file).lines() {
        let line = l.unwrap();
        let caps = re.captures(line.as_str()).unwrap();
        let valve: String = caps.get(1).map_or("".to_string(), |m| m.as_str().to_string());
        let flow: usize = caps.get(2).map_or("", |m| m.as_str()).parse().unwrap();
        let conn_str = caps.get(3).map_or("", |m| m.as_str());
        let connections: HashSet<String> = conn_str.split(", ").map(|m| m.to_string()).collect();

        res.insert(valve, (flow, connections));
    }

    res
}


// Floyd Warshall to pre-count distances
fn adj_matrix(valves: ValveMap) -> (Vec<String>, Vec<Vec<usize>>) {
    let mut valve_index: Vec<String> = Vec::new();
    for v in valves.keys() {
        valve_index.push(v.clone());
    }
    valve_index.sort();

    let mut arr : Vec<Vec<usize>> = Vec::new();

    for i in &valve_index {
        let mut line_map : Vec<usize> = Vec::new();

        for j in &valve_index {
            let val = if i == j {
                0
            } else if valves[i].1.contains(j) {
                1
            } else {
                10_000
            };

            line_map.push(val);
        }
        arr.push(line_map);
    }


    for i in 0..arr.len() {

        let mut neighbours: Vec<usize> = Vec::new();
        for (i, &v) in arr[i].iter().enumerate() {
            if v != 10000 {
                neighbours.push(i);
            }
        }

        for perm in neighbours.iter().permutations(2).unique() {
            let &n1 = perm[0];
            let &n2 = perm[1];

            let dist = min(arr[i][n1] + arr[i][n2], arr[n1][n2]);
            arr[n1][n2] = dist;
            arr[n2][n1] = dist;
        }

    }

    (valve_index, arr)
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Path {
    pressure: usize,
    time: usize,
    initial_cave: usize,
    visited: Vec<usize>,
}

// DFS to calculate paths
fn complete_paths(total_time: usize, stopping: bool, flows : Vec<usize>, adj_map: Vec<Vec<usize>>) -> Vec<Path> {
    let initial_cave : usize = 0; //AA always 1st

    let p = Path {
        time: total_time,
        initial_cave,
        visited: vec![initial_cave],
        pressure: 0
    };

    let mut stack : Vec<Path> = Vec::new();

    stack.push(p);

    let mut complete_path : Vec<Path> = Vec::new();

    while stack.len() > 0 {
        let path = stack.pop().unwrap();

        if stopping {
            complete_path.push(path.clone());
        }

        let mut new : Vec<Path> = Vec::new();

        let mut possible_next_caves : Vec<usize> = Vec::new();
        let mut times_per_cave : Vec<usize> = Vec::new();
        for (i, &f) in flows.iter().enumerate() {
            if (f != 0 ) && (!path.visited.contains(&i) ) {
                possible_next_caves.push(i);
                let t = adj_map[path.visited.last().unwrap().clone()][i]+1;
                times_per_cave.push(t);
            }

        }

        for (t, c) in zip(times_per_cave, possible_next_caves) {
            if path.time as isize - t as isize <= 0 {
                continue
            }

            let mut extended_path = path.clone();
            extended_path.time -= t;
            extended_path.visited.push(c);
            extended_path.pressure += (path.time - t) * flows[c];

            new.push(extended_path);
        }

        if new.len() > 0 {
            for n in new {
                stack.push(n);
            }
        } else {
            if !stopping {
                complete_path.push(path);
            }
        }
    }

    complete_path

}

pub fn part1(input_file: &str, total_time: usize, stopping: bool) -> usize {
    let valves = parse_input(input_file);
    let (valve_index, adj_map) = adj_matrix(valves.clone());

    let mut flows : Vec<usize> = Vec::new();
    for v in valve_index {
        flows.push(valves[&v].0);
    }

    let paths = complete_paths(total_time, stopping, flows, adj_map);

    let mut max_pressure = 0;

    for p in paths {
        if p.pressure > max_pressure {
            max_pressure = p.pressure
        }
    }

    max_pressure
}


// use pre-calculated paths from part1
pub fn part2(input_file: &str, total_time: usize, stopping: bool) -> usize {
    let valves = parse_input(input_file);
    let (valve_index, adj_map) = adj_matrix(valves.clone());

    let mut flows : Vec<usize> = Vec::new();
    for v in valve_index {
        flows.push(valves[&v].0);
    }

    let mut paths = complete_paths(total_time, stopping, flows, adj_map);

    paths.sort();
    paths.reverse();

    let mut max_pressure = 0;
    let mut j = 0;


    for (i, a) in paths.iter().enumerate() {
        if i > j {
            continue
        }

        let x = &a.visited[1..];

        let bslice = &paths[i+1..];
        for k in 0 .. bslice.len() {
            j = k+i;
            let b = &paths[k+i];

            if a.pressure + b.pressure <= max_pressure {
                break
            }

            let y = &b.visited[1..];

            let mut intersect : Vec<usize> = Vec::new();

            for xx in x {
                if y.contains(xx) {
                    intersect.push(xx.clone());
                }
            }

            if intersect.len() == 0 {
                if a.pressure + b.pressure > max_pressure {
                    max_pressure = a.pressure + b.pressure;
                }

            }

        }

    }

    max_pressure
}


fn main() {
    let p1 = part1("input/day16.txt", 30, false);
    println!("Result 1 - {p1:?}");
    let p2 = part2("input/day16.txt", 26, true);
    println!("Result 2 - {p2:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1("input/day16_example.txt", 30, false), 1651)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input/day16.txt", 30, false), 1701)
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2("input/day16_example.txt", 26, true), 1707)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input/day16.txt", 26, true), 2455)
    }

}