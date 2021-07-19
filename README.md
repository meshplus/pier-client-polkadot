# Pier client plugin for polkadot

```
# start appChainPlugin server implemented in rust

$ cargo build
$ cargo run --bin plugin_server
```

# Test TLS

Rust client and server can talk via TLS.

```
$ cargo run --bin greeter_server -- --tls

$ cargo run --bin greeter_client -- --tls
```
