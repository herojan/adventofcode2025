use crate::Rotation::L;
use crate::Rotation::R;
use std::fs::read_to_string;
use std::str::FromStr;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let rotations = input
        .lines()
        .map(|line| line.parse::<Rotation>().unwrap())
        .collect::<Vec<_>>();

    println!("{:?}", part1(rotations.to_vec()));
    println!("{:?}", part2(rotations.to_vec()))
}

fn part1(rotations: Vec<Rotation>) -> i32 {
    let mut pos: i32 = 50;
    let mut zeroes = 0;
    for rotation in rotations {
        pos = match rotation {
            L(n) => (pos - n as i32).rem_euclid(100),
            R(n) => (pos + n as i32).rem_euclid(100),
        };

        if pos == 0 {
            zeroes += 1;
        }
    }

    zeroes
}

fn part2(rotations: Vec<Rotation>) -> u16 {
    let mut pos: u16 = 50;
    let mut zeroes = 0;
    for rotation in rotations {
        match rotation {
            L(n) => {
                zeroes += n.div_euclid(100);
                let to_move = n.rem_euclid(100);
                let maybe_invalid = pos as i32 - to_move as i32;
                if maybe_invalid < 0 {
                    if pos != 0 {
                        zeroes += 1;
                    }
                    pos = 100 - maybe_invalid.abs() as u16;
                } else if maybe_invalid == 0 {
                    if pos != 0 {
                        zeroes += 1;
                    }
                    pos = 0;
                } else {
                    pos = maybe_invalid as u16;
                }
            },
            R(n) => {
                let maybe_invalid = pos + n;
                zeroes += maybe_invalid.div_euclid(100);
                pos = maybe_invalid.rem_euclid(100);
            },
        }

    }

    zeroes
}


#[derive(Debug)]
#[derive(Clone)]
enum Rotation {
    L(u16),
    R(u16),
}

impl FromStr for Rotation {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, num) = s.split_at(1);
        let num = num.parse::<u16>().unwrap();

        match dir {
            "R" => Ok(R(num)),
            "L" => Ok(L(num)),
            _ => Err("Invalid rotation".to_string()),
        }
    }
}
