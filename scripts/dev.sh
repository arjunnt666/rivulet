#!/usr/bin/env bash
set -e
echo "rivulet dev: building cli..."
cargo build -p rivulet-cli 2>/dev/null || echo "(rust build skipped — toolchain missing is fine for the hollow tour)"
