alias r := run-dynamic
alias c := clippy
alias f := fmt

# List all just commands
default:
  just --list

# Run binary in debug with bevy/dynamic feature flag
run-dynamic:
  cargo run --features bevy/dynamic

# Clippy lint check
clippy:
  cargo clippy

# Format with rustfmt
fmt:
  cargo fmt
