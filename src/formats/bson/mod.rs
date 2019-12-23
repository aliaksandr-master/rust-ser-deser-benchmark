use crate::structure::tick::Tick;
use std::fs::File;
use std::io::{BufReader, BufWriter};

pub fn deserialize_from_bson(f: BufReader<File>) -> Vec<Tick> {
    vec![]
}

pub fn serialize_to_bson(ticks: &Vec<Tick>, f: &mut BufWriter<File>) {
    // It doesn't work yet
    //    let b = bson::to_bson(ticks).unwrap();
    //    f.write(b.as_str().unwrap().as_bytes()).unwrap();
    //    f.flush().unwrap();
}
