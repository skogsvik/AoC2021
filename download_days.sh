#!/bin/bash
set -e

# https://stackoverflow.com/a/30969768/6639411
set -o allexport
source .env  # Try to load COOKIE variable
set +o allexport

DAYS="[1-25]"

mkdir -p days/inputs days/static
curl "https://adventofcode.com/2021/day/$DAYS" -b "$COOKIE" -o days/day_#1.html
sed -i -e 's/\/static\/style.css/static\/style.css/' days/day_*.html

curl "https://adventofcode.com/2021/day/$DAYS/input" -b "$COOKIE" -o days/inputs/aoc#1
curl 'https://adventofcode.com/static/style.css?28' -o days/static/style.css
