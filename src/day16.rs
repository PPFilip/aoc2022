use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;
use rand::prelude::*;


const MAX_ITERATIONS: usize = 10_000_000;
type ValveMap = HashMap<String, (isize, HashSet<String>)>;


fn parse_input(input_file: &str) -> ValveMap {
    let file = File::open(input_file).unwrap();
    let mut res : ValveMap = HashMap::new();

    let re = regex::Regex::new(r"Valve ([A-Z]{2}) has flow rate=([0-9]+); tunnels? leads? to valves? (.*)").unwrap();

    for l in io::BufReader::new(file).lines() {
        let line = l.unwrap();
        let caps = re.captures(line.as_str()).unwrap();
        let valve: String = caps.get(1).map_or("".to_string(), |m| m.as_str().to_string());
        let flow: isize = caps.get(2).map_or("", |m| m.as_str()).parse().unwrap();
        let conn_str = caps.get(3).map_or("", |m| m.as_str());
        let connections: HashSet<String> = conn_str.split(", ").map(|m| m.to_string()).collect();

        res.insert(valve, (flow, connections));
    }

    res
}

//TODO: Make the solution optimized. This random traversal works, but takes ages to complete. I was submitting answers as it was printing XD
pub fn part1(input_file: &str) -> isize {
    let valves = parse_input(input_file);

    let mut max_pressure = 0;

    let mut rng = thread_rng();

    for i in 0..MAX_ITERATIONS {

        if i % 100_000 == 0 {
            println!("Iteration: {i}");
        }
        let mut previous = "AA".to_string();
        let mut current = "AA".to_string();
        let mut opened : HashSet<String> = HashSet::new();
        let mut minutes = 30;
        let mut pressure = 0;

        while minutes >= 0 {
            let valve = valves[&current].clone();
            if (valve.0 > 0) && !opened.contains(&current) && (random::<f32>() >= 0.15) {
                minutes -= 1;
                opened.insert((&current).clone());
                pressure += minutes * &valves[&current].0
            }
            let mut choices = valve.1;
            if (random::<f32>() >= 0.05) && (choices.len() > 1) && choices.contains(&previous) {
                choices.remove(&previous);
            }

            previous = current.clone();
            current = choices.iter().nth(rng.gen_range(0..choices.len())).unwrap().clone();
            minutes -= 1;
        }

        if pressure > max_pressure {
            max_pressure = pressure;
            println!("{i} -> {max_pressure}");
        }
    }

    max_pressure
}


pub fn part2(input_file: &str) -> isize {
    let valves = parse_input(input_file);

    let mut max_pressure = 0;

    let mut rng = thread_rng();

    for i in 0..MAX_ITERATIONS {

        if i % 100_000 == 0 {
            println!("Iteration: {i}");
        }
        let mut previous = Vec::from(["AA".to_string(), "AA".to_string()]);
        let mut current = Vec::from(["AA".to_string(), "AA".to_string()]);
        let mut opened : HashSet<String> = HashSet::new();
        let mut minutes = 26;
        let mut pressure = 0;

        while minutes >= 0 {
            for m in 0..2 {
                let valve = valves[&current[m]].clone();
                if (valve.0 > 0) && !opened.contains(&current[m]) && (random::<f32>() >= 0.15){
                    opened.insert((&current[m]).clone());
                    pressure += max(0, minutes-1) * &valves[&current[m]].0
                } else {
                    let mut choices = valve.1;
                    if (random::<f32>() >= 0.05) && (choices.len() > 1) && choices.contains(&previous[m]) {
                        choices.remove(&previous[m]);
                    }
                    if (random::<f32>() >= 0.20) && (m == 1) && (choices.len() > 1) && choices.contains(&current[0]) {
                        choices.remove(&current[0]);
                    }
                    previous[m] = (&current[m]).clone();
                    current[m] = choices.iter().nth(rng.gen_range(0..choices.len())).unwrap().clone();
                }
            }

            minutes -= 1;
        }

        if pressure > max_pressure {
            max_pressure = pressure;
            println!("{i} -> {max_pressure}");
        }
    }

    max_pressure
}


fn main() {
    let p1 = part1("input/day16.txt");
    println!("Result 1 - {p1:?}");
    let p2 = part2("input/day16.txt");
    println!("Result 2 - {p2:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[ignore]
    #[test]
    fn test_part1_example() {
        assert_eq!(part1("input/day16_example.txt"), 1651)
    }

    #[ignore]
    #[test]
    fn test_part1() {
        assert_eq!(part1("input/day16.txt"), 1701)
    }

    #[ignore]
    #[test]
    fn test_part2_example() {
        assert_eq!(part2("input/day16_example.txt"), 1707)
    }

    #[ignore]
    #[test]
    fn test_part2() {
        assert_eq!(part2("input/day16.txt"), 2455)
    }

}