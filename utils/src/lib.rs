use regex::Regex;

mod vec2d;
pub use vec2d::Vec2D;

mod input;
pub use input::Input;

pub mod graphs;

pub mod grid;

pub struct Solution {
    pub part1: String,
    pub part2: String,
}

impl Solution {
    pub fn new<T1: std::fmt::Display, T2: std::fmt::Display>(p1: T1, p2: T2) -> Solution {
        Solution {
            part1: p1.to_string(),
            part2: p2.to_string(),
        }
    }
}

#[macro_export]
macro_rules! solution {
    () => {
        utils::solution!("")
    };
    ($part1: expr) => {
        utils::solution!($part1, "")
    };
    ($part1: expr, $part2: expr) => {
        Solution::new($part1, $part2)
    };
}

pub trait Numbers {
    fn numbers_i8(&self) -> Vec<i8>;
}

impl Numbers for &str {
    fn numbers_i8(&self) -> Vec<i8> {
        let reg = Regex::new(r"-?\d+").unwrap();
        reg.find_iter(self).map(|m| m.as_str().parse().unwrap()).collect()
    }
}

