# SpaceShooter

## Dev

### Linting

```bash
cargo +nightly fmt

```

### Setup Bevy

-   Enable Bevy's Dynamic Linking Feature: This is the most impactful compilation time decrease! If bevy
    is a dependency you can compile the binary with the "dynamic" feature flag (enables dynamic
    linking). Note that right now, this doesn't work on Windows. [Source](https://bevyengine.org/learn/book/getting-started/setup/)

    ```bash
    cargo run --features bevy/dynamic
    ```

-   LLD linker: The Rust compiler spends a lot of time in the "link" step. LLD is much faster at linking than the default Rust linker. To install LLD, find your OS below and run the given command:

    ```bash
    # Ubuntu:
        sudo apt-get install lld
    # Arch:
        sudo pacman -S lld
    # Windows: Ensure you have the latest cargo-binutils
        cargo install -f cargo-binutils
        rustup component add llvm-tools-preview
    # MacOS: Modern LLD does not yet support MacOS, but we can use zld instead:
        brew install michaeleisel/zld/zld
    ```

### Bevy Dependencies

-   [Linux](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md)
-   [More information](https://bevyengine.org/learn/book/getting-started/setup/)

#### Fedora Setup

[Source: github](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md)

```bash
sudo dnf install gcc-c++ libX11-devel alsa-lib-devel systemd-devel
```

## Credits

https://github.com/BorisBoutillier/Kataster

<!--.cargo/config-->

```
# Add the contents of this file to `config.toml` to enable "fast build" configuration. Please read the notes below.

# NOTE: For maximum performance, build using a nightly compiler
# If you are using rust stable, remove the "-Zshare-generics=y" below.

[target.x86_64-unknown-linux-gnu]
linker = "/usr/bin/clang"
rustflags = ["-Clink-arg=-fuse-ld=lld"] #, "-Zshare-generics=y"]

# NOTE: you must manually install https://github.com/michaeleisel/zld on mac. you can easily do this with the "brew" package manager:
# `brew install michaeleisel/zld/zld`
[target.x86_64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/zld" ] #, "-Zshare-generics=y"]

[target.aarch64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/zld" ] #, "-Zshare-generics=y"]

[target.x86_64-pc-windows-msvc]
linker = "rust-lld.exe"
rustflags = [ ] #"-Zshare-generics=n"]

[target.wasm32-unknown-unknown]
runner = "wasm-server-runner"
# Optional: Uncommenting the following improves compile times, but reduces the amount of debug info to 'line number tables only'
# In most cases the gains are negligible, but if you are on macos and have slow compile times you should see significant gains.
#[profile.dev]
#debug = 1
```
