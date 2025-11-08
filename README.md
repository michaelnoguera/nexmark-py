## PyO3 bindings for nexmark-rs

See the nexmark-rs repo for the benchmark implementation: https://github.com/risingwavelabs/nexmark-rs

Python type hints are hardcoded in nexmark.pyi, at some point in the future it should become
possible to automatically generate them from the Rust code using pyo3-introspection.

### Building
You will need rust and the uv package manager for python

1. Clone the repo
2. `uv sync` to set up the virtual env, and `source .venv/bin/activate` to activate it
3. `cargo check` make sure nothing is very wrong before we use maturin to build with uv
4. `maturin develop --uv` to build and install the package in the virtual env
5. check that `uv run pytest` passes and `uv run generate.py` works

### generate.py
You can use generate.py to run the generator and output one json object per line. You should probably just
use nexmark-rs directly if this is all you want; the implementation here is mostly for testing the PyO3 bindings.

If you try to compare the output for correctness, note that the date_time values will differ between runs because they appear to depend on clock time.

### Rate of generation
By default the generator sleeps some small amount between events. I noticed that 
nexmark-rs generated about 10k events per second on my laptop, so I tuned the 
delay in generate.py to match that.

You can disable the delay with the --no-wait flag, which will generate events
as fast as possible. For generate.py I hit 75k/s (after 10s, piping stdout to /dev/null) and nexmark-rs produces and prints events at 750k/s (same setup).

In the rust library there is a paramter to control the number of generators running in parallel, and I have neither explored that nor exposed it (or any other customizable parameters) here.

This is almost certainly CPU bound and limited by the Python GIL, so if you need performance you should use the Rust library directly.

In a for loop just throwing the events away (benchmark.py):
```
Generated 1400060 events in 10.00 seconds (140005.82 events/sec)
Generated 1403470 events with 4 generator threads in 10 seconds (140347.00 events/sec)
Generated 5388012 events with 4 generator processes in 10 seconds (538801.20 events/sec)
```