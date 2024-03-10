# Spellbook: A Simple Spell API

This project is part of the [Dreams of Code](https://youtube.com/@dreamsofcode) video
on caching patterns for handling TTL.

## Requirements

This server is built with Rust, and so requires Rust v1.76. You can install this 
using rustup.

Additionally, the server uses both Redis and PostgreSQL backends for storing data.

You can deploy an instance yourself locally or use a free instance provided by
Aiven.

# Running

First, make sure you have set your`REDIS_URL` and `POSTGRES_URL` in the .env.

Then, to run this server, just use cargo.

```
cargo run 
```
