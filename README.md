# Advent of Code 2019

![HitCount](http://hits.dwyl.io/mrrobb/advent-of-code-2019.svg)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/MrRobb/advent-of-code-2019/blob/master/LICENSE)

|       | Problem                                          | Solution                                                                              | Time execution   | Finished |
|-------|--------------------------------------------------|---------------------------------------------------------------------------------------|------------------|----------|
| Day 1 | [Problem 1](https://adventofcode.com/2019/day/1) | [day1.rs](https://github.com/MrRobb/advent-of-code-2019/blob/master/src/day1.rs?ts=4) | 2.7 ms ± 0.5 ms  | ✓        |
| Day 2 | [Problem 2](https://adventofcode.com/2019/day/2) | [day2.rs](https://github.com/MrRobb/advent-of-code-2019/blob/master/src/day2.rs?ts=4) | 5.2 ms ± 0.5 ms  | ✓        |
| Day 3 | [Problem 3](https://adventofcode.com/2019/day/3) | [day3.rs](https://github.com/MrRobb/advent-of-code-2019/blob/master/src/day3.rs?ts=4) | 4.5 ms ± 0.6 ms  | ✓        |
| Day 4 | [Problem 4](https://adventofcode.com/2019/day/4) | [day4.rs](https://github.com/MrRobb/advent-of-code-2019/blob/master/src/day4.rs?ts=4) | 2.7 ms ± 0.4 ms  | ✓        |
| Day 5 | [Problem 5](https://adventofcode.com/2019/day/5) | [day5.rs](https://github.com/MrRobb/advent-of-code-2019/blob/master/src/day5.rs?ts=4) | 2.9 ms ± 0.6 ms  | ✓        |
| Day 6 | [Problem 6](https://adventofcode.com/2019/day/6) | [day6.rs](https://github.com/MrRobb/advent-of-code-2019/blob/master/src/day6.rs?ts=4) | 5.8 ms ± 0.4 ms  | ✓        |
| Day 7 | [Problem 7](https://adventofcode.com/2019/day/7) | [day7.rs](https://github.com/MrRobb/advent-of-code-2019/blob/master/src/day7.rs?ts=4) | 13.2 ms ± 0.6 ms | ✓        |

## Install Rust

If you don't have Rust installed ([how dare you](https://media.giphy.com/media/U1aN4HTfJ2SmgB2BBK/giphy.gif)) just run this:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

> If you are not using a Unix-like OS, check the instructions [here](https://www.rust-lang.org/tools/install)

## Usage

### Clone

```sh
git clone https://github.com/MrRobb/advent-of-code-2019.git
cd advent-of-code-2019
```

### Build

```sh
cargo build
```

### Run

#### Run all

```sh
cargo run
```

#### Run a specific day

```sh
cargo run --bin day1
```

#### Benchmarks

```txt
Benchmark #1: target/release/day1
  Time (mean ± σ):       2.7 ms ±   0.5 ms    [User: 0.9 ms, System: 0.7 ms]
  Range (min … max):     2.2 ms …   6.6 ms    450 runs

Benchmark #2: target/release/day2
  Time (mean ± σ):       5.2 ms ±   0.5 ms    [User: 3.2 ms, System: 0.8 ms]
  Range (min … max):     4.4 ms …   7.0 ms    322 runs

Benchmark #3: target/release/day3
  Time (mean ± σ):       4.5 ms ±   0.6 ms    [User: 2.5 ms, System: 0.8 ms]
  Range (min … max):     3.9 ms …   7.5 ms    305 runs

Benchmark #4: target/release/day4
  Time (mean ± σ):       2.7 ms ±   0.4 ms    [User: 0.8 ms, System: 0.7 ms]
  Range (min … max):     2.2 ms …   4.7 ms    447 runs

Benchmark #5: target/release/day5
  Time (mean ± σ):       2.9 ms ±   0.6 ms    [User: 1.0 ms, System: 0.8 ms]
  Range (min … max):     2.3 ms …   6.3 ms    412 runs

Benchmark #6: target/release/day6
  Time (mean ± σ):       5.8 ms ±   0.4 ms    [User: 3.7 ms, System: 0.9 ms]
  Range (min … max):     5.3 ms …   7.3 ms    290 runs

Benchmark #7: target/release/day7
  Time (mean ± σ):      13.2 ms ±   0.6 ms    [User: 10.9 ms, System: 1.0 ms]
  Range (min … max):    12.3 ms …  15.0 ms    180 runs
```
