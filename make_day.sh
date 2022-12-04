#!/bin/bash

set -eu

day=$1

touch src/day${day}.rs
touch resources/day${day}.txt
touch resources/day${day}_example.txt
