#!/usr/bin/env nu

def what_to_test [] {
    let lock_diffstat = (git diff --stat main -- Cargo.lock)
    let diffstat = (git diff --name-status main)
    let puzzles = (
        $diffstat
        | parse --regex '(?P<year>\d{2})/(?P<day>\d{2})'
        | uniq
    )
    if $lock_diffstat =~ deletion or ($puzzles | length) > 1 {
        # If more than one puzzle has changed, or the cargo lockfile
        # has other changes than just additions, test everything.
        "--all"
    } else if ($puzzles | length) == 1 {
        let puzzle = $puzzles.0
        $"-p aoc_($puzzle.year)_($puzzle.day)"
    }
}

what_to_test