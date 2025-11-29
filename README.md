# My (opinionated) collection of AoC utilities in Rust

This is my utility library for solving Advent of Code in Rust. It grew over the years and every year I've carried it
over to the next year's repository. Now I finally took the time to move it to its own repository.

## How to use this repository

### Project layout

If you just want to use the utilities (such as parsing, graph stuff, etc.), add this repository as a submodule and add
the `utils` folder to your cargo workspace.

However, if you want to automatically set up each day via the `init_day.sh` script, your project layout needs to look
like this:

```
|
- aoc_lib # This repository, as a git submodule
- init_day.sh # Symlink to aoc_lib/init_day.sh
- aoc.lua # Symlink to aoc_lib/aoc.lua (if you use neovim)
- .session # Contains your session cookie, see https://github.com/wimglenn/advent-of-code-wim/issues/1
- Cargo.toml
```

Your `Cargo.toml` should initially look something like this:

```toml
[workspace]
resolver = "2"
members = [
    "aoc_lib/aoc_derive",
    "aoc_lib/utils",
]

[workspace.dependencies]
derive_more = {version = "2.0.1", features = ["full"]}
itertools = "0.13.0"
ndarray = "0.16.1"
parse-display = "0.10.0"
rayon = "1.10"
regex = {version = "1.11", features = ["pattern"]}
lazy-regex = "3.3.0"
pretty_assertions = "1.4.0"
```

Now, on each day you can run `./init_day.sh <n>` which will create a `day<n>` crate, add it to the workspace, and
initialize `day<n>/src/main.rs` with a basic solution skeleton.

At the end there's also code that will open firefox with the current day's problem, and a neovim workspace with the
solution file opened and some nice aoc-specific shortcuts. You will probably want to adjust this in your fork of this
repository.

### Solving a day

Initially, your solution file will look something like this:

```rust
use aoc_derive::aoc_main;
use utils::ParseInput;
use utils::*;
use lazy_regex::regex;

// The aoc_main marco takes care of reading the input from the downloaded file,
// and displaying your solution.
// To run, use `cargo run --package day<n> -- inputs/<n>.in` to run with your real input,
// or `cargo run --package day<n> -- inputs/<file>` to run on an example file instead.
#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    // Input will contain the puzzle input. It has some utility functions to quickly parse it,
    // e.g. splitting lines or turning it into a grid of some given type.

    // The return value is the solution of that day.
    // String, &str, and all integers implement Into<Solution>. 
    // So for part1, just return the solution as an instance of this type.
    // When solving part2, you need to return a tuple of (part1, part2).
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        use utils::assert_example;
        // This macro takes either 2 or 3 parameters.
        // The 2-parameter version will pass the given string to the solve() function and compare its part1 solution
        // against the 2nd macro argument, ignoring part2.
        // The 3-parameter version will additionally compare the part2 solution against the 3rd macro argument.
        assert_example!(
            r#"
                "#,
            ""
        );

        // To only test part2 but ignore part1, there's also a `assert_part2!` macro.
    }
}
```
