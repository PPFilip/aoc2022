use std::fs::File;
use std::{io};
use std::io::{Read};


#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {Void, Open, Wall}

#[derive(Debug)]
enum Direction {Right, Down, Left, Up}

#[derive(Debug)]
enum Instruction {
    Move(usize),
    TurnRight,
    TurnLeft,
}

fn parse_input(input_file: &str) -> (Vec<Vec<Tile>>, Vec<Instruction>) {
    let file = File::open(input_file).unwrap();
    let mut result = "".to_string();
    let mut _lc = io::BufReader::new(file).read_to_string(&mut result).unwrap();
    let (maze, str_inst) = result.split_once("\n\n").unwrap();

    let maze = maze.to_string();
    let mut res : Vec<Vec<Tile>> = maze.lines()
        .map(|l| l.chars()
            .map(|c| match c {
                ' ' => Tile::Void,
                '#' => Tile::Wall,
                '.' => Tile::Open,
                _ => panic!("wtf")
            }).collect::<Vec<Tile>>()).collect();

    let width = res.iter().map(|r| r.len()).max().unwrap() + 1;
    for row in res.iter_mut() {
        if row.len() < width {
            row.append(&mut vec![Tile::Void; width - row.len()]);
        }
    }

    let str_inst = str_inst.to_string().replace("R", " R ").replace("L", " L ");

    let instructions: Vec<Instruction> = str_inst.split(' ').collect::<Vec<&str>>()
        .iter().map(|&d|
        match d {
            "R" => Instruction::TurnRight,
            "L" => Instruction::TurnLeft,
            _ => Instruction::Move(d.parse().unwrap())
        }
    ).collect();

    (res, instructions)
}

pub fn part1(input_file: &str) -> usize {
    let (maze, instrucrions) = parse_input(input_file);

    let mut x = maze[0].iter().position(|&t| t == Tile::Open).unwrap();
    let mut y = 0_usize;

    let dirs = vec![Direction::Right, Direction::Down, Direction::Left, Direction::Up];

    let mut dir_index = 0;

    for i in instrucrions {
        match i {
            Instruction::TurnRight => dir_index += 1,
            Instruction::TurnLeft => dir_index += 3,
            Instruction::Move(c) => {
                for _ in 0..c {
                    match dirs[dir_index%4] {
                        Direction::Right => {
                            let mut next_x = x;
                            loop {
                                if next_x+1 == maze[y].len() {
                                    next_x = 0
                                } else {
                                    next_x += 1
                                };

                                match maze[y][next_x] {
                                    Tile::Void => { continue }
                                    Tile::Wall => { break }
                                    Tile::Open => { x = next_x; break }
                                }
                            }
                        }
                        Direction::Down => {
                            let mut next_y = y;
                            loop {
                                    if next_y+1 == maze.len() {
                                        next_y = 0
                                    } else {
                                        next_y += 1
                                    };

                                match maze[next_y][x] {
                                    Tile::Void => { continue }
                                    Tile::Wall => { break }
                                    Tile::Open => { y = next_y; break }
                                }
                            }
                        }
                        Direction::Left => {
                            let mut next_x = x;
                            loop {
                                if next_x == 0 {
                                    next_x = maze[y].len() - 1
                                } else {
                                    next_x -= 1
                                };

                                match maze[y][next_x] {
                                    Tile::Void => { continue }
                                    Tile::Wall => { break }
                                    Tile::Open => { x = next_x; break }
                                }
                            }
                        }
                        Direction::Up => {
                            let mut next_y = y;
                            loop {
                                if next_y == 0 {
                                    next_y = maze.len() - 1
                                } else {
                                    next_y -= 1
                                };

                                match maze[next_y][x] {
                                    Tile::Void => { continue }
                                    Tile::Wall => { break }
                                    Tile::Open => { y = next_y; break }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    1000 * (y+1) + 4* (x+1) + (dir_index%4)
}


pub fn part2(input_file: &str) -> isize {
    let (maze, instrucrions) = parse_input(input_file);

    0
}


fn main() {
    let p1 = part1("input/day22.txt");
    println!("Result 1 - {p1:?}");
    // let p2 = part2("input/day22.txt");
    // println!("Result 2 - {p2:?}");
}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1("input/day22_example.txt"), 6032)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input/day22.txt"), 75254)
    }

    #[ignore]
    #[test]
    fn test_part2_example() {
        assert_eq!(part2("input/day22_example.txt"), 5031)
    }

    #[ignore]
    #[test]
    fn test_part2() {
        assert_eq!(part2("input/day22.txt"), 0)
    }

}