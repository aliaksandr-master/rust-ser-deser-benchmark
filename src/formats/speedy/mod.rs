use crate::structure::tick::Tick;
use speedy::{Endianness, Readable, Reader, Writable, Writer};
use std::fs::File;
use std::io::{BufReader, BufWriter};

pub fn deserialize_from_speedy(f: BufReader<File>) -> usize {
    let endian = Endianness::LittleEndian;

    let ticks: Vec<Tick> = Readable::read_from_stream(endian, f).unwrap();

    ticks.len()
}

pub fn serialize_to_speedy(ticks: &Vec<Tick>, f: &mut BufWriter<File>) {
    let endian = Endianness::LittleEndian;

    Writable::write_to_stream(ticks, endian, f).unwrap();
}
