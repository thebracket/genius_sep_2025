# Custom Build Profiles

You can customize your build profiles in `Cargo.toml`. This allows you to create different optimization levels for different scenarios.

For example, sometimes a `debug` build is too slow - but you'd like to have *some* optimizations enabled---but still be able to use a debugger. You can create a custom profile for this:

```toml
[profile.dev]
opt-level = 2
debug = true
```

Or you might want a `production` profile that takes a really long time to compile, but produces the fastest possible binary:

```toml
[profile.production]
opt-level = "3" # Optimize for fastest
lto = true      # Enable Link Time Optimization
codegen-units = 1 # Better optimizations with a single codegen unit
debug = false  # No debug symbols
strip = true  # Strip symbols to reduce size
```

You can then build with this profile using:

```sh
cargo build --profile production
```