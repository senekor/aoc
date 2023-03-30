_default:
    @just --list

set dotenv-load

# scaffold a new puzzle
new year day:
    #!/bin/bash
    cargo generate aoc \
        --git "https://github.com/remlse/cargo-templates" \
        --branch main \
        --init \
        --name whatever \
        --define year={{year}} \
        --define day={{day}}

    if [ "$SESSION_ID" != "REPLACE_ME" ] ; then
        curl --header "Cookie: session=$SESSION_ID" \
            "https://adventofcode.com/20{{year}}/day/$(printf %i {{day}})/input" \
            > {{year}}/{{day}}/input/input.txt
    else
        echo "No session ID found. Add it to '.env' to automatically fetch inputs."
    fi

# run puzzle solution - specify input: 'just run sample_2'
[no-cd]
run input="input":
    @cargo run -q -- {{input}}

# run on sample input - specify sample: 'just sample-run 2'
[no-cd]
sample-run nr="":
    #!/bin/bash
    if [ "{{nr}}" == "" ] ; then
        cargo run -q -- sample
    else
        cargo run -q -- sample_{{nr}}
    fi

# run tests
[no-cd]
test:
    cargo nextest run --final-status-level slow
