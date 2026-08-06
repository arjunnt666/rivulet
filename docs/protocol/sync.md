# sync protocol (sketch)

1. hello with version vector
2. request missing ops for a doc
3. response with ops + updated vv
4. optional op broadcast for live peers

that's the whole movie. everything else is retries and edge cases.
