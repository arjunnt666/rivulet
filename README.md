# rivulet

local-first sync pieces that compile.

version vectors, last-write-wins maps, in-memory storage traits. the hard merge edge cases are still written down more than proven.

not a collab product. a CRDT bag you can test without a relay.

## works today

- version vector increment / merge
- LWW map last write wins
- `rivulet version`

## does not work yet

- production offline queue
- multi-peer relay you would ship

## try it

```bash
cargo test --workspace
cargo build -p rivulet-cli
./target/debug/rivulet version
```

## license

apache-2.0
