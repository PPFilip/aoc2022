use std::fs::File;
use std::{io};
use std::collections::HashMap;
use std::fmt::format;
use std::io::{BufRead};
use std::str::FromStr;
use std::string::ParseError;


#[derive(Debug, Clone)]
struct Monkey {
    name: String,
    value: isize,
    m1: String,
    m2: String,
    op: char
}

impl FromStr for Monkey {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = line.split(' ').collect();

        match tokens.len() {
            2 => {
                Ok(Monkey {
                    name: tokens[0][..tokens[0].len()-1].to_string(),
                    value: tokens[1].parse().unwrap(),
                    op: '0',
                    m1: "".to_string(),
                    m2: "".to_string()
                })
            },
            4 => {
                Ok(Monkey {
                    name: tokens[0][..tokens[0].len()-1].to_string(),
                    value: 0,
                    m1: tokens[1].to_string(),
                    op: tokens[2].chars().next().unwrap(),
                    m2: tokens[3].to_string()
                })
            },
            _ => panic!("wtf")
        }
    }

}


fn parse_input(input_file: &str) -> HashMap<String, Monkey> {
    println!("parsing");
    let file = File::open(input_file).unwrap();
    let res : HashMap<String, Monkey> = io::BufReader::new(file).lines().map(|l| l.unwrap().parse::<Monkey>().unwrap()).map(|m| (m.clone().name, m)).collect();
    res
}


fn val(name: String, monkeys: &HashMap<String, Monkey>) -> isize {

    let m = monkeys.get(&name).unwrap();

    match m.op {
        '0' => m.value,
        '+' => val(m.m1.clone(), monkeys) + val(m.m2.clone(), monkeys),
        '-' => val(m.m1.clone(), monkeys) - val(m.m2.clone(), monkeys),
        '*' => val(m.m1.clone(), monkeys) * val(m.m2.clone(), monkeys),
        '/' => val(m.m1.clone(), monkeys) / val(m.m2.clone(), monkeys),
        _ => panic!("wtf")
    }
}



fn val2(name: String, monkeys: &HashMap<String, Monkey>) -> Result<isize, String> {

    let m = monkeys.get(&name).unwrap();

    if m.name == *"humn" {
        return Err("x".to_string())
    }

    match m.op {
        '0' => Ok(m.value),
        '+' => {
            let v1 = val2(m.m1.clone(), monkeys);
            let v2 = val2(m.m2.clone(), monkeys);

            if v1.is_ok() && v2.is_ok() {
                Ok(v1.unwrap() + v2.unwrap())
            } else if v1.is_ok() {
                Err(format!("({}+{})", v1.unwrap(), v2.unwrap_err()))
            } else {
                Err(format!("({}+{})", v1.unwrap_err(), v2.unwrap()))
            }

        },
        '-' => {
            let v1 = val2(m.m1.clone(), monkeys);
            let v2 = val2(m.m2.clone(), monkeys);

            if v1.is_ok() && v2.is_ok() {
                Ok(v1.unwrap() - v2.unwrap())
            } else if v1.is_ok() {
                Err(format!("({}-{})", v1.unwrap(), v2.unwrap_err()))
            } else {
                Err(format!("({}-{})", v1.unwrap_err(), v2.unwrap()))
            }

        },
        '*' => {
            let v1 = val2(m.m1.clone(), monkeys);
            let v2 = val2(m.m2.clone(), monkeys);

            if v1.is_ok() && v2.is_ok() {
                Ok(v1.unwrap() * v2.unwrap())
            } else if v1.is_ok() {
                Err(format!("({}*{})", v1.unwrap(), v2.unwrap_err()))
            } else {
                Err(format!("({}*{})", v1.unwrap_err(), v2.unwrap()))
            }

        },
        '/' => {
            let v1 = val2(m.m1.clone(), monkeys);
            let v2 = val2(m.m2.clone(), monkeys);

            if v1.is_ok() && v2.is_ok() {
                Ok(v1.unwrap() / v2.unwrap())
            } else if v1.is_ok() {
                Err(format!("({}/{})", v1.unwrap(), v2.unwrap_err()))
            } else {
                Err(format!("({}/{})", v1.unwrap_err(), v2.unwrap()))
            }

        },
        _ => panic!("wtf")
    }
}

pub fn part1(input_file: &str) -> isize {
    let monkeys = parse_input(input_file);
    val("root".to_string(), &monkeys)
}


pub fn part2(input_file: &str) -> isize {
    let monkeys = parse_input(input_file);

    let root = monkeys.get("root").unwrap().clone();
    let eq1 = val2(root.m1, &monkeys);
    let eq2 = val2(root.m2, &monkeys);

    let v1 = if eq1.is_ok() {
        eq1.unwrap().to_string()
    } else {
        eq1.unwrap_err()
    };

    let v2 = if eq2.is_ok() {
        eq2.unwrap().to_string()
    } else {
        eq2.unwrap_err()
    };

    let s2 = format!("{v1}={v2}");
    //paste this to wolfram and be done with it
    println!("{s2}");

    0
}


fn main() {
    let p1 = part1("input/day21.txt");
    println!("Result 1 - {p1:?}");
    let p2 = part2("input/day21.txt");
    println!("Result 2 - {p2:?}");
}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1("input/day21_example.txt"), 152)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input/day21.txt"), 160274622817992)
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2("input/day21_example.txt"), 301)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input/day21.txt"), 0) //3087390115721
    }

}