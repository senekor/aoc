#!/usr/bin/env nu

def day_completed [year: int, day: int] {
    ($day | into string | fill -a r -c "0" -w 2) in (ls -s $"($year)" | get name)
}

def get_filler [year: int, day: int] {
    if (day_completed $year $day) {
        "▓▓"
    } else { "?" }
}

def get_years [] {
    ls | where name =~ "[0-9]{2}" | get name
}

def generate_table [] {
    let body = (get_years | each { |year|
        let main_rows = (1..25 | each { |day|
            let filler = (get_filler $year $day)
            let buffer = if ($filler | str length | $in > 1) { "" } else { " " }
            let href = $'href="https://adventofcode.com/20($year)/day/($day)"'
            $"> ($buffer)<a ($href | fill -a l -w 43)>($filler)</a"
        } | str join "\n\n")
        $"20($year) │<span></span\n($main_rows)\n> │"
    } | str join "\n")

    let stars = (get_years | each { |year|
        1..25 | where { |day| day_completed $year $day }
    } | flatten | length | $in * 2)

    $"<pre>
        1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25
     ┌────────────────────────────────────────────────────────────────────────────┐
($body)  ($stars) ⭐
     └────────────────────────────────────────────────────────────────────────────┘
</pre>"
}

def update_readme [] {
    let marker = "\n<!-- generate_readme_table_marker -->\n"
    let old_readme_parts = (open --raw README.md | split row $marker)
    [
        $old_readme_parts.0,
        (generate_table),
        $old_readme_parts.2,
    ] | str join $marker | save -f README.md
}

update_readme
