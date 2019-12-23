use crate::structure::tick::Tick;
use std::fs::File;
use std::io::{BufReader, BufWriter};

pub fn deserialize_from_cbor(f: BufReader<File>) -> Vec<Tick> {
    serde_cbor::from_reader(f).unwrap()
}

pub fn serialize_to_cbor(ticks: &Vec<Tick>, f: &mut BufWriter<File>) {
    serde_cbor::to_writer(f, ticks).unwrap();
}
