#![allow(dead_code)]
use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn day04part1() {
    let file = File::open("input/day04.txt").unwrap();
    let mut points = 0_u32;

    for l in io::BufReader::new(file).lines() {
        let line = l.unwrap();
        let v: Vec<&str> = line.split(',').collect();
        let a: Vec<&str> = v.get(0).unwrap().split('-').collect();
        let b: Vec<&str> = v.get(1).unwrap().split('-').collect();
        let ax: u32 = a.get(0).unwrap().parse().unwrap();
        let ay: u32 = a.get(1).unwrap().parse().unwrap();
        let bx: u32 = b.get(0).unwrap().parse().unwrap();
        let by: u32 = b.get(1).unwrap().parse().unwrap();

        if (ax >= bx  &&  ay <= by) || (bx >= ax && by <= ay) {
            points += 1
        }

    }


    println!("{}", points);
}

pub fn day04part2() {
    let file = File::open("input/day04.txt").unwrap();
    let mut points = 0_u32;

    for l in io::BufReader::new(file).lines() {
        let line = l.unwrap();
        let v: Vec<&str> = line.split(',').collect();
        let a: Vec<&str> = v.get(0).unwrap().split('-').collect();
        let b: Vec<&str> = v.get(1).unwrap().split('-').collect();
        let ax: u32 = a.get(0).unwrap().parse().unwrap();
        let ay: u32 = a.get(1).unwrap().parse().unwrap();
        let bx: u32 = b.get(0).unwrap().parse().unwrap();
        let by: u32 = b.get(1).unwrap().parse().unwrap();

        if (bx <= ax && ax <= by) || (ax <= bx && bx <= ay) {
            points += 1
        }

    }


    println!("{}", points);
}