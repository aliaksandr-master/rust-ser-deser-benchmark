use crate::structure::tick::Tick;
use std::fs::File;
use std::io::{BufReader, BufWriter};

pub fn deserialize_from_message_pack(f: BufReader<File>) -> usize {
    let ticks: Vec<Tick> = rmp_serde::decode::from_read(f).unwrap();

    ticks.len()
}

pub fn serialize_to_message_pack(ticks: &Vec<Tick>, f: &mut BufWriter<File>) {
    rmp_serde::encode::write(f, ticks).unwrap()
}
