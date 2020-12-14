#!/usr/bin/env raku

sub MAIN (
    Int $part where * == 1|2 = 1 #= part to run (1 or 2)
) {
    my @input = "input".IO.lines;
    my Int $x-max = @input[0].chars - 1;
    my Int $row-length = $x-max + 1;

    my @seats = @input.join.comb;
    my $max-seats = @seats.end;

    my @directions = (+$row-length, -$row-length); # down, up

    # @non-left-corner contains directions that left corner can't
    # follow. It should only be followed by non-left corner seats.
    my @non-left-corner = (
        -$row-length - 1, # up-left
        -1, # left
        +$row-length - 1, # down-left
    );

    # @non-right-corner contains directions that right corner can't
    # follow. It should only be followed by non-right corner seats.
    my @non-right-corner = (
        -$row-length + 1, # up-right
        +1, # right
        +$row-length + 1, # down-right
    );

    my Int $round = 0;
    loop {
        my @changed;
        my Bool $change = False;

        INNER: for @seats.kv -> $idx, $seat {
            @changed[$idx] = $seat;
            given $seat {
                when '.' { next INNER; }
                when 'L' {
                    if adjacent-occupied($idx, 1) == 0 {
                        @changed[$idx] = '#';
                        $change = True;
                    }
                }
                when '#' {
                    if adjacent-occupied($idx, 1) >= 4 {
                        @changed[$idx] = 'L';
                        $change = True;
                    }
                }
            }
        }
        $round++;
        print "Round $round.\r";

        last unless $change;

        for @changed.kv -> $idx, $changed_seat {
            @seats[$idx] = $changed_seat;
        }
    }

    my Int $occupied = @seats.comb('#').elems;
    say "Part $part: ", $occupied;

    # adjacent-occupied returns the number of adjacent cells that have
    # been occupied by others. $visibility_range should be 1 if only
    # directly adjacent seats are to be counted. Make it -1 for
    # infinite visibility. It ignores floors ('.').
    sub adjacent-occupied (
        Int $idx, Int $visibility_range --> Int
    ) {
        my Int $occupied = 0;

        for @directions -> $direction {
            with @seats[$idx + $direction] {
                $occupied++ if $_ eq '#';
            }
        }

        # Elements in right corner can't follow @non-right-corner.
        unless ($idx + 1) % 10 == 0 {
            for @non-right-corner -> $direction {
                with @seats[$idx + $direction] {
                    $occupied++ if $_ eq '#';
                }
            }
        }

        # Elements in left corner can't follow @non-left-corner.
        unless $idx % 10 == 0 {
            for @non-left-corner -> $direction {
                with @seats[$idx + $direction] {
                    $occupied++ if $_ eq '#';
                }
            }
        }

        return $occupied;
    }
}
