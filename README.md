# aoc
2023 Advent of Code in Rust

## Check

`ls day* -d | xargs -I {} bash -c "cd '{}' && cargo clippy -- -W clippy::pedantic"`

## Run:

 `ls day* -d | xargs -I {} bash -c "cd '{}' && cargo run --release"`

## Tests:

`ls day* -d | xargs -I {} bash -c "cd '{}' && cargo test"`
