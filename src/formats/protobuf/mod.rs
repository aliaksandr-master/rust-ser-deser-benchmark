mod protos;
use crate::formats::protobuf::protos::tick::ProtoTick;
use crate::structure::tick::Tick;
use protobuf::{parse_from_bytes, parse_from_reader, Message};
use protos::tick::ProtoTickList;
use std::fs::File;
use std::io::{BufReader, BufWriter};

pub fn deserialize_from_protobuf(mut f: BufReader<File>) -> usize {
    let list = parse_from_reader::<ProtoTickList>(&mut f).unwrap();

    list.ticks
        .iter()
        .map(|x| Tick {
            timestamp: x.timestamp,
            price_ips: x.price_ips,
            volume: x.volume,
        })
        .len()
}

pub fn serialize_to_protobuf(ticks: &Vec<Tick>, f: &mut BufWriter<File>) {
    let mut list = ProtoTickList::new();

    list.ticks = ticks
        .iter()
        .map(|x| {
            let mut t = ProtoTick::new();
            t.timestamp = x.timestamp;
            t.price_ips = x.price_ips;
            t.volume = x.volume;
            t
        })
        .collect();

    list.write_to_writer(f).unwrap();
}
