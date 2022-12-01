#![allow(dead_code)]

use std::cmp::max;
use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn day01part1() {
    let file = File::open("input/day01.txt").unwrap();
    let mut elf_sum = 0_u64;
    let mut elf_max = 0_u64;
    for l in io::BufReader::new(file).lines() {
        let line = l.unwrap();
        if line == "" {
            elf_max = max(elf_sum, elf_max);
            elf_sum = 0;
        } else {
            elf_sum += line.parse::<u64>().unwrap();
        }
    }
    println!("{}", elf_max);
}

pub fn day01part2() {
    let file = File::open("input/day01.txt").unwrap();
    let mut elf_sum = 0_u64;
    let mut elf_vec = Vec::new();

    for l in io::BufReader::new(file).lines() {
        let line = l.unwrap();
        if line == "" {
            elf_vec.push(elf_sum);
            elf_sum = 0;
        } else {
            elf_sum += line.parse::<u64>().unwrap();
        }
    }

    elf_vec.sort();
    elf_vec.reverse();

    let mut elf_sum = 0_u64;
    for v in &elf_vec[0..3] {
        elf_sum += v;
    }
    println!("{}", elf_sum);
}