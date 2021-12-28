use std::collections::BTreeMap;
use std::fs::File;
use std::sync::mpsc::channel;

use clap::{App, Arg};
use threadpool::ThreadPool;

use fastanvil::{Chunk, JavaChunk, RegionBuffer};

const IGNORE_BLOCKS: &[&str] = &["minecraft:air", "minecraft:cave_air"];
const CHUNK_FULL: &str = "full";

fn main() {
    let matches = App::new("mc-block-stats")
        .arg(Arg::with_name("file").required(true))
        .get_matches();

    //    let file_path = matches.value_of("file").expect("file is required");

    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();
    {
        // TODO: loop over region files
        let tx = tx.clone();

        pool.execute(move || {
            let file_path = matches.value_of("file").expect("file is required");
            let file = File::open(file_path).expect("file does not exist");

            let mut region = RegionBuffer::new(file);
            let mut region_counts: BTreeMap<String, Vec<isize>> = BTreeMap::new();

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
                    // let world_height = usize::try_from(chunk.y_range().end - chunk.y_range().start).unwrap();
                    let world_height = 256; // hard-coded for 1.17 -> will break things for 1.18!
                    let height_offset = 0 - chunk.y_range().start;

                    for chunk_y in chunk.y_range() {
                        let counter_idx = usize::try_from(height_offset + chunk_y).unwrap();
                        for chunk_x in 0..16 {
                            for chunk_z in 0..16 {
                                let block_type =
                                    chunk.block(chunk_x, chunk_y, chunk_z).unwrap().name();
                                // skip blocks we're not interested in
                                if !IGNORE_BLOCKS.iter().any(|&i| i == block_type) {
                                    // can't use the entry API without changing ownership, which is expensive
                                    if region_counts.contains_key(block_type) {
                                        (*region_counts.get_mut(block_type).unwrap())
                                            [counter_idx] += 1;
                                    } else {
                                        // need to own block_type name
                                        region_counts
                                            .insert(block_type.to_string(), vec![0; world_height]);
                                    }
                                }
                            }
                        }
                    }
                }
            });

            tx.send(region_counts).expect("Could not send data!");
        });
    }
    drop(tx);

    // ### REDUCE phase
    let mut final_counts: BTreeMap<String, Vec<isize>> = BTreeMap::new();
    for region_counts in rx.iter() {
        println!(
            "got result for region [{} threads active]",
            pool.active_count()
        );

        for (block_type, count) in region_counts {
            final_counts
                .entry(block_type)
                .and_modify(|t| {
                    // this can fail if not all chunks have the same world height
                    for (i, total) in t.iter_mut().enumerate() {
                        *total += count[i]
                    }
                })
                .or_insert(count);
        }
    }

    for (k, v) in final_counts {
        println!("{} -> {:?}", k, v);
    }
}
