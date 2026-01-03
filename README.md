## Dev

```
cargo clippy --all-features --tests -- -D clippy::all
cargo +nightly clippy --all-features --tests -- -D clippy::all

cargo fmt -- --check

cargo test-all-features -- --nocapture

cargo test -p serde-field-default --features alloc,chrono,chrono-tz -- --nocapture

cargo test -p serde-attributes --features _integration_tests --test integration_tests -- --nocapture
```

## Publish order

serde-rename-rule

serde-attributes

serde-enum-str

serde-datetime

serde-aux-ext

serde-field-default

serde-field-with
