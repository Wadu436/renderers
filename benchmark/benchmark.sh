#!/usr/bin/env bash

BENCHMARK_DIR="$(dirname "${BASH_SOURCE[0]}")"
RUN_ID="$(date +%Y%m%d_%H%M%S)"
RESULTS_DIR="$BENCHMARK_DIR/results/$RUN_ID"

cargo build --release
mkdir -p "$RESULTS_DIR"

BIN="$BENCHMARK_DIR/../target/release/cli"
# renderers=("cpu-rasterizer" "cpu-ray-tracer")
renderers=("cpu-ray-tracer")

for renderer in "${renderers[@]}"; do
    echo "Benchmarking $renderer..."
    OUTPUT_FILE="$RESULTS_DIR/${renderer}_benchmark.ppm"
    OUTPUT_FILE_PNG="$RESULTS_DIR/${renderer}_benchmark.png"
    LOG_FILE="$RESULTS_DIR/${renderer}_run.log"
    JSON_FILE="$RESULTS_DIR/${renderer}_hyperfine.json"

    hyperfine --warmup 2 \
        --export-json "$JSON_FILE" \
        "$BIN --renderer $renderer --format ppm --camera-x 10 --camera-y 10 --camera-z 10 > $OUTPUT_FILE 2> $LOG_FILE"

    magick $OUTPUT_FILE $OUTPUT_FILE_PNG 

    echo "Saved benchmark result to $OUTPUT_FILE (stats in $JSON_FILE)"
done
