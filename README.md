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
serialization results (bufsize=16384)):
csv              duration:      0.221 s   size:  48565.713 kb
json             duration:      0.529 s   size: 117813.541 kb
protobuf         duration:      0.158 s   size:  31663.314 kb
cbor             duration:      0.241 s   size:  81138.500 kb
message_pack     duration:      0.165 s   size:  27690.138 kb
bincode          duration:      0.079 s   size:  39574.344 kb
speedy           duration:      0.073 s   size:  39574.340 kb

deserialization results (bufsize=16384)):
csv              duration:      0.377 s
json             duration:      1.609 s
protobuf         duration:      0.156 s
cbor             duration:      0.717 s
message_pack     duration:      0.166 s
bincode          duration:      0.092 s
speedy           duration:      0.079 s
```
