use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;


pub fn part1(input_file: &str) -> u32 {
    let file = File::open(input_file).unwrap();
    let mut act_path: Vec<String> = Vec::new();

    let mut file_sizes:HashMap<(Vec<String>, String),u32> = HashMap::new();
    let mut dir_sizes:HashMap<Vec<String>, u32> = HashMap::new();

    for l in io::BufReader::new(file).lines() {
        let line = l.unwrap();
        let v: Vec<&str> = line.split(' ').collect();
        let pos0 = v.get(0).unwrap().to_owned();
        let pos1 = v.get(1).unwrap().to_owned();
        if pos0 == "$" {
            if pos1 == "cd" {
                let dir = v.get(2).unwrap().to_owned();
                match dir {
                    ".." => {
                        act_path.pop(); ()
                    },
                    "/" => {
                        act_path.clear()
                    },
                    _ => {
                        act_path.push(dir.to_string())
                    }
                };

                if !dir_sizes.contains_key(&*act_path.clone()) {
                    dir_sizes.insert(act_path.clone(), 0);
                }
            }
        } else {
            if let Ok(size) = pos0.parse::<u32>() {
                let fname = pos1.to_string();
                if file_sizes.insert((act_path.clone(), fname), size) == None {

                    let mut ap = act_path.clone();

                    for _x in 0 .. ap.len()+1 {
                        if let Some(act_size) = dir_sizes.get(&*ap.clone()) {
                            dir_sizes.insert(ap.clone(), act_size+size);
                            ap.pop();
                        }
                    }

                }
            }
        }
    }

    let total = dir_sizes.values().filter(|&&x| x<=100000).sum();
    total
}

pub fn part2(input_file: &str) -> u32 {
    let file = File::open(input_file).unwrap();
    let mut act_path: Vec<String> = Vec::new();

    let mut file_sizes:HashMap<(Vec<String>, String),u32> = HashMap::new();
    let mut dir_sizes:HashMap<Vec<String>, u32> = HashMap::new();

    for l in io::BufReader::new(file).lines() {
        let line = l.unwrap();
        let v: Vec<&str> = line.split(' ').collect();
        let pos0 = v.get(0).unwrap().to_owned();
        let pos1 = v.get(1).unwrap().to_owned();
        if pos0 == "$" {
            if pos1 == "cd" {
                let dir = v.get(2).unwrap().to_owned();
                match dir {
                    ".." => {
                        act_path.pop(); ()
                    },
                    "/" => {
                        act_path.clear()
                    },
                    _ => {
                        act_path.push(dir.to_string())
                    }
                };

                if !dir_sizes.contains_key(&*act_path.clone()) {
                    dir_sizes.insert(act_path.clone(), 0);
                }
            }
        } else {
            if let Ok(size) = pos0.parse::<u32>() {
                let fname = pos1.to_string();
                if file_sizes.insert((act_path.clone(), fname), size) == None {

                    let mut ap = act_path.clone();

                    for _x in 0 .. ap.len()+1 {
                        if let Some(act_size) = dir_sizes.get(&*ap.clone()) {
                            dir_sizes.insert(ap.clone(), act_size+size);
                            ap.pop();
                        }
                    }

                }
            }
        }
    }

    const TOTAL_SIZE:u32 = 70000000;
    const DESIRED_SIZE:u32 = 30000000;

    let root_dir:Vec<String> = Vec::from([]);
    let unused_space = TOTAL_SIZE - dir_sizes.get(&*root_dir).unwrap();
    let find_free_space = DESIRED_SIZE - unused_space;

    let mut total = u32::MAX;
    for (_dir, size) in dir_sizes {
        if find_free_space < size && size < total {
            total = size;
        }
    }

    total
}

fn main() {
    let p1 = part1("input/day07.txt");
    println!("Result 1 - {p1:?}");
    let p2 = part2("input/day07.txt");
    println!("Result 2 - {p2:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1("input/day07_example.txt"), 95437)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input/day07.txt"), 1449447)
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2("input/day07_example.txt"), 24933642)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input/day07.txt"), 8679207)
    }

}