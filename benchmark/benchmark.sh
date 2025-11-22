#!/usr/bin/env bash

BENCHMARK_DIR="$(dirname "${BASH_SOURCE[0]}")"
RUN_ID="$(date +%Y%m%d_%H%M%S)"
RESULTS_DIR="$BENCHMARK_DIR/results/$RUN_ID"

# RESOLUTION_X=800
# RESOLUTION_Y=450

RESOLUTION_X=1920
RESOLUTION_Y=1080

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

    # Generate a picture for verifying the output
    $BIN --renderer $renderer --format ppm --camera-x 2 --camera-y 1 --camera-z 1 --resolution-x $RESOLUTION_X --resolution-y $RESOLUTION_Y -o $OUTPUT_FILE &> $LOG_FILE

    hyperfine --warmup 2 \
        --export-json "$JSON_FILE" \
        "$BIN --renderer $renderer --format none --camera-x 2 --camera-y 1 --camera-z 1 --resolution-x $RESOLUTION_X --resolution-y $RESOLUTION_Y"

    magick $OUTPUT_FILE $OUTPUT_FILE_PNG 

    echo "Saved benchmark result to $OUTPUT_FILE_PNG (stats in $JSON_FILE)"
done
