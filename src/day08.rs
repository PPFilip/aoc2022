#![allow(dead_code)]
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct Visibility {
    up: bool,
    down: bool,
    left: bool,
    right: bool
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility {
            up: true,
            down: true,
            left: true,
            right: true
        }
    }
}

#[derive(Debug)]
struct VisibilityCount {
    up: u32,
    down: u32,
    left: u32,
    right: u32
}

impl Default for VisibilityCount {
    fn default() -> Self {
        VisibilityCount {
            up: 0,
            down: 0,
            left: 0,
            right: 0
        }
    }
}

pub fn part1(input_file: &str) -> u32 {
    let file = File::open(input_file).unwrap();
    let mut tree_map:Vec<Vec<char>> = Vec::new();
    let mut vis_map:Vec<Vec<Visibility>> = Vec::new();

    for l in io::BufReader::new(file).lines() {
        let line = l.unwrap();
        let tree_row: Vec<char> = line.chars().collect();

        let mut row_visibility:Vec<Visibility> = Vec::new();
        for _ in 0..tree_row.len() {
            row_visibility.push(Visibility::default());
        }
        vis_map.push(row_visibility);
        tree_map.push(tree_row);
    }

    let col_len = tree_map.get(0).unwrap().len();
    let row_len = tree_map.len();


    for row in 1..row_len-1 {
        let mut tallest_left = tree_map[row][0];
        for col in 1..col_len-1 {
            let act = tree_map[row][col];
            if act <= tallest_left {
                vis_map[row][col].left = false;
            } else {
                tallest_left = act;
            }
        }


        let mut tallest_right = tree_map[row][col_len-1];
        for col in (1..col_len-1).rev() {
            let act = tree_map[row][col];
            if act <= tallest_right {
                vis_map[row][col].right = false;
            } else {
                tallest_right = act;
            }
        }
    }

    for col in 1..col_len-1 {
        let mut tallest_up = tree_map[0][col];
        for row in 1..row_len-1 {
            let act = tree_map[row][col];
            if act <= tallest_up {
                vis_map[row][col].up = false;
            } else {
                tallest_up = act;
            }
        }

        let mut tallest_down = tree_map[row_len-1][col];
        for row in (1..row_len-1).rev() {
            let act = tree_map[row][col];
            if act <= tallest_down {
                vis_map[row][col].down = false;
            } else {
                tallest_down = act;
            }
        }

    }

    let mut count = 0_u32;

    for x in vis_map.iter() {
        for v in x.iter() {
            if v.up || v.down || v.left || v.right  {
                count += 1;
            }
        }
    }

    count
}

pub fn part2(input_file: &str) -> u32 {
    let file = File::open(input_file).unwrap();
    let mut tree_map:Vec<Vec<char>> = Vec::new();
    let mut vis_count:Vec<Vec<VisibilityCount>> = Vec::new();

    for l in io::BufReader::new(file).lines() {
        let line = l.unwrap();
        let v: Vec<char> = line.chars().collect();

        let mut ww:Vec<VisibilityCount> = Vec::new();
        for _ in 0..v.len() {
            ww.push(VisibilityCount::default());
        }
        tree_map.push(v);
        vis_count.push(ww);
    }

    let col_len = tree_map.get(0).unwrap().len();
    let row_len = tree_map.len();

    for row in 0..row_len {
        for col in 0..col_len {
            let reference_tree = tree_map[row][col];

            for irow in row+1..row_len {
                let compare_tree = tree_map[irow][col];
                vis_count[row][col].down += 1;
                if compare_tree >= reference_tree {
                    break;
                }
            }

            for irow in (0..row).rev() {
                let compare_tree = tree_map[irow][col];
                vis_count[row][col].up += 1;
                if compare_tree >= reference_tree {
                    break;
                }
            }

            for icol in col+1..col_len {
                let compare_tree = tree_map[row][icol];
                vis_count[row][col].right += 1;
                if reference_tree > compare_tree {
                } else {
                    break;
                }
            }

            for icol in (0..col).rev() {
                let compare_tree = tree_map[row][icol];
                vis_count[row][col].left += 1;
                if reference_tree > compare_tree {
                } else {
                    break;
                }
            }
        }
    }

    let mut max_scenic_score = 0_u32;

    for x in vis_count.iter() {
        for v in x.iter() {
            let scenic_score = v.up * v.down * v.left * v.right;
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score
            }
        }
    }

    max_scenic_score
}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1("input/day08_example.txt"), 21)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input/day08.txt"), 1823)
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2("input/day08_example.txt"), 8)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input/day08.txt"), 211680)
    }

}