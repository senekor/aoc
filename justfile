_default:
    @just --list

# scaffold a new puzzle
new year day:
    #!/bin/bash
    set -euo pipefail

    cargo generate aoc \
        --git "https://github.com/senekor/cargo_templates" \
        --branch main \
        --init \
        --name whatever \
        --define year={{year}} \
        --define day={{day}}

    ./dev/scripts/readme_table.nu

    if [ -f dev/session_id ] ; then
        curl --header "Cookie: session=$(cat dev/session_id)" \
            "https://adventofcode.com/20{{year}}/day/$(echo {{day}} | sd '^0' '')/input" \
            > {{year}}/{{day}}/input/input.txt
    else
        echo "No session ID found. Add it to a file 'dev/session_id' to automatically fetch inputs."
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

set positional-arguments

# run tests
[no-cd]
test *args='':
    @cargo nextest --config-file "$(git rev-parse --show-toplevel)/dev/nextest.toml" run --final-status-level slow $@
