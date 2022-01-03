# byregot
Byregot is yet another simulation library for crafting in FINAL FANTASY XIV,
heavily inspired by the [Teamcraft simulator](https://github.com/ffxiv-teamcraft/simulator).

This library is written in Rust and is designed for very high performance
applications that require simulations of FFXIV crafting. For this reason,
this library purposefully only implements a **subset** of crafting. If you
need the full breadth of functionality for simulating crafts, I highly suggest
you just use Teamcraft simulator instead.

## Features
:heavy_check_mark: Accurately simulates crafts with non-specialist actions at
level 90

:heavy_check_mark: Support for expert crafting

:heavy_check_mark: Simulates all random events in crafting, such as failed "Hasty
Touches" and "Good" conditions.

:heavy_check_mark: Ability to manually set the next success and/or condition for
the next step.

:x: Byregot does not factor in job traits when using actions. All actions are
assumed to be the versions seen at level 90.  For instance, a craft configured
with job level 80 will see accurate values **except** when using "Careful
Synthesis" or "Groundwork".

:x: Byregot does not implement Trained Eye.

:x: Byregot does not currently implement the specialist actions Careful
Observation and Heart and Soul.

## Development
Byregot is written in Rust, and so needs
[Rust](https://www.rust-lang.org/tools/install) installed for development.

To run the tests, run `cargo test`.

To run the benchmarks, run `cargo bench`.

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
