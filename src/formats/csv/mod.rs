use crate::structure::tick::Tick;
use std::fs::File;
use std::io::{BufReader, BufWriter};

pub fn deserialize_from_csv(f: BufReader<File>) -> Vec<Tick> {
    let ticks: Vec<Tick> = csv::Reader::from_reader(f)
        .deserialize::<Tick>()
        .map(|x| x.unwrap())
        .collect();
    ticks
}

pub fn serialize_to_csv(ticks: &Vec<Tick>, f: &mut BufWriter<File>) {
    let mut w = csv::Writer::from_writer(f);
    for d in ticks {
        w.serialize(d).unwrap();
    }
    w.flush().unwrap();
}
