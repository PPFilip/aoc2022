#![allow(dead_code)]
use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn day05part1() {
    let file = File::open("input/day05_parsed.txt").unwrap();
    const NUM_STACKS:usize = 9;

    const NEW_VEC: Vec<char> = Vec::new();
    let mut stacks= [NEW_VEC; NUM_STACKS];

    let mut lnum = 0;
    for l in io::BufReader::new(file).lines() {
        lnum += 1;
        let line = l.unwrap();

        if lnum <= NUM_STACKS {
            for c in line.chars() {
                stacks[lnum-1].push(c);
            }
        } else {
            let v: Vec<&str> = line.split(' ').collect();
            let move_count:usize = v.get(1).unwrap().parse().unwrap();
            let from_stack:usize = v.get(3).unwrap().parse().unwrap();
            let to_stack:usize = v.get(5).unwrap().parse().unwrap();
            println!("{move_count:?} {from_stack:?} {to_stack:?}");

            for _ in 0 .. move_count {
                let cpop = stacks[from_stack-1].pop().unwrap();
                println!("Moving {cpop} from {from_stack} to {to_stack}");

                stacks[to_stack-1].push(cpop);
            }

        }

    }

    println!("Stacks: {stacks:?}");

    print!("Result: ");
    for mut s in stacks {
        print!("{}", s.pop().unwrap());
    }
    println!();



}

pub fn day05part2() {

    let file = File::open("input/day05_parsed.txt").unwrap();
    const NUM_STACKS:usize = 9;

    const NEW_VEC: Vec<char> = Vec::new();
    let mut stacks= [NEW_VEC; NUM_STACKS];

    let mut lnum = 0;
    for l in io::BufReader::new(file).lines() {
        lnum += 1;
        let line = l.unwrap();

        if lnum <= NUM_STACKS {
            for c in line.chars() {
                stacks[lnum-1].push(c);
            }
        } else {
            let v: Vec<&str> = line.split(' ').collect();
            let move_count:usize = v.get(1).unwrap().parse().unwrap();
            let from_stack:usize = v.get(3).unwrap().parse().unwrap();
            let to_stack:usize = v.get(5).unwrap().parse().unwrap();
            println!("{move_count:?} {from_stack:?} {to_stack:?}");

            let mut backpop: Vec<char> = Default::default();
            for _ in 0 .. move_count {
                let cpop = stacks[from_stack-1].pop().unwrap();
                println!("Storing {cpop} from {from_stack}");
                backpop.push(cpop);
            }

            backpop.reverse();
            for c in backpop {
                println!("Pushing {c} from {from_stack} to {to_stack}");
                stacks[to_stack-1].push(c);
            }

        }

    }

    println!("Stacks: {stacks:?}");

    print!("Result: ");
    for mut s in stacks {
        print!("{}", s.pop().unwrap());
    }
    println!();


}