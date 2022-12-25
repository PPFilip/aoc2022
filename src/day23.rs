use std::fs::File;
use std::{io};
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::io::{BufRead};

type Point = (isize, isize);

#[derive(Debug, Clone, Eq, PartialEq)]
struct Elf {
    pos_act: Point,
    pos_next: Point
}

fn parse_input2(input_file: &str) -> Vec<Elf> {
    let file = File::open(input_file).unwrap();
    let mut elves : Vec<Elf> = Vec::new();

    for (y, line) in io::BufReader::new(file).lines().enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
            match c {
                '.' => (),
                '#' => {
                    elves.push(Elf {
                        pos_act: (x as isize, y as isize),
                        pos_next: (x as isize, y as isize),
                    })
                },
                _ => panic!("wtf")
            }
        }
    }

    elves
}

fn add_point(p1: Point, p2: Point) -> Point {
    (p1.0 + p2.0, p1.1 + p2.1)
}

fn has_neighbour(point: Point, checks: &[Point], elven_positions: &HashSet<Point>) -> bool {
    let points: HashSet<Point> = checks.iter().map(|&c| add_point(c, point)).collect();

    points.iter().any(|p| elven_positions.contains(p))
}


fn play_turn(elves: &mut [Elf], dir_index: usize) {
    let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    let dir_checks = [vec![(-1, -1), (0, -1), (1, -1)], vec![(-1, 1), (0, 1), (1, 1)], vec![(-1, -1), (-1, 0), (-1, 1)], vec![(1, -1), (1, 0), (1, 1)]];

    let can_move_checks = vec![(-1, -1), (0, -1), (1, -1), (-1, 1), (0, 1), (1, 1), (-1, 0), (1, 0)];


    let mut props: HashMap<Point, isize> = HashMap::new();
    let elven_positions: HashSet<Point> = elves.iter().map(|e| e.pos_act).collect();

    for elf in elves.iter_mut() {
        elf.pos_next = elf.pos_act;

        if !has_neighbour(elf.pos_act, &can_move_checks, &elven_positions) {
            continue
        }

        for d in 0..4 {
            let move_idx = (dir_index + d) % 4;
            let checks = &dir_checks[move_idx];

            if has_neighbour(elf.pos_next, checks, &elven_positions) {
                continue
            }

            elf.pos_next = add_point(elf.pos_act, directions[move_idx]);

            if let Some(prop) = props.get_mut(&elf.pos_next) {
                *prop += 1;
            } else {
                props.insert(elf.pos_next, 1);
            }
            break;
        }
    }

    for elf in elves.iter_mut() {
        if let Some(&count) = props.get(&elf.pos_next) {
            if count == 1 {
                elf.pos_act = elf.pos_next;
            } else {
                elf.pos_next = elf.pos_act;
            }
        }
    }
}

fn minmax(elves: &[Elf]) -> (Point, Point) {
    let mut min_x = isize::MAX;
    let mut min_y = isize::MAX;
    let mut max_x = isize::MIN;
    let mut max_y = isize::MIN;

    for e in elves {
        min_x = min(e.pos_act.0, min_x);
        min_y = min(e.pos_act.1, min_y);
        max_x = max(e.pos_act.0, max_x);
        max_y = max(e.pos_act.1, max_y);
    }

    ((min_x, min_y), (max_x, max_y))
}

#[allow(dead_code)]
fn dbg_map(elves: &[Elf]) {
    let elves_positions: HashSet<Point> = elves.iter().map(|e| e.pos_act).collect();

    let (p1, p2) = minmax(elves);

    for i in p1.1 .. p2.1 + 1 {
        for j in p1.0 .. p2.0 + 1 {
            if elves_positions.contains(&(j, i)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}


pub fn part1(input_file: &str) -> isize {
    let mut elves = parse_input2(input_file);

    for (dir_index, _) in (0..10).enumerate() {
        play_turn(&mut elves, dir_index);
    }

    let (p1, p2) = minmax(&elves);
    ((p2.0 - p1.0).abs() + 1) * ((p2.1 - p1.1).abs() + 1) - (elves.len() as isize)

}

pub fn part2(input_file: &str) -> isize {
    let mut elves_act = parse_input2(input_file);

    'outer: for turn in 0.. {
        let mut elves_new = elves_act.clone();
        play_turn(&mut elves_new, turn);

        for (idx, elf) in elves_new.iter().enumerate() {
            if elves_act[idx] != *elf {
                elves_act = elves_new;
                continue 'outer
            }
        }

        return turn as isize + 1
    }

    unreachable!()
}


fn main() {
    let p1 = part1("input/day23.txt");
    println!("Result 1 - {p1:?}");
    let p2 = part2("input/day23.txt");
    println!("Result 2 - {p2:?}");
}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1("input/day23_example.txt"), 110)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input/day23.txt"), 3947)
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2("input/day23_example.txt"), 20)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input/day23.txt"), 1012)
    }

}