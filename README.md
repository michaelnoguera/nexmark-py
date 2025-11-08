PyO3 bindings for nexmark-rs

### Building
1. Clone the repo
2. `uv sync` to set up the virtual env, and `source .venv/bin/activate` to activate it
3. `cargo check` make sure nothing is very wrong before we use maturin to build with uv
4. `maturin develop --uv` to build and install the package in the virtual env
5. check that `uv run pytest` passes and `uv run main.py` works