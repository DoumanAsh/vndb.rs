vndb.rs
==============

[![Rust](https://github.com/DoumanAsh/vndb.rs/actions/workflows/rust.yml/badge.svg)](https://github.com/DoumanAsh/vndb.rs/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/vndb.svg)](https://crates.io/crates/vndb)
[![Documentation](https://docs.rs/vndb/badge.svg)](https://docs.rs/crate/vndb/)
[![dependency status](https://deps.rs/crate/vndb/0.16.0/status.svg)](https://deps.rs/crate/vndb/0.16.0)

Kawaii VNDB API

## Features

* Tokio support
* Provides protocol requests/responses for user to use directly.
* Optional parsing of get responses into static structs.

## TLS client

Due to bad default choice of underlying crypto library, `rustls` is included with `default-features = false` which essentially makes it unusable until user provides feature.
