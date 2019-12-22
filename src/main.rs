mod formats;
mod structure;

#[macro_use]
extern crate bencher;
use bencher::Bencher;

use crate::structure::tick::Tick;
use clap::{App, AppSettings, Arg, SubCommand};
use std::fs;
use std::fs::File;
use std::path::Path;
use std::time::Instant;
//use bencher::

const SUBC_GENERATE: &'static str = "generate";
const SUBC_BENCHMARK_SERIALIZE: &'static str = "benchmark_serialize";
const SUBC_BENCHMARK_DESERIALIZE: &'static str = "benchmark_deserialize";

fn run_generate() {
    unimplemented!("unimplemented");
}

fn serialize_to_protobuf(ticks: &Vec<Tick>, f: &mut File) {}
fn serialize_to_cbor(ticks: &Vec<Tick>, f: &mut File) {
    serde_cbor::to_writer(f, ticks).unwrap();
}
fn serialize_to_csv(ticks: &Vec<Tick>, f: &mut File) {
    let mut w = csv::Writer::from_writer(f);
    for d in ticks {
        w.serialize(d).unwrap();
    }
    w.flush().unwrap();
}
fn serialize_to_json(ticks: &Vec<Tick>, f: &mut File) {
    serde_json::to_writer(f, ticks).unwrap();
}
fn serialize_to_rmp(ticks: &Vec<Tick>, f: &mut File) {
    rmp_serde::encode::write(f, ticks).unwrap()
}
fn serialize_to_bincode(ticks: &Vec<Tick>, f: &mut File) {}
fn serialize_to_bson(ticks: &Vec<Tick>, f: &mut File) {}

fn benchmark_serialize<T>(ticks: &Vec<Tick>, name: &str, func: T)
where
    T: Fn(&Vec<Tick>, &mut File),
{
    let now = Instant::now();
    let mut f = File::create(Path::new(format!(".tmp/serialize/{}.dat", name).as_str())).unwrap();
    func(&ticks, &mut f);

    let secs = now.elapsed().as_secs_f32();

    if secs >= 0.1 {
        println!(
            "{:<15}    duration: {:>10.03}s     size: {:>10.03} kb",
            name,
            secs,
            f.metadata().unwrap().len() as f64 / 1024_f64
        );
    }
}

fn run_serialize(src_file: &str) {
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

    println!("\nserialization results:");
    benchmark_serialize(&ticks, "csv", serialize_to_csv);
    benchmark_serialize(&ticks, "json", serialize_to_json);
    benchmark_serialize(&ticks, "protobuf", serialize_to_protobuf);
    benchmark_serialize(&ticks, "cbor", serialize_to_cbor);
    benchmark_serialize(&ticks, "message_pack", serialize_to_rmp);
    benchmark_serialize(&ticks, "message_bincode", serialize_to_bincode);
    benchmark_serialize(&ticks, "bson", serialize_to_bson);
}

fn run_deserialize() {
    unimplemented!("unimplemented");
    //    fn a(bench: &mut Bencher) {
    //        bench.iter(|| (0..1000).fold(0, |x, y| x + y))
    //    }
    //
    //    fn b(bench: &mut Bencher) {
    //        const N: usize = 1024;
    //        bench.iter(|| vec![0u8; N]);
    //
    //        bench.bytes = N as u64;
    //    }
    //
    //    benchmark_group!(benches, a, b);
    //    benchmark_main!(benches);
}

fn main() {
    let matches = App::new("rust-ser-deser-benchark")
        .version("0.0.1")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name(SUBC_GENERATE))
        .subcommand(
            SubCommand::with_name(SUBC_BENCHMARK_SERIALIZE).arg(
                Arg::with_name("file")
                    .long("file")
                    .takes_value(true)
                    .default_value("./data/test.csv"),
            ),
        )
        .subcommand(SubCommand::with_name(SUBC_BENCHMARK_DESERIALIZE))
        .get_matches();

    match matches.subcommand_name().unwrap() {
        SUBC_GENERATE => {
            run_generate();
        }
        SUBC_BENCHMARK_SERIALIZE => {
            let serialize_arg = matches
                .subcommand_matches(SUBC_BENCHMARK_SERIALIZE)
                .unwrap();

            let src_file = serialize_arg.value_of("file").unwrap();

            run_serialize(src_file);
        }
        SUBC_BENCHMARK_DESERIALIZE => {
            run_deserialize();
        }
        _ => panic!("invalid param"),
    };
}
