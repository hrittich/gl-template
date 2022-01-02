#!/bin/bash
SRC="$(cd "$(dirname "$0")"; pwd)"
export CARGO_TARGET_DIR="$SRC/target"

D="$(mktemp -d --tmpdir gl-template.XXXXXX)"
(cd "$D" \
  && cargo generate --init --name demo --path "$SRC" \
     -d gl-major=4 -d gl-minor=1 -d gl-profile=Core \
  && cargo run) \
  && rm -r "$D"
