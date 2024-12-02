_default:
    @just --list --unsorted

# scaffold a new puzzle
new year day:
    #!/bin/bash
    set -euo pipefail

    cargo generate \
        --path devel/cargo-generate-template \
        --init \
        --name whatever \
        --define year={{year}} \
        --define day={{day}}

    ./devel/scripts/readme_table.nu

    if [ -f devel/session_id ] ; then
        curl --header "Cookie: session=$(cat devel/session_id)" \
            "https://adventofcode.com/20{{year}}/day/$(echo {{day}} | sd '^0' '')/input" \
            > {{year}}/{{day}}/input/input.txt
    else
        echo "No session ID found. Add it to a file 'devel/session_id' to automatically fetch inputs."
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
    @cargo nextest --config-file "$(git rev-parse --show-toplevel)/devel/nextest.toml" run --final-status-level slow $@
