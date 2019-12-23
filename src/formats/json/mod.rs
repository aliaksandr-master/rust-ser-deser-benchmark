use crate::structure::tick::Tick;
use std::fs::File;
use std::io::{BufReader, BufWriter};

pub fn deserialize_from_json(f: BufReader<File>) -> Vec<Tick> {
    serde_json::from_reader::<_, Vec<Tick>>(f).unwrap()
}

pub fn serialize_to_json(ticks: &Vec<Tick>, f: &mut BufWriter<File>) {
    serde_json::to_writer(f, ticks).unwrap();
}
