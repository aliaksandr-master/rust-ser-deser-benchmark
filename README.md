# rust-ser-deser-benchmark

```bash
cargo run --release -- benchmark_serialize

cargo run --release -- benchmark_deserialize
```

## Libraries

- [speedy](https://crates.io/crates/speedy)
- [serde_json](https://crates.io/crates/serde_json)
- [csv](https://crates.io/crates/csv)
- [bincode](https://crates.io/crates/bincode)
- [serde_cbor](https://crates.io/crates/serde_cbor)
- [rmp-serde](https://crates.io/crates/rmp-serde)
- [protobuf](https://crates.io/crates/protobuf)


## Results

- test stand: CPU: `i7-6850K CPU @ 3.60GHz one thread`

- initial csv file size:  `48565.713 kb`
- rows in csv file: `2026424`
- data format:

| timestamp |  price_ips |  volume |
|---|---|---|
| f64 | u64 | u32 |

```text
serialization results (bufsize=16384)):
                 duration
json             0.529 s |||||||   117813.541 kb  |||||||||
cbor             0.241 s ||||||     81138.500 kb  |||||||| 
csv              0.221 s |||||      48565.713 kb  |||||||  
message_pack     0.165 s ||||       27690.138 kb  |||   
protobuf         0.158 s |||        31663.314 kb  ||||    
bincode          0.079 s ||         39574.344 kb  ||||||     
speedy           0.073 s |          39574.340 kb  |||||      


deserialization results (bufsize=16384)):
                 duration
json             1.609 s |||||||
cbor             0.717 s |||||| 
csv              0.377 s |||||  
message_pack     0.166 s ||||   
protobuf         0.156 s |||    
bincode          0.092 s ||     
speedy           0.079 s |      
```
