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
    
    if 'SESSION_ID' in (env).name {
        (
            fetch
            -H ['Cookie' $'session=($env.SESSION_ID)']
            $'https://adventofcode.com/20{{year}}/day/({{day}} | into int)/input'

        ) | str trim --right | save {{year}}/{{day}}/input/input.txt

    } else {
        echo 'No session ID found. Add it to ".env" to automatically fetch inputs.'
    }

# run puzzle solution
[no-cd]
run:
    cargo run

# run puzzle solution on sample input
[no-cd]
sample-run:
    cargo run --example sample

# run tests
[no-cd]
test:
    cargo nextest run --final-status-level slow
