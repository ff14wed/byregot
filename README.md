# byregot
[![Build][build-badge]][build-url]
[![MIT License][license-badge]][license-url]
[![Crates.io][crates-badge]][crates-url]

[build-badge]: https://github.com/ff14wed/byregot/workflows/Build/badge.svg?branch=master
[build-url]: https://github.com/ff14wed/byregot/actions/workflows/build.yml
[license-badge]: https://img.shields.io/crates/l/byregot
[license-url]: https://github.com/ff14wed/byregot/blob/master/LICENSE
[crates-badge]: https://img.shields.io/crates/v/byregot
[crates-url]: https://crates.io/crates/byregot

Byregot is yet another simulation library for crafting in FINAL FANTASY XIV,
heavily inspired by the [Teamcraft simulator](https://github.com/ffxiv-teamcraft/simulator).

This library is written in Rust and is designed for very high performance
applications that require simulations of FFXIV crafting. For this reason,
this library purposefully only implements a **subset** of crafting. If you
need the full breadth of functionality for simulating crafts, I highly suggest
you just use Teamcraft simulator instead.

## Features
:heavy_check_mark: Accurately simulates crafts with non-specialist actions.

:heavy_check_mark: Supports simulation for all job levels up to 100, but actions
will not individually validate if your job level meets the minimum requirement.

:heavy_check_mark: Support for expert crafting

:heavy_check_mark: Simulates all random events in crafting, such as failed "Hasty
Touches" and "Good" conditions.

:heavy_check_mark: Ability to manually set the next success and/or condition for
the next step.

:x: Byregot does not implement Trained Eye.

:x: Byregot does not currently implement the specialist actions Careful
Observation and Heart and Soul. This means Muscle Memory and Reflect can
only be used on Normal conditions.

:x: Byregot does not currently implement the specialist action Quick
Innovation.

## Development
Byregot is written in Rust, and so needs
[Rust](https://www.rust-lang.org/tools/install) installed for development.

To run the tests, run `cargo test`.

To run the benchmarks, run `cargo bench`.

## Usage
For example usage of the library, check out the [benchmark
code](benches/craft_sim_benchmark.rs).

## Benchmarks
Abbreviated single-threaded benchmark results on a AMD Ryzen 5950x:

```
full_craft_no_validate  time:   [260.27 ns 261.41 ns 262.63 ns]
full_craft              time:   [301.78 ns 303.11 ns 304.61 ns]
get_valid_actions1      time:   [13.971 ns 14.042 ns 14.118 ns]
get_valid_actions2      time:   [14.026 ns 14.103 ns 14.189 ns]
set_next_state_rng      time:   [53.456 ns 54.026 ns 54.666 ns]
```

This benchmark indicates roughly 3.3 million crafts per second for a 17-step
craft, or roughly 56 million steps per second.

## License

MIT

Special thanks to the folks at Teamcraft for the research and development
of Teamcraft simulator.
