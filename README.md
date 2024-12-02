# Advent of Code

## Installation

```shell
git clone https://github.com/yoonthegoon/advent-of-code.git
cd advent-of-code
```

## Usage

### Get your puzzle inputs

To download all available puzzle inputs, you'll need to be logged in.

Copy your login cookie from your request header and run `echo COOKIE="session=07eb32...d4641b" > .env`. \
(obviously replace with your cookie)

Now run `cargo run` and all available puzzle inputs will be downloaded to `inputs/<year>/<day>.txt`. \
**DO NOT** run this repeatedly; this makes 225 requests in quick succession and I do not want unnecessary traffic
bombarding Eric.

### Run tests

Each test will run with the example puzzle input then with your personal puzzle input. \
The example input is used for the test results, and the personal input is written to stdout.

Use any of the following commands:

```shell
cargo test --package advent-of-code -- --format=pretty unstable-options --show-output  # all tests
cargo test --package advent-of-code --lib year2024 -- --format=pretty unstable-options --show-output  # year
cargo test --package advent-of-code --lib year2024::day01 -- --format=pretty unstable-options --show-output  # day
cargo test --package advent-of-code --lib year2024::day01::tests::test_part1 -- --format=pretty unstable-options --show-output  # part
```

Here's a sample of my test output

```text
successes:

---- year2024::day01::tests::test_part1 stdout ----
1197984

---- year2024::day01::tests::test_part2 stdout ----
23387399
```
