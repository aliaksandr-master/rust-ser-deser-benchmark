use crate::structure::tick::Tick;
use std::fs::File;
use std::io::{BufReader, BufWriter};

pub fn deserialize_from_bincode(f: BufReader<File>) -> Vec<Tick> {
    bincode::deserialize_from(f).unwrap()
}

pub fn serialize_to_bincode(ticks: &Vec<Tick>, f: &mut BufWriter<File>) {
    bincode::serialize_into(f, ticks).unwrap();
}
