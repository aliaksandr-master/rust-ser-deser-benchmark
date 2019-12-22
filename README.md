# rust-ser-deser-benchmark

```bash
cargo run --release -- benchmark_serialize

cargo run --release -- benchmark_deserialize
```

## Results

- test stand: Dell XPS, CPU: `Intel i7-8550U (4 cores, 8 threads)`

- initial csv file size:  `48565.713 kb`
- rows in csv file: `2026424`

```text
deserialization results:
csv                duration:      0.385s
json               duration:     77.258s
cbor               duration:     16.245s
message_pack       duration:      7.561s
bincode            duration:      3.716s
```

```text
serialization results:
csv                duration:      0.215s     size:  48565.713 kb
json               duration:     53.301s     size: 117828.227 kb
cbor               duration:     27.566s     size:  81149.124 kb
message_pack       duration:     16.442s     size:  27705.913 kb
bincode            duration:      8.901s     size:  39578.602 kb

```
