use std::fs::File;
use std::{io};
use std::collections::{HashSet};
use std::io::{BufRead};
use std::primitive::i128;
use itertools::{all, iproduct};

type Point = (i128, i128);

#[derive(Debug, Clone)]
struct Sensor {
    pos: Point,
    beacon: Point
}

impl Sensor {
    fn beacon_distance(&self) -> i128 {
        manhattan(self.pos, self.beacon)
    }

    fn range_intersecting_at_y(&self, level: i128) -> Point {
        let (px, py) = self.pos;
        let dist = self.beacon_distance();

        let pointy = if level < py {
            (px, py - dist)
        } else {
            (px, py + dist)
        };

        let left = (px - dist, py);
        let right = (px + dist, py);

        let left_line = (left, pointy);
        let right_line = (pointy, right);

        let level_line = ((px - dist, level), (px + dist, level));

        let (lx, _ly) = line_intersection(left_line, level_line);
        let (rx, _ry) = line_intersection(right_line, level_line);

        (lx, rx)
    }

    fn get_slopes(&self) -> (HashSet<(Point, Point)>, HashSet<(Point, Point)>) {
        let (px, py) = self.pos;
        let dist = self.beacon_distance() + 1;

        let top = (px, py - dist);
        let bottom = (px, py + dist);
        let left = (px - dist, py);
        let right = (px + dist, py);

        (HashSet::from([(left, top), (bottom, right)]), HashSet::from([(top, right), (left, bottom)]))
    }
}

fn manhattan(p1: Point, p2: Point) -> i128 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

fn line_intersection(l1: (Point, Point), l2: (Point, Point)) -> Point {
    let xdiff = (l1.0.0 - l1.1.0, l2.0.0 - l2.1.0);
    let ydiff = (l1.0.1 - l1.1.1, l2.0.1 - l2.1.1);

    fn det(a: Point, b: Point) -> i128 {
        a.0 * b.1 - a.1 * b.0
    }

    let div = det(xdiff, ydiff);

    if div == 0 {
        panic!("wtf?");
    }

    let d = (det(l1.0, l1.1), det(l2.0, l2.1));
    let x = det(d, xdiff) / div;
    let y = det(d, ydiff) / div;

    (x,y)
}

fn parse_input(input_file: &str) -> Vec<Sensor> {
    let file = File::open(input_file).unwrap();

    let mut res: Vec<Sensor> = Vec::new();

    let re = regex::Regex::new(r"Sensor at x=([\-0-9]+), y=([\-0-9]+): closest beacon is at x=([\-0-9]+), y=([\-0-9]+)").unwrap();

    for l in io::BufReader::new(file).lines() {
        let line = l.unwrap();
        let caps = re.captures(line.as_str()).unwrap();
        let sx: i128 = caps.get(1).map_or("", |m| m.as_str()).parse().unwrap();
        let sy: i128 = caps.get(2).map_or("", |m| m.as_str()).parse().unwrap();
        let bx: i128 = caps.get(3).map_or("", |m| m.as_str()).parse().unwrap();
        let by: i128 = caps.get(4).map_or("", |m| m.as_str()).parse().unwrap();

        res.push(Sensor {pos: (sx, sy), beacon: (bx, by)});
    }

    res
}


pub fn part1(input_file: &str, level: i128) -> usize {
    let input = parse_input(input_file);
    let mut impossible: HashSet<i128> = HashSet::new();

    for s in input {
        if !( (s.pos.1 - s.beacon_distance() <= level) && (level <= s.pos.1 + s.beacon_distance()) ) {
            continue
        }

        let (left_intersect, right_intersect) = s.range_intersecting_at_y(level);

        for i in left_intersect .. right_intersect {
            impossible.insert(i);
        }
    }

    impossible.len()
}


pub fn part2(input_file: &str, level: i128) -> i128 {
    let input = parse_input(input_file);

    let mut all_l_slopes : HashSet<(Point, Point)> = HashSet::new();
    let mut all_r_slopes : HashSet<(Point, Point)> = HashSet::new();

    for s in input.clone() {
        let (lslope, rslope) = s.get_slopes();

        for slope in &lslope {
            all_l_slopes.insert(slope.clone());
        }

        for slope in &rslope {
            all_r_slopes.insert(slope.clone());
        }
    }

    for (left, right) in iproduct!(&all_l_slopes, &all_r_slopes) {
        let intersection = line_intersection(left.clone(), right.clone());
        let x = intersection.0;
        let y = intersection.1;

        if !((0 < x) && (x < level) && (0 < y) && (y < level)) {
            continue
        }

        if all(input.clone(), |s| manhattan(intersection, s.pos) > s.beacon_distance()) {
            return x * 4_000_000 + y
        }
    }

    0
}

fn main() {
    let p1 = part1("input/day15.txt", 2_000_000);
    println!("Result 1 - {p1:?}");
    let p2 = part2("input/day15_example.txt", 20);
    println!("Result 2 - {p2:?}");
}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1("input/day15_example.txt", 10), 26)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input/day15.txt", 2_000_000), 5256611)
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2("input/day15_example.txt", 20), 56000011)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input/day15.txt", 4_000_000), 13337919186981)
    }

}