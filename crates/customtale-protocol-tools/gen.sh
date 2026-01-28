#!/bin/bash

set -euo pipefail
cd "$(dirname "$0")"

HYTALE_JAR="$HOME/Library/Application Support/Hytale/install/release/package/game/latest/Server/HytaleServer.jar"
OUT_DIR="$(realpath ../customtale-protocol/src/generated)"

echo "$HYTALE_JAR"

./gradlew run --args \
    "\"$HYTALE_JAR\" \"$OUT_DIR\""\

cargo test -p customtale-protocol
