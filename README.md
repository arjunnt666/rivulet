# rivulet

local-first sync engine. crdts, presence, offline edits, the whole deal.

your app keeps working when the wifi doesn't. when the network comes back, things merge instead of asking the user to pick a winner like it's 2011.

---

## what this is

rivulet is a stack for collaborative data that lives on the client first:

- **crdts** — maps, text (rga), lists, counters, a tiny graph
- **sync protocol** — version vectors, missing-op exchange, live broadcast
- **storage** — memory / sqlite / filesystem backends
- **presence** — cursors & "is typing" that vanish when the tab dies (as they should)
- **optional relay** — catch-up cache for peers that were offline, not a source of truth
- **js + react/solid/vue bindings** — because most uis are not written in rust
- **cli** — `rivulet version` will at least say something nice

rust core, ts surface, monorepo that looks intentional.

---

## quick start

```bash
# rust side
cargo build -p rivulet-cli
cargo run -p rivulet-cli -- version

# js side
pnpm install
pnpm --filter @rivulet/js build
```

```ts
import { RivuletClient } from "@rivulet/js";

const client = new RivuletClient();
const doc = client.open();

// apply ops, sync with peers, go offline, touch grass, come back
console.log(doc.id);
```

examples under `examples/` — todo list, whiteboard, code editor shell, presence-only chat layer.

---

## layout

```
crates/
  rivulet-core/       # actors, dots, version vectors, ops
  rivulet-crdt/       # LWW map, RGA text, OR-list, PN-counter, graph
  rivulet-sync/       # protocol + session + transport trait
  rivulet-storage/    # memory / sqlite / fs
  rivulet-presence/   # ephemeral peer state
  rivulet-query/      # path lookups over snapshots
  rivulet-crypto/     # e2ee stubs (experimental, don't get cocky)
  rivulet-relay/      # optional catch-up server logic
  rivulet-cli/        # binary

packages/
  rivulet-js/         # browser/node client
  rivulet-react/
  rivulet-solid/
  rivulet-vue/

apps/                 # relay server, playground, docs site
examples/             # copy-paste and break things
tests/                # integration, property, chaos
docs/                 # protocol + architecture notes
```

---

## honest section

- a chunk of the networking and sqlite paths are stubs. the types and the crdt math are the real spine.
- concurrent text edits will look weird for a moment. that's interleaving, not a ghost.
- e2ee is marked experimental for a reason.
- if two peers diverge forever, check your version vectors before filing a haunted bug.

---

## status

v0.4.x — usable for experiments and demos. production? depends how brave you are and how good your tests are.

---

## license

apache 2.0. go build something that still works on a train.

---

offline is a feature, not a failure mode.
