## How to profile

Set up the kernel to allow Samply

```
echo '1' | sudo tee /proc/sys/kernel/perf_event_paranoid
sudo sysctl kernel.perf_event_mlock_kb=2048
cargo build --profile profiling && samply record ./target/profiling/cli --renderer cpu-ray-tracer --format ppm --camera-x 2 --camera-y 1 --camera-z 1 --resolution-x 1920 --resolution-y 1080 -o render.ppm
```
