use std::str::FromStr;

use regex::Regex;

mod vec2d;
pub use vec2d::Vec2D;

mod box2d;
pub use box2d::Box2D;

mod input;
pub use input::*;

pub mod graphs;

pub mod grid;

pub struct Solution {
    pub part1: Option<String>,
    pub part2: Option<String>,
}

trait PartSolution {
    fn as_part_solution(&self) -> Option<String>;
}


// Rusts's orphan rules prevent a generic implementation because ToString is not implemented
// for (), so do this manually here for all types that might be aoc solutions
macro_rules! impl_part_solution {
    ($($t:ty),*) => {
        $(
            impl PartSolution for $t {
                fn as_part_solution(&self) -> Option<String> {
                    Some(self.to_string())
                }
            }
        )*
    };
}
impl_part_solution!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, String, &str);

impl PartSolution for () {
    fn as_part_solution(&self) -> Option<String> {
        None
    }
}

impl<T: PartSolution> From<T> for Solution {
    fn from(part1: T) -> Self {
        Solution {
            part1: part1.as_part_solution(),
            part2: None,
        }
    }
}

impl<T: PartSolution, U: PartSolution> From<(T, U)> for Solution {
    fn from((part1, part2): (T, U)) -> Self {
        Solution {
            part1: part1.as_part_solution(),
            part2: part2.as_part_solution(),
        }
    }
}

pub trait Numbers {
    fn numbers<T>(&self) -> Vec<T>
    where
        T: num::Integer + FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug;
}

impl Numbers for &str {
    fn numbers<T>(&self) -> Vec<T>
    where
        T: num::Integer + FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        let reg = Regex::new(r"-?\d+").unwrap();
        reg.find_iter(self)
            .map(|m| m.as_str().parse::<T>().unwrap())
            .collect()
    }
}
