_default:
    @just --list

# scaffold a new puzzle
new year day:
    cargo generate aoc \
        --git "https://github.com/remlse/cargo-templates" \
        --branch main \
        --init \
        --name whatever \
        --define year={{year}} \
        --define day={{day}}

# run tests
test:
    cd {{ invocation_directory() }} && \
    cargo nextest run  --final-status-level slow
