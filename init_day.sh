# !/usr/bin/env zsh

set -o errexit
set -o nounset
set -o pipefail

day=$1
day_package="day${day}"

impl=$day_package/src/main.rs

input="inputs/${day}.in" 
example="inputs/${day}.example" 

url="https://adventofcode.com/2023/day/${day}"

if [[ ! -d $day_package ]]; then
    cargo new $day_package --bin

    cat << EOF >> $day_package/Cargo.toml
aoc_derive.path = '../aoc_derive'
utils.path = '../utils'
itertools.workspace = true
ndarray.workspace = true
num.workspace = true
parse-display.workspace = true
priority-queue.workspace = true
rayon.workspace = true
regex.workspace = true
EOF

    cat << EOF > $impl
#[allow(unused_imports)]
use parse_display::FromStr;
#[allow(unused_imports)]
use itertools::Itertools;
use utils::*;
use aoc_derive::aoc_main;

#[aoc_main]
fn solve(input: Input) -> Solution {
    solution!()
}
EOF

    rustfmt $impl

    git add $day_package

    session=$(cat .session)
    mkdir -p inputs
    curl "$url/input" -H "Cookie: session=$session" > $input
    touch $example
fi


i3-msg "workspace 2; exec firefox $url"
sleep 0.1
i3-msg "workspace 1"

nvim -c "lua require'aoc'.init($day, '$impl', '$example', '$input')"
