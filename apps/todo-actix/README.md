# TODO API Actix Web

A sample project to try out Actix Web building a very simple TODO API.

## Running

```bash
cargo run --release
```

## TODO

- Caching
- Metrics
- Alerts

RUST_BACKTRACE=1 RUST_LOG=actix_web=debug cargo run

If you want panics and errors to both have backtraces, set RUST_BACKTRACE=1;
If you want only errors to have backtraces, set RUST_LIB_BACKTRACE=1;
If you want only panics to have backtraces, set RUST_BACKTRACE=1 and RUST_LIB_BACKTRACE=0