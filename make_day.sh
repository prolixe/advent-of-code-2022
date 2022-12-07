#!/bin/bash

set -eu

day=$1

touch src/day${day}.rs
touch resources/day${day}.txt
touch resources/day${day}_example.txt

cat <<EOF > src/day${day}.rs
pub fn day$day() {
    let contents = include_str!("../resources/day${day}.txt");

    println!("Day ${day}, part 1:");
    println!("Day ${day}, part 2:");
}
EOF
