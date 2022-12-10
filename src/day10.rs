use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn part1(input_file: &str) -> i32 {
    let file = File::open(input_file).unwrap();
    let mut commands: Vec<i32> = Vec::new();
    let mut register = 1_i32;
    let mut result = 0_i32;

    for l in io::BufReader::new(file).lines() {
        let line = l.unwrap();
        let command: Vec<&str> = line.split(' ').collect();
        match command[0] {
            "noop" => commands.push(0),
            "addx" => {
                commands.push(0);
                commands.push(command[1].parse().unwrap())
            },
            _ => panic!("wtf")
        };
    }

    for (i, x) in commands.iter().enumerate() {
        let step = i as i32 + 1;
        if (step+20)%40 == 0 {
            result += register * step;
        }
        register += x;

    }

    result
}

pub fn part2(input_file: &str) -> String {
    let file = File::open(input_file).unwrap();
    let mut commands: Vec<i32> = Vec::new();
    let mut register = 1_i32;

    for l in io::BufReader::new(file).lines() {
        let line = l.unwrap();
        let command: Vec<&str> = line.split(' ').collect();
        match command[0] {
            "noop" => commands.push(0),
            "addx" => {
                commands.push(0);
                commands.push(command[1].parse().unwrap())
            },
            _ => panic!("wtf")
        };
    }

    let mut crt = [['.';40];6];

    for (i, x) in commands.iter().enumerate() {
        let col = (i as i32) % 40;

        if register-1 <= col && col <= register +1 {
            crt[i/40][i%40] = '#';
        }

        register += x;

    }

    // print_crt(crt);
    // added just so the tests can be run
    debug_crt(crt)
}

#[allow(dead_code)]
fn print_crt(crt: [[char;40];6]) {
    for x in crt.iter() {
        let line: String = x.iter().collect();
        println!("{line}");
    }
}

fn debug_crt(crt: [[char;40];6]) -> String {
    let mut res = "".to_string();
    for x in crt.iter() {
        let line: String = x.iter().collect();
        res.push_str(&*line);
    }
    res
}

fn main() {
    let p1 = part1("input/day10.txt");
    println!("Result 1 - {p1:?}");
    let p2 = part2("input/day10.txt");
    println!("Result 2 - {p2:?}");
}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1("input/day10_example.txt"), 13140)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input/day10.txt"), 17840)
    }

    // ##..##..##..##..##..##..##..##..##..##.
    // ###...###...###...###...###...###...###
    // ####....####....####....####....####...
    // #####.....#####.....#####.....#####....
    // ######......######......######......###
    // #######.......#######.......#######....
    const RES_P2_EX: &str = "##..##..##..##..##..##..##..##..##..##..###...###...###...###...###...###...###.####....####....####....####....####....#####.....#####.....#####.....#####.....######......######......######......###########.......#######.......#######.....";

    #[test]
    fn test_part2_example() {
        assert_eq!(part2("input/day10_example.txt"), RES_P2_EX)
    }

    // ####..##..#.....##..#..#.#....###...##.
    // #....#..#.#....#..#.#..#.#....#..#.#..#
    // ###..#..#.#....#....#..#.#....#..#.#...
    // #....####.#....#.##.#..#.#....###..#.##
    // #....#..#.#....#..#.#..#.#....#....#..#
    // ####.#..#.####..###..##..####.#.....###
    const RES_P2: &str = "####..##..#.....##..#..#.#....###...##..#....#..#.#....#..#.#..#.#....#..#.#..#.###..#..#.#....#....#..#.#....#..#.#....#....####.#....#.##.#..#.#....###..#.##.#....#..#.#....#..#.#..#.#....#....#..#.####.#..#.####..###..##..####.#.....###.";

    #[test]
    fn test_part2() {
        assert_eq!(part2("input/day10.txt"), RES_P2)
    }

}