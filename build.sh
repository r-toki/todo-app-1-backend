#!/usr/bin/env bash
cargo build --release
cargo install sqlx-cli --no-defeault-features --features rustls,postgres
sqlx database create
sqlx migrate run
