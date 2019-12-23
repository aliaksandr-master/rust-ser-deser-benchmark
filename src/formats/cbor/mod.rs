use crate::structure::tick::Tick;
use std::fs::File;
use std::io::{BufReader, BufWriter};

pub fn deserialize_from_cbor(f: BufReader<File>) -> usize {
    let ticks: Vec<Tick> = serde_cbor::from_reader::<Vec<Tick>, _>(f).unwrap();

    ticks.len()
}

pub fn serialize_to_cbor(ticks: &Vec<Tick>, f: &mut BufWriter<File>) {
    serde_cbor::to_writer(f, ticks).unwrap();
}
