use std::fs::File;
use std::{io};
use std::cmp::{max, min};
use std::collections::{HashSet};
use std::io::{BufRead};
use std::str::FromStr;
use std::string::ParseError;


type Cost = (usize, usize, usize);

#[derive(Debug, Clone)]
struct Blueprint {
    id: usize,
    ore: Cost,
    clay: Cost,
    obs: Cost,
    geode: Cost
}

impl Blueprint {
    fn max_ore_cost(&self) -> usize {
        max(max(max(self.ore.0, self.clay.0), self.obs.0), self.geode.0)
    }

    fn max_clay_cost(&self) -> usize {
        self.obs.1
    }

    fn max_obs_cost(&self) -> usize {
        self.geode.2
    }
}

impl FromStr for Blueprint {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let tokens : Vec<&str> = line.split(' ').collect();
        let id: usize = tokens[1].strip_suffix(':').unwrap().parse().unwrap();
        let ore_ore: usize = tokens[6].parse().unwrap();
        let clay_ore: usize = tokens[12].parse().unwrap();
        let obs_ore: usize = tokens[18].parse().unwrap();
        let obs_clay: usize = tokens[21].parse().unwrap();
        let geode_ore: usize = tokens[27].parse().unwrap();
        let geode_obs: usize = tokens[30].parse().unwrap();


        Ok(Blueprint {
            id,
            ore: (ore_ore, 0, 0),
            clay: (clay_ore, 0, 0),
            obs: (obs_ore, obs_clay, 0),
            geode: (geode_ore, 0, geode_obs)
        })
    }
}


fn parse_input(input_file: &str) -> Vec<Blueprint> {
    let file = File::open(input_file).unwrap();
    let mut blueprints : Vec<Blueprint> = Vec::new();

    for l in io::BufReader::new(file).lines() {
        let s = l.unwrap();
        let b: Blueprint = s.parse().unwrap();
        blueprints.push(b);
    }
    blueprints
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
struct State {
    geode_reserve: usize,
    time: usize,
    geode_miners: usize,
    obs_miners: usize,
    clay_miners: usize,
    ore_miners: usize,
    ore_reserve: usize,
    clay_reserve: usize,
    obs_reserve: usize,
}

enum Miners {Ore, Clay, Obsidian, Geode}

impl State {
    fn buildable(&self, bp: Blueprint, no_limit: bool) -> Vec<Miners> {
        let mut res: Vec<Miners> = Vec::new();

        if (self.ore_miners < bp.max_ore_cost() || no_limit) && (self.ore_reserve >= bp.ore.0) {
            res.push(Miners::Ore);
        }

        if (self.clay_miners < bp.max_clay_cost() || no_limit) && (self.ore_reserve >= bp.clay.0) {
            res.push(Miners::Clay)
        }

        if (self.obs_miners < bp.max_obs_cost() || no_limit) && (self.clay_reserve >= bp.obs.1) && (self.ore_reserve >= bp.obs.0) {
            res.push(Miners::Obsidian)
        }

        if (self.ore_reserve >= bp.geode.0) && (self.obs_reserve >= bp.geode.2) {
            res.push(Miners::Geode)
        }

        res
    }

}

fn calc_blueprint(bp: Blueprint, time: usize) -> usize {
    let start_state = State {
        time,
        ore_miners: 1,
        geode_miners: 0,
        obs_miners: 0,
        clay_miners: 0,
        geode_reserve: 0,
        obs_reserve: 0,
        clay_reserve: 0,
        ore_reserve: 0,
    };

    let mut stack = Vec::from([start_state]);
    let mut visited : HashSet<State> = HashSet::new();

    let mut max_yield = 0;

    while let Some(act_state) = stack.pop() {
        if !visited.insert(act_state.clone()) {
            continue
        }

        if act_state.time == 0 {
            max_yield = max(max_yield, act_state.geode_reserve);
            continue
        }

        // How much geode can we mine, if we build geode miner every turn for remainder of time
        let fast_yield:usize = act_state.geode_reserve + act_state.geode_miners * act_state.time + (1..=act_state.time-1).sum::<usize>();

        // Optimization - if we can't possibly mine more geode we just discard the branch - this is what saves most time
        if fast_yield <= max_yield {
            continue
        }

        let mut new_state = act_state.clone();
        new_state.time -= 1;
        new_state.ore_reserve += new_state.ore_miners;
        new_state.clay_reserve += new_state.clay_miners;
        new_state.obs_reserve += new_state.obs_miners;
        new_state.geode_reserve += new_state.geode_miners;

        //optimization - if we have enough miners to sustain building geode miner every turn, we just fast track
        if new_state.ore_miners >= bp.geode.0 && new_state.obs_miners >= bp.geode.2 {
            max_yield = max(max_yield, fast_yield);
            continue
        }

        //optimization - just make sure we hit cache more often, as we do not need extra resources at this point
        if new_state.ore_miners > bp.max_ore_cost() {
            new_state.ore_reserve = bp.max_ore_cost()
        }

        if new_state.clay_miners > bp.max_clay_cost() {
            new_state.clay_reserve = bp.max_clay_cost()
        }

        if new_state.obs_miners > bp.max_obs_cost() {
            new_state.obs_reserve = bp.max_obs_cost()
        }

        stack.push(new_state.clone());

        if new_state.time > 0 {

            for build in act_state.buildable(bp.clone(), false) {
                match build {
                    Miners::Ore => {
                        let mut b_state = new_state.clone();
                        b_state.ore_miners += 1;
                        b_state.ore_reserve -= bp.ore.0;
                        stack.push(b_state);
                    },

                    Miners::Clay => {
                        let mut b_state = new_state.clone();
                        b_state.clay_miners += 1;
                        b_state.ore_reserve -= bp.clay.0;
                        stack.push(b_state);
                    },

                    Miners::Obsidian => {
                        let mut b_state = new_state.clone();
                        b_state.obs_miners += 1;
                        b_state.ore_reserve -= bp.obs.0;
                        b_state.clay_reserve -= bp.obs.1;
                        stack.push(b_state);
                    },

                    Miners::Geode => {
                        let mut b_state = new_state.clone();
                        b_state.geode_miners += 1;
                        b_state.ore_reserve -= bp.geode.0;
                        b_state.obs_reserve -= bp.geode.2;
                        stack.push(b_state);
                    }
                }
            }

        }
    }

    max_yield
}

pub fn part1(input_file: &str) -> usize {
    let blueprints = parse_input(input_file);
    let mut sum = 0;

    for bp in blueprints {
        let calc = calc_blueprint(bp.clone(), 24);
        sum += calc * bp.id;
    }

    sum
}

pub fn part2(input_file: &str) -> usize {
    let blueprints = parse_input(input_file);
    let mut prod = 1;

    let bp_cnt = min(blueprints.len(), 3);

    for bp in &blueprints[..bp_cnt] {
        let calc = calc_blueprint(bp.clone(), 32);
        prod *= calc;
    }

    prod
}

fn main() {
    let p1 = part1("input/day19.txt");
    println!("Result 1 - {p1:?}");
    let p2 = part2("input/day19.txt");
    println!("Result 2 - {p2:?}");
}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[ignore]
    #[test]
    fn test_part1_example() {
        assert_eq!(part1("input/day19_example.txt"), 33)
    }

    #[ignore]
    #[test]
    fn test_part1() {
        assert_eq!(part1("input/day19.txt"), 1382)
    }

    #[ignore]
    #[test]
    fn test_part2_example() {
        assert_eq!(part2("input/day19_example.txt"), 3472)
    }

    #[ignore]
    #[test]
    fn test_part2() {
        assert_eq!(part2("input/day19.txt"), 31740)
    }

}