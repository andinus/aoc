#!/usr/bin/env raku

my Int @inputs = "input".IO.lines>>.Int;

{
    my Int $larger-than-previous = 0;

    for (^@inputs.elems).skip -> $idx {
        $larger-than-previous++ if @inputs[$idx] > @inputs[$idx - 1];
    }

    put "Part 1: ", $larger-than-previous;
}

{
    my Int $larger-than-previous = 0;

    for (^@inputs.elems).skip(3) -> $idx {
        $larger-than-previous++ if @inputs[$idx - 2 .. $idx].sum > @inputs[$idx - 3 .. $idx - 1].sum;
    }

    put "Part 2: ", $larger-than-previous;
}
