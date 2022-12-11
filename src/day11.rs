use std::fs::File;
use std::io;
use std::io::Read;
use itertools::Itertools;


#[derive(Debug, Clone)]
struct Monkey {
    #[allow(dead_code)]
    id: usize,
    items: Vec<u64>,
    inspected_items: usize,
    op_code: char,
    op_amt: u64,
    div_test: u64,
    pass_true: usize,
    pass_false: usize,
}

impl Monkey {
    fn inspect_item(&mut self) {
        self.inspected_items += 1;
    }

    fn add_item(&mut self, item: u64) {
        self.items.push(item);
    }
}

fn parse_input(input_file: &str) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let file = File::open(input_file).unwrap();
    let mut str = "".to_string();
    let mut _lc = io::BufReader::new(file).read_to_string(&mut str).unwrap();
    let monkeys_str: Vec<&str> = str.split("\n\n").collect();

    for m in monkeys_str {
        let ml: Vec<&str> = m.lines().collect();

        //id
        let str_id = &ml[0][7 .. ml[0].len()-1];
        let id: usize = str_id.parse().unwrap();

        // items
        let mut items= Vec::new();
        let istr = &ml[1][18 .. ml[1].len()];
        let str_items:Vec<&str> = istr.split(", ").collect();
        for i in str_items {
            let ni: u64 = i.parse().unwrap();
            items.push(ni);
        }

        // op
        let str_test:Vec<&str> = ml[2].split(' ').collect();
        let mut op_code: char = str_test[6].chars().nth(0).unwrap();
        let op_amt: u64 = if let Ok(o) = str_test.last().unwrap().parse() {
            o
        } else {
            op_code = '^';
            0
        };

        //test
        let str_test:Vec<&str> = ml[3].split(' ').collect();
        let div_test: u64 = str_test.last().unwrap().parse().unwrap();

        //if true
        let str_true:Vec<&str> = ml[4].split(' ').collect();
        let pass_true: usize = str_true.last().unwrap().parse().unwrap();

        //if false
        let str_false:Vec<&str> = ml[5].split(' ').collect();
        let pass_false: usize = str_false.last().unwrap().parse().unwrap();

        monkeys.push(Monkey {
            id,
            items,
            inspected_items: 0,
            op_code,
            op_amt,
            div_test,
            pass_true,
            pass_false
        });

    }

    monkeys
}


pub fn part1(input_file: &str) -> usize {
    let mut monkeys: Vec<Monkey> = parse_input(input_file);

    for _ in 0..20 {
        for mi in 0..monkeys.len() {
            let m = monkeys[mi].clone();

            for item in m.items {
                monkeys[mi].inspect_item();

                let new_level:u64 = match m.op_code {
                    '^' => item * item,
                    '*' => item * m.op_amt,
                    '+' => item + m.op_amt,
                    _ => panic!("wtf")
                } / 3;

                if new_level % m.div_test == 0 {
                    monkeys[m.pass_true].add_item(new_level);
                } else {
                    monkeys[m.pass_false].add_item(new_level);
                }
            }
            monkeys[mi].items.clear();
        }
    }

    monkeys.iter().map(|m| m.inspected_items).sorted().rev().take(2).product()
}


pub fn part2(input_file: &str) -> usize {
    let mut monkeys: Vec<Monkey> = parse_input(input_file);

    let factor: u64 = monkeys.iter().map(|m| m.div_test).product();

    for _ in 0..10_000 {
        for mi in 0..monkeys.len() {
            let m = monkeys[mi].clone();

            for item in m.items {
                monkeys[mi].inspect_item();

                let new_level:u64 = match m.op_code {
                    '^' => item * item,
                    '*' => item * m.op_amt,
                    '+' => item + m.op_amt,
                    _ => panic!("wtf")
                } % factor;

                if new_level % m.div_test == 0 {
                    monkeys[m.pass_true].add_item(new_level);
                } else {
                    monkeys[m.pass_false].add_item(new_level);
                }
            }
            monkeys[mi].items.clear();
        }
    }

    monkeys.iter().map(|m| m.inspected_items).sorted().rev().take(2).product()
}

fn main() {
    let p1 = part1("input/day11.txt");
    println!("Result 1 - {p1:?}");
    let p2 = part2("input/day11.txt");
    println!("Result 2 - {p2:?}");
}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1("input/day11_example.txt"), 10605)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input/day11.txt"), 108240)
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2("input/day11_example.txt"), 2713310158)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input/day11.txt"), 25712998901)
    }

}