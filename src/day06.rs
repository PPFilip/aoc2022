#![allow(dead_code)]
use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn day06part1() {
    let file = File::open("input/day06.txt").unwrap();

    for l in io::BufReader::new(file).lines() {
        let line = l.unwrap();
        let mut v: Vec<char> = Vec::new();
        let mut pos = 0_u32;
        for c in line.chars() {
            pos += 1;
            v.push(c);
            if v.len() >= 4 {
                if v.len() == 5 {
                    v.remove(0);
                }

                let mut vv = v.clone();
                vv.sort();
                vv.dedup();
                if vv.len() == 4 {
                    println!("{}", pos);
                    break;
                }
            }
        }
    }


}

pub fn day06part2() {
        let file = File::open("input/day06.txt").unwrap();

    for l in io::BufReader::new(file).lines() {
        let line = l.unwrap();
        let mut v: Vec<char> = Vec::new();
        let mut pos = 0_u32;
        for c in line.chars() {
            pos += 1;
            v.push(c);
            if v.len() >= 14 {
                if v.len() == 15 {
                    v.remove(0);
                }

                let mut vv = v.clone();
                vv.sort();
                vv.dedup();
                if vv.len() == 14 {
                    println!("{}", pos);
                    break;
                }
            }
        }
    }


}