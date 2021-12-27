use std::collections::BTreeMap;
use std::fs::File;

use clap::{App, Arg};
use fastanvil::{Chunk, JavaChunk, RegionBuffer};

fn main() {
    let matches = App::new("mc-block-stats")
        .arg(Arg::with_name("file").required(true))
        .get_matches();

    let file = matches.value_of("file").expect("file is required");
    let file = File::open(file).expect("file does not exist");
    let mut final_counts: BTreeMap<String, isize> = BTreeMap::new();

    let mut region = RegionBuffer::new(file);

    let _ = region.for_each_chunk(|x, z, data| {
        let chunk: JavaChunk = fastnbt::de::from_bytes(data.as_slice()).unwrap();
        println!(
            "Chunk( x={}, z={} ) -> y = [{}..{}]",
            x,
            z,
            chunk.y_range().start,
            chunk.y_range().end
        );
        let mut counts: BTreeMap<&str, isize> = BTreeMap::new();
        for chunk_y in chunk.y_range() {
            for chunk_x in 0..15 {
                for chunk_z in 0..15 {
                    let material = chunk.block(chunk_x, chunk_y, chunk_z).unwrap().name();
                    if material != "minecraft:air" {
                        *(counts.entry(material).or_insert(0)) += 1;
                    }
                }
            }
        }
        for (material, count) in counts {
            *(final_counts.entry(material.to_string()).or_insert(0)) += count;
        }
    });

    for (k, v) in final_counts {
        println!("{} -> {}", k, v);
    }
}
