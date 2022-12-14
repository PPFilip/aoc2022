use std::cmp::{max, min};
use std::fs::File;
use std::{io, isize};
use std::io::{BufRead};

const MAX_ROWS: usize = 1034;
const MAX_LINES: usize = 171;

fn parse_input(input_file: &str, bottom_at: usize) -> [[char;MAX_ROWS];MAX_LINES] {
    let file = File::open(input_file).unwrap();
        let mut max_x: isize = isize::MIN;
        let mut max_y: isize = isize::MIN;
        let mut min_x: isize = isize::MAX;
        let mut min_y: isize = isize::MAX;

    let mut cave = [['.';MAX_ROWS];MAX_LINES];

    for l in io::BufReader::new(file).lines() {
        let line = l.unwrap();
        let points: Vec<&str> = line.split(" -> ").collect();

        let mut prev_x = isize::MAX;
        let mut prev_y = isize::MAX;

        for (pi, p) in points.iter().enumerate() {
            let pc: Vec<&str> = p.split(',').collect();
            let x: isize = pc[0].parse().unwrap();
            let y: isize = pc[1].parse().unwrap();

            max_x = max(x, max_x);
            max_y = max(y, max_y);
            min_x = min(x, min_x);
            min_y = min(y, min_y);


            if pi == 0 {
                prev_x = x;
                prev_y = y;
                continue;
            }

            if prev_x == x {
                let step = (y-prev_y)/(y-prev_y).abs();
                let mut pos_y = prev_y;
                while pos_y != y {
                    cave[pos_y as usize][x as usize] = '#';
                    pos_y += step;
                }
                                    cave[pos_y as usize][x as usize] = '#';

                prev_x = x;
                prev_y = y;
                continue
            }

            if prev_y == y {
                let step = (x-prev_x)/(x-prev_x).abs();

                let mut pos_x = prev_x;
                while pos_x != x {
                    cave[y as usize][pos_x as usize] = '#';
                    pos_x += step;
                }
                                    cave[y as usize][pos_x as usize] = '#';

                prev_x = x;
                prev_y = y;
                continue
            }
        }

    }

    for x in 0 .. cave[0].len() {
        cave[max_y as usize + bottom_at][x] = 'A';
    }

    cave
}

#[allow(dead_code)]
fn debug_cave(cave: [[char;MAX_ROWS];MAX_LINES], x_off: usize, y_off:usize) {
    for y in 0 .. y_off {
        for x in x_off .. cave[y].len() {
            print!("{}", cave[y][x]);
        }
        println!();
    }
    println!();
}

pub fn part1(input_file: &str) -> usize {
    let mut cave = parse_input(input_file, 1);
    let mut sand = 0;

    'outer: loop {
        let (mut x, mut y) = (500_usize, 0_usize);
        sand += 1;

        loop {
            if cave[y+1][x] == 'A' {
                break 'outer;
            } else if cave[y+1][x] == '.' {
                y += 1;
                continue;
            } else {

                if cave[y+1][x-1] == '.' {
                    x -= 1;
                    y += 1;
                    continue;
                } else if cave[y+1][x+1] == '.' {
                    x += 1;
                    y += 1;
                    continue;
                } else {
                    cave[y][x] = 'O';
                    continue 'outer;
                }
            }
        }

    }

    sand - 1
}


pub fn part2(input_file: &str) -> usize {
   let mut cave = parse_input(input_file, 2);
    let mut sand = 0;

    'outer: loop {
        let (mut x, mut y) = (500_usize, 0_usize);
        sand += 1;

        if cave[y][x] != '.' {
            break 'outer;
        }

        loop {
            if cave[y+1][x] == '.' {
                y += 1;
                continue;
            } else {

                if cave[y+1][x-1] == '.' {
                    x -= 1;
                    y += 1;
                    continue;
                } else if cave[y+1][x+1] == '.' {
                    x += 1;
                    y += 1;
                    continue;
                } else {
                    cave[y][x] = 'O';
                    continue 'outer;
                }
            }
        }

    }

    sand - 1
}

fn main() {
    let p1 = part1("input/day14.txt");
    println!("Result 1 - {p1:?}");
    let p2 = part2("input/day14.txt");
    println!("Result 2 - {p2:?}");
}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1("input/day14_example.txt"), 24)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input/day14.txt"), 644)
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2("input/day14_example.txt"), 93)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input/day14.txt"), 27324)
    }

}