# P2P

Simple P2P whispering application

# Usage

1. Compile the project:

```rust
cargo build
```

2. Start cli applications:

```rust
RUST_LOG=info  ./p2p --period 6 --port 4000
```

```rust
RUST_LOG=info  ./p2p --period 6 --port 5000 --connect '127.0.0.1:4000'
```

Optionally you can start additional nodes.

Available parameters:

- period - every node sends `hello` message to available peers every `period` seconds. Required parameter.
- port - port used for http server. Required parameter.
- connect - bootnode address. Optional parameter

# Description

Every node starts http server with the following endpoints:

- GET `/ping` - it's used to check if a node is available. it returns `pong`
- POST `/connect` - new node calls this endpoint to broadcast its address to available peers
- POST `/add_peer` - adds a new peer
- GET `/fetch_peers` - returns available peers
- POST `/whisper` - prints a message


Notes:

- If `connect` parameter is passed, a node tries to connect to the specified node.
- Every `period` seconds, a node sends `hello` message to available peers.
- If a node fails to respond to any requests, it will be removed from peers.
- Every 10 seconds a node tries to fetch sync peers from available nodes.
