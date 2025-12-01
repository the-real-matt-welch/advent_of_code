#!/usr/bin/env fish

source .env
set -l day $argv[1]

curl -H "Cookie: session=$AOC_COOKIE" \
    https://adventofcode.com/2025/day/$day/input \
    > inputs/2025/day(string pad -w2 -c0 $day).txt
