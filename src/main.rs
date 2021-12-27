use std::collections::BTreeMap;
use std::fs::File;

use clap::{App, Arg};
use fastanvil::{Chunk, JavaChunk, RegionBuffer};

const IGNORE_BLOCKS: &[&str] = &["minecraft:air", "minecraft:cave_air"];
const CHUNK_FULL: &str = "full";

fn main() {
    let matches = App::new("mc-block-stats")
        .arg(Arg::with_name("file").required(true))
        .get_matches();

    let file_path = matches.value_of("file").expect("file is required");

    let mut final_counts: BTreeMap<String, Vec<isize>> = BTreeMap::new();

    let file = File::open(file_path).expect("file does not exist");
    let mut region = RegionBuffer::new(file);
    let _ = region.for_each_chunk(|x, z, data| {
        let chunk: JavaChunk = fastnbt::de::from_bytes(data.as_slice()).unwrap();
        println!(
            "Chunk( x={}, z={}, status={} ) -> y = [{}..{}]",
            x,
            z,
            chunk.status(),
            chunk.y_range().start,
            chunk.y_range().end
        );

        if chunk.status() == CHUNK_FULL {
            let mut counts: BTreeMap<&str, Vec<isize>> = BTreeMap::new();
            // let world_height = usize::try_from(chunk.y_range().end - chunk.y_range().start).unwrap();
            let world_height = 256; // hard-coded for 1.17 -> will break things for 1.18!
            let height_offset = 0 - chunk.y_range().start;

            for chunk_y in chunk.y_range() {
                let counter_idx = usize::try_from(height_offset + chunk_y).unwrap();
                for chunk_x in 0..16 {
                    for chunk_z in 0..16 {
                        let block_type = chunk.block(chunk_x, chunk_y, chunk_z).unwrap().name();
                        if !IGNORE_BLOCKS.iter().any(|&i| i == block_type) {
                            // increase counter at the given height
                            (*(counts.entry(block_type).or_insert(vec![0; world_height])))
                                [counter_idx] += 1;
                        }
                    }
                }
            }

            for (block_type, count) in counts {
                final_counts
                    .entry(block_type.to_string())
                    .and_modify(|t| {
                        // this can fail if not all chunks have the same world height
                        for (i, total) in t.iter_mut().enumerate() {
                            *total += count[i]
                        }
                    })
                    .or_insert(count);
            }
        }
    });
    for (k, v) in final_counts {
        println!("{} -> {:?}", k, v);
    }
}
