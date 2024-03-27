#!/usr/bin/env nu

def get_years [] {
    ls | where name =~ "[0-9]{2}" | get name
}

def day_completed [year: string, day: int] {
    ($day | into string | fill -a r -c "0" -w 2) in (ls -s $year | get name)
}

# let difficulty = (http get https://raw.githubusercontent.com/senekor/aoc/main/dev/difficulty.toml)
let difficulty = (open ./dev/difficulty.toml)

def check_difficulty [] {
    exit (get_years | any { |year|
        (1..25 | each { |day|
            let missing = (day_completed $year $day) and $day > ($difficulty | get $year | length)
            if $missing {
                let day_str = ($day | into string | fill -a r -c "0" -w 2)
                print $"The difficulty spec for ($year)/($day_str) is missing!"
            }
            $missing
        } | any { || $in })
    } | into int)
}

check_difficulty
