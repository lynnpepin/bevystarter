# BevyStarter -- My personal Bevy Starter template, which is completely not ready for anyone to use!

> **TLDR:**
>
> ```sh
> cargo run
> ```
> 
> This is a template which starts with:
> 
> 1. A perf-optimized `Cargo.toml` and `rust-toolchain.toml`,
> 
> 2. The [`DefaultPlugins`](https://docs.rs/bevy/latest/bevy/struct.DefaultPlugins.html) PluginGroup,
>
> 3. Uses [`gilrs`](https://gitlab.com/gilrs-project/gilrs) for [gamepad input](https://bevy-cheatbook.github.io/input/gamepad.html), 
> 
> 4. And a 640x480 window with a simple setup system. (TODO)


## What is this?

This is a template of a blank Bevy project which [optimizes for performance](https://bevyengine.org/learn/quick-start/getting-started/setup/#compile-with-performance-optimizations), by enabling `opt-level = 1` optimizations for dev code and `opt-level = 3` for dependencies, including recommended release-mode and wasm optimizations.

Get your first, slowest compile out of the way early with

```sh
cargo run
```
and you're golden!

This template does NOT enable [the fast compile optimizations](https://bevyengine.org/learn/quick-start/getting-started/setup/#enable-fast-compiles-optional). In order of most impactful, you might want to:
 

1. **Enable dynamic linking**, bypassing the longest part of the Rust compiler:

```sh
cargo run --features bevy/dynamic_linking
```

2. **Use an alternate linker,** making the linking step faster than Rust's default. [See instructions here.](https://bevyengine.org/learn/quick-start/getting-started/setup/#alternative-linkers)

3. **Switch to Rust's unstable toolchain** by adding this line to `rust-toolchain.toml`. (More on Rust's unstable-only performance improvements in the docs)

```toml
[toolchain]
channel = "nightly"
```
