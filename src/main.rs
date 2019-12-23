#![allow(unused)]

mod formats;
mod structure;

use crate::structure::tick::Tick;
use clap::{App, AppSettings, Arg, SubCommand};
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::time::Instant;
//use bencher::

const SUBC_GENERATE: &'static str = "generate";
const SUBC_BENCHMARK: &'static str = "benchmark";

fn run_generate() {
    unimplemented!("unimplemented");
}

fn serialize_to_protobuf(ticks: &Vec<Tick>, f: &mut BufWriter<File>) {
    // TODO
}

fn serialize_to_cbor(ticks: &Vec<Tick>, f: &mut BufWriter<File>) {
    serde_cbor::to_writer(f, ticks).unwrap();
}

fn serialize_to_csv(ticks: &Vec<Tick>, f: &mut BufWriter<File>) {
    let mut w = csv::Writer::from_writer(f);
    for d in ticks {
        w.serialize(d).unwrap();
    }
    w.flush().unwrap();
}

fn serialize_to_json(ticks: &Vec<Tick>, f: &mut BufWriter<File>) {
    serde_json::to_writer(f, ticks).unwrap();
}

fn serialize_to_rmp(ticks: &Vec<Tick>, f: &mut BufWriter<File>) {
    rmp_serde::encode::write(f, ticks).unwrap()
}

fn serialize_to_bincode(ticks: &Vec<Tick>, f: &mut BufWriter<File>) {
    bincode::serialize_into(f, ticks).unwrap();
}

fn serialize_to_bson(ticks: &Vec<Tick>, f: &mut BufWriter<File>) {
    // It doesn't work yet
    //    let b = bson::to_bson(ticks).unwrap();
    //    f.write(b.as_str().unwrap().as_bytes()).unwrap();
    //    f.flush().unwrap();
}

fn benchmark_serialize<T>(ticks: &Vec<Tick>, name: &str, bufsize: usize, func: T)
where
    T: Fn(&Vec<Tick>, &mut BufWriter<File>),
{
    let path = format!(".tmp/serialize/{}.dat", name);
    let path = Path::new(path.as_str());
    let f = File::create(path).unwrap();
    let mut f = BufWriter::with_capacity(bufsize, f);

    let now = Instant::now();
    func(&ticks, &mut f);
    let secs = now.elapsed().as_secs_f32();

    if secs >= 0.001 {
        println!(
            "{:<15}    duration: {:>10.03}s     size: {:>10.03} kb",
            name,
            secs,
            File::open(path).unwrap().metadata().unwrap().len() as f64 / 1024_f64
        );
    }
}

fn deserialize_from_csv(f: BufReader<File>) -> Vec<Tick> {
    let ticks: Vec<Tick> = csv::Reader::from_reader(f)
        .deserialize::<Tick>()
        .map(|x| x.unwrap())
        .collect();
    ticks
}
fn deserialize_from_json(f: BufReader<File>) -> Vec<Tick> {
    serde_json::from_reader::<_, Vec<Tick>>(f).unwrap()
}
fn deserialize_from_protobuf(f: BufReader<File>) -> Vec<Tick> {
    vec![]
}
fn deserialize_from_cbor(f: BufReader<File>) -> Vec<Tick> {
    serde_cbor::from_reader(f).unwrap()
}
fn deserialize_from_rmp(f: BufReader<File>) -> Vec<Tick> {
    rmp_serde::decode::from_read(f).unwrap()
}
fn deserialize_from_bincode(f: BufReader<File>) -> Vec<Tick> {
    bincode::deserialize_from(f).unwrap()
}
fn deserialize_from_bson(f: BufReader<File>) -> Vec<Tick> {
    vec![]
}

fn benchmark_deserialize<T>(name: &str, bufsize: usize, func: T)
where
    T: Fn(BufReader<File>) -> Vec<Tick>,
{
    let f = File::open(Path::new(format!(".tmp/serialize/{}.dat", name).as_str())).unwrap();
    let f = BufReader::with_capacity(bufsize, f);

    let now = Instant::now();
    let ticks_len = func(f).len();
    let secs = now.elapsed().as_secs_f32();

    if secs >= 0.001 {
        println!("{:<15}    duration: {:>10.03}s", name, secs);
    }
}

fn run_benchmark(src_file: &str) {
    let init_now = Instant::now();
    let f = File::open(src_file).unwrap();
    let ticks: Vec<Tick> = csv::Reader::from_reader(&f)
        .deserialize::<Tick>()
        .map(|x| x.unwrap())
        .collect();

    println!(
        "initial    duration {:?}    size: {:>10.03} kb",
        init_now.elapsed(),
        f.metadata().unwrap().len() as f64 / 1024_f64
    );

    fs::create_dir_all(Path::new(".tmp/serialize")).unwrap();

    let bufsize = 16384;

    println!("\nserialization results (bufsize={})):", bufsize);
    benchmark_serialize(&ticks, "csv", bufsize, serialize_to_csv);
    benchmark_serialize(&ticks, "json", bufsize, serialize_to_json);
    benchmark_serialize(&ticks, "protobuf", bufsize, serialize_to_protobuf);
    benchmark_serialize(&ticks, "cbor", bufsize, serialize_to_cbor);
    benchmark_serialize(&ticks, "message_pack", bufsize, serialize_to_rmp);
    benchmark_serialize(&ticks, "bincode", bufsize, serialize_to_bincode);
    benchmark_serialize(&ticks, "bson", bufsize, serialize_to_bson);

    println!("\ndeserialization results (bufsize={})):", bufsize);
    benchmark_deserialize("csv", bufsize, deserialize_from_csv);
    benchmark_deserialize("json", bufsize, deserialize_from_json);
    benchmark_deserialize("protobuf", bufsize, deserialize_from_protobuf);
    benchmark_deserialize("cbor", bufsize, deserialize_from_cbor);
    benchmark_deserialize("message_pack", bufsize, deserialize_from_rmp);
    benchmark_deserialize("bincode", bufsize, deserialize_from_bincode);
    benchmark_deserialize("bson", bufsize, deserialize_from_bson);
}

fn main() {
    let matches = App::new("rust-ser-deser-benchark")
        .version("0.0.1")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name(SUBC_GENERATE))
        .subcommand(
            SubCommand::with_name(SUBC_BENCHMARK).arg(
                Arg::with_name("file")
                    .long("file")
                    .takes_value(true)
                    .default_value("./data/test.csv"),
            ),
        )
        .get_matches();

    match matches.subcommand_name().unwrap() {
        SUBC_GENERATE => {
            run_generate();
        }
        SUBC_BENCHMARK => {
            let serialize_arg = matches.subcommand_matches(SUBC_BENCHMARK).unwrap();

            let src_file = serialize_arg.value_of("file").unwrap();

            run_benchmark(src_file);
        }
        _ => panic!("invalid param"),
    };
}
