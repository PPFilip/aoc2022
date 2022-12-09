use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn part1(input_file: &str) -> u32 {
    let file = File::open(input_file).unwrap();
    let mut commands: Vec<char> = Vec::new();
    let mut act_h_x = 0_i32;
    let mut act_h_y = 0_i32;
    let mut act_t_x = 0_i32;
    let mut act_t_y = 0_i32;

    let mut pos_tail_visited: HashSet<(i32,i32)> = HashSet::new();

    for l in io::BufReader::new(file).lines() {
        let line = l.unwrap();
        let command: Vec<&str> = line.split(' ').collect();
        let direction: char = command[0].chars().nth(0).unwrap();
        let count: usize = command[1].parse().unwrap();
        for _ in 0..count {
            commands.push(direction);
        }
    }

    for command in commands {
        let (x_inc, y_inc) = match command {
            'R' => (1, 0),
            'L' => (-1, 0),
            'U' => (0, -1),
            'D' => (0, 1),
            _ => (0, 0)
        };

        act_h_x += x_inc;
        act_h_y += y_inc;

        let (t_x_inc, t_y_inc) = match (act_h_x-act_t_x, act_h_y-act_t_y) {
            (0, 2) => (0, 1),
            (0, -2) => (0, -1),
            (2, 0) => (1, 0),
            (-2, 0) => (-1, 0),

            (1, -2) | (2, -1) => (1, -1),
            (1, 2)  | (2, 1) => (1, 1),
            (-2, 1) | (-1, 2) => (-1, 1),
            (-1, -2)| (-2,-1) => (-1, -1),

            _ => (0, 0)
        };

        act_t_x += t_x_inc;
        act_t_y += t_y_inc;

        pos_tail_visited.insert((act_t_x, act_t_y));
    }

    pos_tail_visited.len() as u32
}

pub fn part2(input_file: &str) -> u32 {
    let file = File::open(input_file).unwrap();
    let mut commands: Vec<char> = Vec::new();

    let mut snake:[(i32,i32);10] = [(0,0);10];
    let mut pos_tail_visited: HashSet<(i32,i32)> = HashSet::new();

    for l in io::BufReader::new(file).lines() {
        let line = l.unwrap();
        let command: Vec<&str> = line.split(' ').collect();
        let direction: char = command[0].chars().nth(0).unwrap();
        let count: usize = command[1].parse().unwrap();
        for _ in 0..count {
            commands.push(direction);
        }
    }

    for command in commands {
        let (x_inc, y_inc) = match command {
            'R' => (1, 0),
            'L' => (-1, 0),
            'U' => (0, -1),
            'D' => (0, 1),
            _ => panic!("wtf")
        };

        let (h_x, h_y) = snake[0];
        snake[0] = (h_x + x_inc, h_y + y_inc);

        for i in 1..10 {
            let (h_x, h_y) = snake[i-1];
            let (t_x, t_y) = snake[i];

            let (t_x_inc, t_y_inc) = match (h_x - t_x, h_y - t_y) {
                (0, 2) => (0, 1),
                (0, -2) => (0, -1),
                (2, 0) => (1, 0),
                (-2, 0) => (-1, 0),

                (1, -2) | (2, -1) | (2, -2) => (1, -1),
                (1, 2)  | (2, 1) | (2, 2) => (1, 1),
                (-2, 1) | (-1, 2) | (-2, 2) => (-1, 1),
                (-1, -2)| (-2,-1) | (-2, -2) => (-1, -1),

                _ => (0, 0)
            };

            snake[i] = (t_x+t_x_inc, t_y+t_y_inc);
        }

        pos_tail_visited.insert(snake[9]);
    }

    pos_tail_visited.len() as u32
}

fn main() {
    let p1 = part1("input/day09.txt");
    println!("Result 1 - {p1:?}");
    let p2 = part2("input/day09.txt");
    println!("Result 2 - {p2:?}");
}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1("input/day09_example.txt"), 13)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input/day09.txt"), 6081)
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2("input/day09_example.txt"), 1)
    }

    #[test]
    fn test_part2_example2() {
        assert_eq!(part2("input/day09_example2.txt"), 36)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input/day09.txt"), 2487)
    }

}