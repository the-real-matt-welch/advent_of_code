#!/usr/bin/env fish

source .env
set -l day $argv[1]

curl -H "Cookie: session=$AOC_COOKIE" \
    https://adventofcode.com/2024/day/$day/input \
    > inputs/2024/day(string pad -w2 -c0 $day).txt
