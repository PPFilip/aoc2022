#![allow(dead_code)]

use std::cmp::max;
use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn day02part1() {
    let file = File::open("input/day02.txt").unwrap();
    let mut points = 0;

    for l in io::BufReader::new(file).lines() {
        let line = l.unwrap();
        let v: Vec<&str> = line.split(' ').collect();
        let our_hand = v.get(1).unwrap().to_owned();
        let _opp_hand = v.get(0).unwrap().to_owned();

        let we_win =  v == ["A", "Y"] || v == ["B", "Z"] || v == ["C", "X"];
        let we_draw =  v == ["A", "X"] || v == ["B", "Y"] || v == ["C", "Z"];

        points += match our_hand {
            "X" => 1,
            "Y" => 2,
            _ => 3
        };

        if we_win {
            points += 6
        } else if we_draw {
            points += 3
        }

    }

    println!("{}", points);
}

pub fn day02part2() {
    let file = File::open("input/day02.txt").unwrap();
    let mut points = 0;

    for l in io::BufReader::new(file).lines() {
        let line = l.unwrap();
        let v: Vec<&str> = line.split(' ').collect();
        let our_choice = v.get(1).unwrap().to_owned();
        let opp_hand = v.get(0).unwrap().to_owned();

        let our_hand = match opp_hand {
            "A" => match our_choice {
                "X" => "Z",
                "Y" => "X",
                "Z" => "Y",
                _ => ""
            },
            "B" => match our_choice {
                "X" => "X",
                "Y" => "Y",
                "Z" => "Z",
                _ => ""
            },
            "C" => match our_choice {
                "X" => "Y",
                "Y" => "Z",
                "Z" => "X",
                _ => ""
            },
            _ => ""
        };


        let we_win =
            (opp_hand == "A"  && our_hand == "Y") ||
                (opp_hand == "B" && our_hand == "Z") ||
                (opp_hand == "C" && our_hand == "X");

        let we_draw =
            (opp_hand == "A"  && our_hand == "X") ||
                (opp_hand == "B" && our_hand == "Y") ||
                (opp_hand == "C" && our_hand == "Z");

        points += match our_hand {
            "X" => 1,
            "Y" => 2,
            _ => 3
        };

        if we_win {
            points += 6
        } else if we_draw {
            points += 3
        }

    }

    println!("{}", points);
}
