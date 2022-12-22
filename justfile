_default:
    @just --list

set dotenv-load

# scaffold a new puzzle
new year day:
    #!/usr/bin/env nu
    (
        cargo generate aoc
        --git "https://github.com/remlse/cargo-templates"
        --branch main
        --init
        --name whatever
        --define year={{year}}
        --define day={{day}}
    )

    if 'SESSION_ID' in (env).name and $env.SESSION_ID != "REPLACE_ME" {
        (
            fetch
            -H ['Cookie' $'session=($env.SESSION_ID)']
            $'https://adventofcode.com/20{{year}}/day/({{day}} | into int)/input'

        ) | str trim --right | save {{year}}/{{day}}/input/input.txt

    } else {
        echo 'No session ID found. Add it to ".env" to automatically fetch inputs.'
    }

# run puzzle solution - specify input: 'just run sample_2'
[no-cd]
run input="input":
    @cargo run -q -- {{input}}

# run on sample input - specify sample: 'just sample-run 2'
[no-cd]
sample-run nr="":
    #!/usr/bin/env nu
    if "{{nr}}" == "" {
        cargo run -q -- sample
    } else {
        cargo run -q -- sample_{{nr}}
    }

# run tests
[no-cd]
test:
    cargo nextest run --final-status-level slow
