#!/bin/bash

next_day=$(ls -d day-* 2>/dev/null | grep -o '[0-9]\+' | sort -n | tail -1)
if [ -z "$next_day" ]; then
    next_day=0
fi
next_day=$((next_day + 1))

formatted_day=$(printf "%02d" $next_day)
new_folder="day-$formatted_day" 

mkdir $new_folder
cargo init $new_folder
touch $new_folder/input.txt
touch $new_folder/test.txt

cp aoc.template.rs $new_folder/src/main.rs
