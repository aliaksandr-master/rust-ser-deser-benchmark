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
csv              duration:      0.226 s   size:  48565.713 kb
json             duration:      0.515 s   size: 117813.541 kb
protobuf         duration:      0.150 s   size:  31663.314 kb
cbor             duration:      0.238 s   size:  81138.500 kb
message_pack     duration:      0.153 s   size:  27690.138 kb
bincode          duration:      0.076 s   size:  39574.344 kb

deserialization results (bufsize=16384)):
csv              duration:      0.382 s
json             duration:      1.598 s
protobuf         duration:      0.143 s
cbor             duration:      0.692 s
message_pack     duration:      0.163 s
bincode          duration:      0.093 s
```
