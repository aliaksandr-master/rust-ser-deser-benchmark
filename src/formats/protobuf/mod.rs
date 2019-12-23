use crate::structure::tick::Tick;
use std::fs::File;
use std::io::{BufReader, BufWriter};

pub fn deserialize_from_protobuf(f: BufReader<File>) -> Vec<Tick> {
    vec![]
}

pub fn serialize_to_protobuf(ticks: &Vec<Tick>, f: &mut BufWriter<File>) {
    // TODO
}
