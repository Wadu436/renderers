benchmark:
    ./benchmark/benchmark.sh

profile:
    sudo sysctl kernel.perf_event_mlock_kb=2048

setup-assets:
    #!/usr/bin/env sh
    set -eu

    manifest="./assets/manifest.json"
    scene="./assets/scenes"

    rm -rf "$scene"
    mkdir -p "$scene"

    items=$(jq -r 'to_entries[] | "\(.key) \(.value.url) \(.value.type)"' "$manifest")

    echo "$items" | while read name url type; do
      [ "$type" = "zip" ] || continue

      outdir="$scene/$name"
      zipfile="$scene/$name.zip"

      curl -L "$url" -o "$zipfile"
      mkdir -p "$outdir"
      unzip -q "$zipfile" -d "$outdir"

      rm "$zipfile"
    done