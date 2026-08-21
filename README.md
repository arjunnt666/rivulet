# rivulet

local-first sync pieces that compile.

version vectors, ops, two peers that actually converge after push/pull. the hard merge edge cases are still written down more than proven.

not a collab product. a CRDT bag you can test without a relay.

## works today

- version vector increment / merge / observe remote dots
- document local_op vs apply (apply does not steal the local actor counter)
- two peers exchange missing ops and end with the same set
- `rivulet demo`

## does not work yet

- production offline queue
- multi-peer relay you would ship

## try it

```bash
cargo test --workspace
cargo build -p rivulet-cli
./target/debug/rivulet demo
```

## license

apache-2.0
