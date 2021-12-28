use fastanvil::{Chunk, JavaChunk, RegionBuffer};
use log::*;
use std::{collections::BTreeMap, fs::File, sync::mpsc::channel};
use structopt::StructOpt;
use threadpool::ThreadPool;

const IGNORE_BLOCKS: &[&str] = &["minecraft:air", "minecraft:cave_air"];
const CHUNK_FULL: &str = "full";

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    /// Silence all output
    #[structopt(short = "q", long = "quiet")]
    quiet: bool,
    /// Verbose mode (-v, -vv, -vvv, etc)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: usize,
    /// Minecraft region file (*.mca)
    #[structopt(short = "f", long = "region-file")]
    region_file: String,
    /// Number of concurrent threads (defaults to number of CPU cores)
    #[structopt(short = "t", long = "threads")]
    threads: Option<usize>,
}

fn main() {
    // parse CLI arguments
    let opt = Opt::from_args();

    // initialise logger
    stderrlog::new()
        .module(module_path!())
        .quiet(opt.quiet)
        .verbosity(opt.verbose + 1)
        .timestamp(stderrlog::Timestamp::Millisecond)
        .init()
        .unwrap();

    // initialise thread pool
    let threads = opt.threads.unwrap_or_default();
    let pool_size = if threads == 0 {
        num_cpus::get()
    } else {
        threads
    };
    info!("Using up to {} threads.", pool_size);
    let pool = ThreadPool::new(pool_size);

    let (tx, rx) = channel();

    // process Minecraft region files
    {
        // TODO: loop over region files
        let tx = tx.clone();

        // process each region file in separate thread
        pool.execute(move || {
            let file_path = opt.region_file;

            info!("Processing file {}...", file_path);
            let file = File::open(file_path).expect("file does not exist");

            let region_counts = gather_region_stats(file);

            tx.send(region_counts).expect("Could not send data!");
        });
    }
    drop(tx);

    // process results coming in from each region file
    let mut final_counts: BTreeMap<String, Vec<isize>> = BTreeMap::new();
    for region_counts in rx.iter() {
        info!(
            "Got result for region [{} threads active]",
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

fn gather_region_stats(file: File) -> BTreeMap<String, Vec<isize>> {
    let mut region = RegionBuffer::new(file);
    let mut region_counts: BTreeMap<String, Vec<isize>> = BTreeMap::new();

    // process all chunks in region file sequentially
    let _ = region.for_each_chunk(|x, z, data| {
        let chunk: JavaChunk = fastnbt::de::from_bytes(data.as_slice()).unwrap();

        debug!(
            "Processing Chunk( x={}, z={}, status={} ) -> y = [{}..{}]",
            x,
            z,
            &chunk.status(),
            chunk.y_range().start,
            chunk.y_range().end
        );

        // skip incomplete chunks
        if chunk.status() == CHUNK_FULL {
            // let world_height = usize::try_from(chunk.y_range().end - chunk.y_range().start).unwrap();
            let world_height = 256; // hard-coded for 1.17 -> will break things for 1.18!
            let height_offset = 0 - chunk.y_range().start;

            for chunk_y in chunk.y_range() {
                let counter_idx = usize::try_from(height_offset + chunk_y).unwrap();
                for chunk_x in 0..16 {
                    for chunk_z in 0..16 {
                        let block_type = chunk.block(chunk_x, chunk_y, chunk_z).unwrap().name();
                        // skip blocks we're not interested in
                        if !IGNORE_BLOCKS.iter().any(|&i| i == block_type) {
                            // can't use the entry API without changing ownership, which is expensive
                            if region_counts.contains_key(block_type) {
                                (*region_counts.get_mut(block_type).unwrap())[counter_idx] += 1;
                            } else {
                                // need to own block_type name
                                region_counts.insert(block_type.to_string(), vec![0; world_height]);
                            }
                        }
                    }
                }
            }
        }
    });
    region_counts
}
