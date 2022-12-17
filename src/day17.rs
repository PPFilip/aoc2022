use std::cmp::{max};
use std::fs::{File};
use std::{io};
use std::collections::HashMap;
use std::io::{BufRead};
use crate::Moves::{LEFT, RIGHT};

const MAX_ITERATIONS_P1:usize = 2022;
const MAX_ITERATIONS_P2:usize = 1_000_000_000_000;

const MAX_WIDTH: usize = 9;
const MAX_HEIGHT: usize = 4 * MAX_ITERATIONS_P1 + 5;
type Chamber = [[bool;MAX_WIDTH];MAX_HEIGHT];

#[derive(Debug)]
enum Moves {LEFT, RIGHT}

struct PuzzlePiece {
    shape: Vec<(usize, usize)>,
    // After submitting I realize these are not really necessary, as we can just shift position
    // and look up if puzzle pieces cover the thing .. but hey, it worked like this as well and
    // it is subtly faster for L and + piece :)
    fall_cond: Vec<(usize, usize)>,
    left_cond: Vec<(usize, usize)>,
    right_cond: Vec<(usize, usize)>,
    height: usize
}

impl PuzzlePiece {
    fn can_fall(&self, x:usize, y:usize, chamber: &Chamber) -> bool {
        for (cx, cy) in &self.fall_cond {
            if chamber[y+cy][x+cx] {
                return false
            }
        }

        true
    }

    fn shift_move(&self, m:&Moves, x:usize, y:usize, chamber: &Chamber) -> usize {

        let conditions = match m {
            LEFT => &self.left_cond,
            RIGHT => &self.right_cond,
        };

        for (cx, cy) in conditions {
            if chamber[y+cy][x+cx] {
                return x
            }
        }

        match m {
            LEFT => x-1,
            RIGHT => x+1,
        }
    }

}

fn parse_input(input_file: &str) -> Vec<Moves> {
    let file = File::open(input_file).unwrap();

    let mut res : Vec<Moves> = Vec::new();
    if let Ok(line) = io::BufReader::new(file).lines().next().unwrap() {
        for c in line.chars() {
            let m = match c {
                '<' => LEFT,
                '>' => RIGHT,
                _ => panic!("wtf")
            };
            res.push(m);
        }
    } else {
        panic!("wtf")
    };

    res
}

#[allow(dead_code)]
fn debug_chamber(chamber: Chamber, y_min:usize, y_max:usize) {
    for y in (y_min .. y_max).rev() {
        print!("|");
        for x in 0 .. MAX_WIDTH {
            print!("{}", if chamber[y][x] {'#'} else {'.'});
        }
        println!("|{y}");
    }
    println!("-{}-", String::from_utf8(vec![b'-';MAX_WIDTH]).unwrap());
}

fn init_puzzles() -> Vec<PuzzlePiece> {
    let mut puzzles : Vec<PuzzlePiece> = Vec::new();

    let puzzle_hor_line = PuzzlePiece {
        shape: vec![(1,1), (2,1), (3,1), (4,1)],
        fall_cond: vec![(1,0), (2,0), (3,0), (4,0)],
        left_cond: vec![(0,1)],
        right_cond: vec![(5,1)],
        height: 1
    };
    puzzles.push(puzzle_hor_line);

    let puzzle_cross = PuzzlePiece {
        shape: vec![(2,1), (2,2), (2,3), (1,2), (3,2)],
        fall_cond: vec![(1,1), (2,0), (3,1)],
        left_cond: vec![(1,1), (0,2), (1,3)],
        right_cond: vec![(3,1), (4,2), (3,3)],
        height: 3
    };
    puzzles.push(puzzle_cross);

    let puzzle_l = PuzzlePiece {
        shape: vec![(1,1), (2,1), (3,1), (3,2), (3,3)],
        fall_cond: vec![(1,0), (2,0), (3,0)],
        left_cond: vec![(0,1), (2,2), (2,3)],
        right_cond: vec![(4,1), (4,2), (4,3)],
        height: 3
    };
    puzzles.push(puzzle_l);


    let puzzle_ver_line = PuzzlePiece {
        shape: vec![(1,1), (1,2), (1,3), (1,4)],
        fall_cond: vec![(1,0)],
        left_cond: vec![(0,1), (0,2), (0,3), (0,4)],
        right_cond: vec![(2,1), (2,2), (2,3), (2,4)],
        height: 4
    };
    puzzles.push(puzzle_ver_line);

    let puzzle_square = PuzzlePiece {
        shape: vec![(1,1), (2,1), (1,2), (2,2)],
        fall_cond: vec![(1,0), (2,0)],
        left_cond: vec![(0,1), (0,2)],
        right_cond: vec![(3,1), (3,2)],
        height: 2
    };
    puzzles.push(puzzle_square);

    puzzles
}

fn init_chamber() -> Chamber {
    let mut chamber: Chamber = [[false;MAX_WIDTH];MAX_HEIGHT];
    chamber[0] = [true;MAX_WIDTH];

    for y in 0 .. MAX_HEIGHT {
        chamber[y][0] = true;
        chamber[y][MAX_WIDTH-1] = true;
    }

    chamber
}

pub fn part1(input_file: &str) -> usize {
    let moves = parse_input(input_file);
    let mut chamber = init_chamber();

    let puzzles = init_puzzles();
    let mut act_height : usize = 0;

    let mut puzzle_count = 0;
    let mut move_count = 0;


    while puzzle_count < MAX_ITERATIONS_P1 {
        let puzzle = &puzzles[puzzle_count % puzzles.len()];
        let mut puzzle_y = act_height + 3;
        let mut puzzle_x : usize = 2;

        loop {
            let my_move = &moves[move_count % moves.len()];
            puzzle_x = puzzle.shift_move(my_move, puzzle_x, puzzle_y, &chamber);
            move_count += 1;

            if puzzle.can_fall(puzzle_x, puzzle_y, &chamber) {
                puzzle_y -= 1;
            } else {
                break
            }
        }

        for (px, py) in &puzzle.shape {
            chamber[puzzle_y + py][puzzle_x + px] = true;
        }

        act_height = max(act_height, puzzle_y + puzzle.height);
        puzzle_count += 1;
    }

    act_height
}

fn chamber_profile(chamber: &Chamber, act_height: usize) -> [usize;MAX_WIDTH] {
    let mut profile = [act_height;MAX_WIDTH];
    profile[0] = 0;
    profile[MAX_WIDTH-1] = 0;

    for y in 0..MAX_HEIGHT {
        for x in 1..MAX_WIDTH-1 {
            if chamber[y][x] {
                profile[x] = act_height-y;
            }
        }
    }

    profile
}

pub fn part2(input_file: &str, max_iterations: usize) -> usize {
    let moves = parse_input(input_file);
    let mut chamber = init_chamber();

    let puzzles = init_puzzles();
    let mut act_height : usize = 0;

    let mut puzzle_count = 0;
    let mut move_count = 0;

    // Added cache based on - https://www.reddit.com/r/adventofcode/comments/znykq2/comment/j0kc9qp/
    // I was originally looking for filled horizontal lines, which yielded a solution but it was slow
    // and ineffective compared to this.
    #[derive(Clone, Copy, Default, Hash, Eq, PartialEq, Debug)]
    struct CacheKey {
        move_index: usize,
        puzzle_index: usize,
        height_profile: [usize; MAX_WIDTH],
    }

    #[derive(Clone, Copy, Default, Hash, Eq, PartialEq, Debug)]
    struct CacheValue {
        move_index: usize,
        act_height: usize,
    }

    let mut cache = HashMap::<CacheKey, CacheValue>::new();

    let mut bonus = 0;
    let mut finishing = false;

    while puzzle_count < max_iterations {
        let puzzle = &puzzles[puzzle_count % puzzles.len()];
        let mut puzzle_y = act_height + 3;
        let mut puzzle_x : usize = 2;

        loop {
            let my_move = &moves[move_count % moves.len()];
            puzzle_x = puzzle.shift_move(my_move, puzzle_x, puzzle_y, &chamber);
            move_count += 1;

            if puzzle.can_fall(puzzle_x, puzzle_y, &chamber) {
                puzzle_y -= 1;
            } else {
                break
            }
        }

        for (px, py) in &puzzle.shape {
            chamber[puzzle_y + py][puzzle_x + px] = true;
        }

        act_height = max(act_height, puzzle_y + puzzle.height);
        puzzle_count += 1;

        if !finishing {
            let cache_key = CacheKey { move_index: move_count % moves.len(), puzzle_index: puzzle_count % puzzles.len(), height_profile: chamber_profile(&chamber, act_height) };
            let cache_value = CacheValue { move_index: puzzle_count, act_height };

            if let Some(last_value) = cache.insert(cache_key, cache_value) {
                let cycle_length = puzzle_count - last_value.move_index;
                let cycles_to_go = (max_iterations - puzzle_count) / cycle_length;

                puzzle_count += cycles_to_go * cycle_length;
                bonus = (cycles_to_go) * (cache_value.act_height - last_value.act_height);
                finishing = true;
            }

        }

    }

    act_height + bonus
}

fn main() {
    let p1 = part1("input/day17.txt");
    println!("Result 1 - {p1:?}");
    let p2 = part2("input/day17.txt", MAX_ITERATIONS_P2);
    println!("Result 2 - {p2:?}");
}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1("input/day17_example.txt"), 3068)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input/day17.txt"), 3181)
    }

    #[test]
    fn test_part1_with_part2_example() {
        assert_eq!(part2("input/day17_example.txt", MAX_ITERATIONS_P1), 3068)
    }

    #[test]
    fn test_part1_with_part2 () {
        assert_eq!(part2("input/day17.txt", MAX_ITERATIONS_P1), 3181)
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2("input/day17_example.txt", MAX_ITERATIONS_P2), 1514285714288)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input/day17.txt", MAX_ITERATIONS_P2), 1570434782634)
    }

}