# Advent Of Code 2020

generate new day (day 6): `./nd.sh 6`
generate wasm: `./wasm.sh`
generate wasm relese: `./wasm.sh r`

## Structure

After you create a new day, `cargo run` will auto run new day binary.
The only file you need to `touch` is data$1 (in data dir), and both day$1.rs where $1 is current day number. To data file you paste AOC input data and other two file you programme.