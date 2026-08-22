rivulet, as far as I have actually proven

1. version vectors increment, merge, and notice remote dots. that part has tests.

2. a document has local_op and apply. apply does not steal the local actor counter. I got that wrong once, so it is now a test, not a comment.

3. two peers in one process exchange missing ops and finish with the same set. `rivulet demo` is that dance.

4. the nasty merge cases are still more written down than proven. I do not have a production offline queue or a relay I would put on the internet.

If you want to watch the two peers converge:

cargo test --workspace && cargo build -p rivulet-cli && ./target/debug/rivulet demo

Apache-2.0
