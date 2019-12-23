# rust-ser-deser-benchmark

```bash
cargo run --release -- benchmark_serialize

cargo run --release -- benchmark_deserialize
```

## Results

- test stand: CPU: `i7-6850K CPU @ 3.60GHz one thread`

- initial csv file size:  `48565.713 kb`
- rows in csv file: `2026424`

```text
serialization (bufsize=16384)):
csv                duration:      0.225s     size:  48565.713 kb
json               duration:      0.508s     size: 117813.541 kb
cbor               duration:      0.238s     size:  81138.500 kb
message_pack       duration:      0.159s     size:  27690.138 kb
bincode            duration:      0.076s     size:  39574.344 kb

deserialization (bufsize=16384)):
csv                duration:      0.385s
json               duration:      1.585s
cbor               duration:      0.692s
message_pack       duration:      0.165s
bincode            duration:      0.093s
```
