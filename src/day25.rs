use std::fs::File;
use std::{io};
use std::io::BufRead;
use std::str::FromStr;
use std::string::ParseError;

struct Snafu {
    value: i64
}

impl FromStr for Snafu {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut val = 0_i64;

        for c in input.chars() {
            let dv = match c {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '-' => -1,
                '=' => -2,
                _ => panic!("wtf")
            };
            val *= 5;
            val += dv;
        }

        Ok(Snafu{value: val })
    }
}

impl ToString for Snafu {
    fn to_string(&self) -> String {
        let mut chars = Vec::new();
        let mut num = self.value;

        while num > 0 {
            let digit = (num % 5) as u32;

            match digit {
                0 | 1 | 2 => {
                    chars.push(std::char::from_digit(digit, 10).unwrap());
                },
                3 => {
                    chars.push('=');
                    num += 2;
                },
                _ => {
                    chars.push('-');
                    num += 1;
                }
            };

            num /= 5;
        }
        chars.reverse();
        chars.into_iter().collect()
    }
}

pub fn part1(input_file: &str) -> String {
    let file = File::open(input_file).unwrap();
    let snafu: i64 = io::BufReader::new(file).lines().map(|l| l.unwrap().parse::<Snafu>().unwrap().value).sum();
    Snafu{value: snafu}.to_string()
}


fn main() {
    let p1 = part1("input/day25.txt");
    println!("Result 1 - {p1:?}");
}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1("input/day25_example.txt"), "2=-1=0".to_string())
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input/day25.txt"), "2=2-1-010==-0-1-=--2".to_string())
    }

}