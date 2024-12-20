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

### First build

Get your first, slowest compile out of the way early with

```sh
cargo run
```
and you're golden!

### WASM export

We use `wasm-bindgen`. (More details [here](https://bevy-cheatbook.github.io/platforms/wasm/webpage.html).) Install it with 

```sh
cargo install wasm-bindgen-cli
```

and then build with

```sh
cargo build --release --target wasm32-unknown-unknown

# change `bevy_starter` below as appropriate
# you may need to `cargo update` if the below fails because of a schema error
wasm-bindgen --no-typescript --target web \
    --out-dir ./out/ \
    --out-name "bevy_starter" \
    ./target/wasm32-unknown-unknown/release/bevy_starter.wasm
```

### Optimizing compile times 

Follow the  [the fast compile optimizations](https://bevyengine.org/learn/quick-start/getting-started/setup/#enable-fast-compiles-optional)
 

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
