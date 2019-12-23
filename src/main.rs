#![allow(unused)]

#[macro_use]
extern crate speedy;

mod formats;
mod structure;

use crate::formats::bincode::{deserialize_from_bincode, serialize_to_bincode};
use crate::formats::cbor::{deserialize_from_cbor, serialize_to_cbor};
use crate::formats::csv::{deserialize_from_csv, serialize_to_csv};
use crate::formats::json::{deserialize_from_json, serialize_to_json};
use crate::formats::message_pack::{deserialize_from_message_pack, serialize_to_message_pack};
use crate::formats::protobuf::{deserialize_from_protobuf, serialize_to_protobuf};
use crate::formats::speedy::{deserialize_from_speedy, serialize_to_speedy};
use crate::structure::tick::Tick;
use clap::{App, AppSettings, Arg, SubCommand};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::time::Instant;
use std::{env, fs};
//use bencher::

const SUBC_GENERATE: &'static str = "generate";
const SUBC_BENCHMARK: &'static str = "benchmark";

fn cwd() -> String {
    env::current_dir().unwrap().to_str().unwrap().to_string()
}

fn run_generate() {
    use protoc_rust::Customize;
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/formats/protobuf/protos",
        input: &["src/formats/protobuf/protos/tick.proto"],
        includes: &["src/formats/protobuf/protos"],
        customize: Customize {
            serde_derive: Some(true),
            carllerche_bytes_for_bytes: Some(true),
            carllerche_bytes_for_string: Some(true),
            ..Default::default()
        },
        ..Default::default()
    })
    .expect("protoc");

    //    capnpc::CompilerCommand::new()
    //        .file("src/formats/capnp/capnp/tick.capnp")
    //        .output_path("src/formats/capnp")
    //        .run()
    //        .expect("capnpc");
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
            "{:<15}  duration: {:>10.03} s   size: {:>10.03} kb",
            name,
            secs,
            File::open(path).unwrap().metadata().unwrap().len() as f64 / 1024_f64
        );
    }
}

fn benchmark_deserialize<T>(name: &str, bufsize: usize, func: T)
where
    T: Fn(BufReader<File>) -> usize,
{
    let f = File::open(Path::new(format!(".tmp/serialize/{}.dat", name).as_str())).unwrap();
    let f = BufReader::with_capacity(bufsize, f);

    let now = Instant::now();
    let ticks_len = func(f);
    let secs = now.elapsed().as_secs_f32();

    if secs >= 0.001 {
        println!("{:<15}  duration: {:>10.03} s", name, secs);
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
    benchmark_serialize(&ticks, "message_pack", bufsize, serialize_to_message_pack);
    benchmark_serialize(&ticks, "bincode", bufsize, serialize_to_bincode);
    benchmark_serialize(&ticks, "speedy", bufsize, serialize_to_speedy);

    println!("\ndeserialization results (bufsize={})):", bufsize);
    benchmark_deserialize("csv", bufsize, deserialize_from_csv);
    benchmark_deserialize("json", bufsize, deserialize_from_json);
    benchmark_deserialize("protobuf", bufsize, deserialize_from_protobuf);
    benchmark_deserialize("cbor", bufsize, deserialize_from_cbor);
    benchmark_deserialize("message_pack", bufsize, deserialize_from_message_pack);
    benchmark_deserialize("bincode", bufsize, deserialize_from_bincode);
    benchmark_deserialize("speedy", bufsize, deserialize_from_speedy);
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
