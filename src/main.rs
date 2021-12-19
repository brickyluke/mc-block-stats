use std::error::Error;
use std::fs::File;

use clap::{App, Arg};
use fastanvil::RegionBuffer;
use fastnbt::Value;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("mc-block-stats")
        .arg(Arg::with_name("file").required(true))
        .get_matches();

    let file = matches.value_of("file").expect("file is required");
    let file = File::open(file).expect("file does not exist");

    let mut region = RegionBuffer::new(file);

    region
        .for_each_chunk(|x, z, data| {
            println!("x={} / z={}", x, z);
            // deserialises the whole chunk which takes a while...
            let _chunk: Value = fastnbt::de::from_bytes(data).unwrap();
        })
        .map_err(|e| e.into())
}
