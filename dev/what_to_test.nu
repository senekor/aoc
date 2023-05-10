#!/usr/bin/env nu

def what_to_test [] {
    let diffstat = (git diff --name-status main)
    let puzzles = ($diffstat | parse --regex '(?P<year>\d{2})/(?P<day>\d{2})')
    if $diffstat =~ "\tCargo." or ($puzzles | length) > 1 {
        # If more than one puzzle has changed, or the workspace
        # files have changed, test everything.
        "--all"
    } else if ($puzzles | length) == 1 {
        let puzzle = $puzzles.0
        $"-p aoc_($puzzle.year)_($puzzle.day)"
    }
}

what_to_test
