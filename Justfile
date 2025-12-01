# List the available recipes
default:
  @just --list --unsorted

# Run all days
@all:
  cargo build --quiet --release
  for day in `seq 1 $(fd -td "day*" | wc -l)`; do just day $day; done;

# Run a specific day
@day DAY:
  echo "Advent of Code Day {{DAY}}"
  cargo run --quiet --release --bin day-$(printf "%02d" {{DAY}})
alias d := day

# Run all tests
@test:
  cargo nextest run --cargo-quiet
alias t := test

# Create a new day from the template
@new:
  day=$(($(fd -td "day*" | wc -l)+1)); \
  name=day-$(printf "%02d" $day); \
  cargo new --bin $name; \
  cp template.rs $name/src/main.rs; \
  touch $name/src/input.txt; \
  code $name/src/*;
