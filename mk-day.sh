#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "usage: $0 <day-number>"
  exit 1
fi

day_num="$1"
# zero-pad to two digits
day_padded=$(printf "%02d" "$day_num")
proj="day${day_padded}"

echo "Creating Rust project: $proj"
cargo new "$proj" --vcs none

# Create empty input files
touch "$proj/input.txt" "$proj/input-test.txt"

# Overwrite main.rs with the provided template
cat > "$proj/src/main.rs" <<'RS'
fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let contents = std::fs::read_to_string(&filename).unwrap();

    println!("{}", contents);
}
RS

echo "Done. Project created at ./$proj"
