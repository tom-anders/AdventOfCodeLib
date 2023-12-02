use std::{str::FromStr, fmt::Display};

use regex::Regex;

pub mod math;

mod input;
pub use input::*;

pub mod graphs;

pub mod grid;
pub mod sparse_grid;

pub trait EvenMoreItertools: Iterator {
    fn sum_u64<I>(self) -> u64
    where
        Self: Iterator<Item = I> + Sized,
        I: Into<u64>,
    {
        self.map(I::into).sum()
    }
}

impl<T: ?Sized> EvenMoreItertools for T where T: Iterator {}

pub struct Solution {
    pub part1: Option<String>,
    pub part2: Option<String>,
}

impl Solution {
    pub fn copy_to_clipboard(&self) {
        let solution_to_copy = if self.part2.is_some() {
            &self.part2
        } else {
            &self.part1
        };

        if let Some(solution_to_copy) = solution_to_copy {
            std::process::Command::new("bash")
                .arg("-c")
                .arg(format!("echo {} | xclip -r -selection clipboard", solution_to_copy))
                .spawn()
                .expect("Failed to copy solution to clipboard");
        }
    }
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

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(part1) = &self.part1 {
            writeln!(f, "Solutions:")?;
            writeln!(f, "Part 1: {}", part1)?;
        }
        if let Some(part2) = &self.part2 {
            writeln!(f, "Part 2: {}", part2)?;
        }
        Ok(())
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
