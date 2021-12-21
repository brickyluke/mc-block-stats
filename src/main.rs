use std::collections::HashMap;
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
            println!("Processing Chunk: x={} / z={}", x, z);

            let compound: HashMap<String, Value> =
                fastnbt::de::from_bytes(data.as_slice()).unwrap();
            match compound["DataVersion"] {
                Value::Int(ver) => println!("Version: {}", ver),
                _ => {}
            }
            // println!("{:#?}", compound);
        })
        .map_err(|e| e.into())
}
