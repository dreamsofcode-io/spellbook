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

First, make sure you have set your`REDIS_URL` and `DATABASE_URL` defined in the .env.

You can deploy a free instance of both redis and PostgreSQL using the sponsor of this
video: Aiven.

To do so, head to [https://go.aiven.io/dreamsofcode](https://go.aiven.io/dreamsofcode)
and then sign up.

You should be then able to create both a redis and PostgreSQL instance at no cost.

Once configured you can run this server using cargo.

```
cargo run 
```
