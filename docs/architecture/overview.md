# architecture

clients own the data. relay is a cache with manners.

```
┌──────────┐     sync protocol      ┌──────────┐
│  client  │ ◄────────────────────► │  client  │
└────┬─────┘                        └────┬─────┘
     │         optional relay            │
     └────────────►┌───────┐◄────────────┘
                   │ relay │
                   └───────┘
```

crdts live in `rivulet-crdt`. causality in `rivulet-core`. network in `rivulet-sync`.
