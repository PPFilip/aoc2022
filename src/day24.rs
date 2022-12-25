use std::fs::File;
use std::{io};
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{Read};


#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
enum Direction {Up, Down, Left, Right}

type Point = (isize, isize);
type BlizzardMap = HashMap<Point, Vec<Direction>>;
type WallMap = HashSet<Point>;
type OffsetMap = HashMap<Direction, (isize, isize, isize, isize)>;

const NEIGHBOURS:[(isize, isize); 5] = [(0, 0), (0, 1), (1, 0), (0, -1), (-1, 0)];


fn parse_input(input_file: &str) -> (BlizzardMap, WallMap, Point, Point, OffsetMap) {
    let file = File::open(input_file).unwrap();
    let mut blizzards: BlizzardMap = HashMap::new();
    let mut walls: WallMap = HashSet::new();

    let mut input = "".to_string();
    let _ = io::BufReader::new(file).read_to_string(&mut input);

    let x = (input.lines().count() - 2) as isize;
    let y = (input.find('\n').unwrap() - 2) as isize;

    for (i, line) in input.lines().enumerate() {
        let i = i as isize;
        for (j, c) in line.chars().enumerate() {
            let j = j as isize;
            match c {
                '#' => { walls.insert((i, j)); },
                '^' => { blizzards.insert((i, j), vec![Direction::Up]); },
                'v' => { blizzards.insert((i, j), vec![Direction::Down]); },
                '>' => { blizzards.insert((i, j), vec![Direction::Right]); },
                '<' => { blizzards.insert((i, j), vec![Direction::Left]); },
                '.' => (),
                _ => panic!("wtf")
            }
        }
    }

    let offsets: OffsetMap = HashMap::from(
        [
            (Direction::Up, (-1, 0, 1, x)),
            (Direction::Down, (1, 0, x, 1)),
            (Direction::Left, (0, -1, 1, y)),
            (Direction::Right, (0, 1, y, 1)),
        ]
    );

    walls.insert((-1, 1));
    walls.insert((x+2, y));

    let start = (0, 1);
    let end = (x+1, y);

    (blizzards, walls, start, end, offsets)
}


fn play_turn(blizzards: &BlizzardMap, offsets: &OffsetMap) -> BlizzardMap {
    let mut new_state = BlizzardMap::new();

    for (pos, directions) in blizzards {
        for d in directions {
            let offset = offsets[d];
            let mut new_position = (pos.0 + offset.0, pos.1 + offset.1);
            if (new_position.0 > pos.0 && new_position.0 > offset.2)
                || (new_position.0 < pos.0 && new_position.0 < offset.2) {
                new_position.0 = offset.3
            } else if (new_position.1 > pos.1 && new_position.1 > offset.2)
                || (new_position.1 < pos.1 && new_position.1 < offset.2) {
                new_position.1 = offset.3
            }

            if let std::collections::hash_map::Entry::Vacant(e) = new_state.entry(new_position) {
                e.insert(vec![*d]);
            } else {
                new_state.get_mut(&new_position).unwrap().push(*d);
            }
        }
    }

    new_state
}


fn get_neighbours(p: Point, blizzards: &BlizzardMap, walls: &WallMap) -> Vec<Point> {
    NEIGHBOURS.iter().map(|n| (n.0 + p.0, n.1 + p.1))
        .filter(|p| !blizzards.contains_key(p) && !walls.contains(p))
        .collect()
}


fn bfs(start: Vec<Point>, end: Vec<Point>, blizzards: &BlizzardMap, walls: &WallMap, offsets: &OffsetMap) -> usize {
    let mut starters = VecDeque::from(start);
    let mut enders = VecDeque::from(end);

    let mut turns = 0;
    let mut queue : VecDeque<Point> = VecDeque::from([starters.pop_front().unwrap()]);
    let mut current_end = enders.pop_front().unwrap();

    let mut map = blizzards.clone();

    loop {
        turns += 1;
        map = play_turn(&map, offsets);

        let mut next_queue : VecDeque<Point> = VecDeque::new();

        while !queue.is_empty() {
            let pos = queue.pop_front().unwrap();
            if pos == current_end {
                if !starters.is_empty() {
                    current_end = enders.pop_front().unwrap();
                    next_queue = VecDeque::from([starters.pop_front().unwrap()]);
                    break
                } else {
                    return turns-1
                }
            }

            for n in get_neighbours(pos, &map, walls) {
                if !next_queue.contains(&n) {
                    next_queue.push_back(n);
                }
            }
        }

        if next_queue.is_empty() {
            panic!()
        } else {
            queue = next_queue;
        }
    }
}


pub fn part1(input_file: &str) -> usize {
    let (blizzards, walls, start, end, offsets) = parse_input(input_file);

    bfs(vec![start], vec![end], &blizzards, &walls, &offsets)
}


pub fn part2(input_file: &str) -> usize {
    let (blizzards, walls, start, end, offsets) = parse_input(input_file);

    bfs(vec![start, end, start], vec![end, start, end], &blizzards, &walls, &offsets)
}


fn main() {
    let p1 = part1("input/day24.txt");
    println!("Result 1 - {p1:?}");
    let p2 = part2("input/day24.txt");
    println!("Result 2 - {p2:?}");
}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1("input/day24_example.txt"), 10)
    }

    #[test]
    fn test_part1_example2() {
        assert_eq!(part1("input/day24_example2.txt"), 18)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input/day24.txt"), 326)
    }

    #[test]
    fn test_part2_example2() {
        assert_eq!(part2("input/day24_example2.txt"), 54)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input/day24.txt"), 976)
    }

}
