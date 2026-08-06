# crdt notes

- LWW map: simple, loses concurrent writes on purpose
- RGA text: positions are the hard part
- OR-list: add-wins, order is best-effort
- PN-counter: increments and decrements from anyone
- graph: nodes/edges as add-wins sets

if you need something fancier, extension payloads exist. good luck.
