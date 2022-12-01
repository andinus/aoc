#!/usr/bin/env raku

# @calories records calories carried by each Elf, sorted.
my Int @calories = "input".IO.split("\n\n").map(*.lines.sum).sort;

put "Part 1: " ~ @calories.tail;
put "Part 2: " ~ @calories.tail(3).sum;
