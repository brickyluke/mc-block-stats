use fastanvil::{Chunk, JavaChunk, RegionBuffer};
use log::*;
use std::{cmp, collections::BTreeMap, fs::File, ops::Range, path::PathBuf, sync::mpsc::channel};
use structopt::StructOpt;
use threadpool::ThreadPool;

const IGNORE_BLOCKS: &[&str] = &["minecraft:air", "minecraft:cave_air"];
const CHUNK_FULL: &str = "full";

#[derive(StructOpt, Debug)]
#[structopt(about, author)]
struct Opt {
    /// Minecraft region files (*.mca)
    #[structopt(name = "FILE", required = true, parse(from_os_str))]
    region_files: Vec<PathBuf>,
    /// Silence all output
    #[structopt(short = "q", long = "quiet")]
    quiet: bool,
    /// Verbose mode (-v, -vv, -vvv, etc)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: usize,
    /// Number of concurrent threads; defaults to the number of available CPU cores
    #[structopt(short = "t", long = "threads")]
    threads: Option<usize>,
    /// Expect high worlds; for Minecraft 1.18 and later: -64 <= y < 320
    #[structopt(short = "h", long = "high-worlds")]
    high_worlds: bool,
    /// Process all chunks, including those that haven't been fully populated yet
    #[structopt(short = "a", long = "all-chunks")]
    all_chunks: bool,
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

    // world height
    let world_y_coords = if opt.high_worlds { -64..320 } else { 0..256 };
    info!(
        "Using Y coordinate range from {} to {}.",
        world_y_coords.start, world_y_coords.end
    );

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

    for region_file in opt.region_files {
        let tx = tx.clone();
        let world_y_coords = world_y_coords.clone();

        // process each region file in separate thread
        pool.execute(move || {
            info!("Processing file {}...", region_file.display());
            let file = File::open(region_file).expect("file does not exist");

            let region_counts = gather_region_stats(file, world_y_coords, opt.all_chunks);

            tx.send(region_counts).expect("Could not send data!");
        });
    }
    drop(tx);

    // REDUCE: process results coming in from each region file
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
                    for (i, total) in t.iter_mut().enumerate() {
                        *total += count[i]
                    }
                })
                .or_insert(count);
        }
    }

    // print CSV header
    print!("block_type");
    for y in world_y_coords {
        print!(",y_{}", y);
    }
    println!();
    // print CSV rows
    for (k, v) in final_counts {
        print!("\"{}\"", k);
        for count in v {
            print!(",{}", count);
        }
        println!();
    }
}

fn gather_region_stats(
    file: File,
    world_y_coords: Range<isize>,
    full_chunks_only: bool,
) -> BTreeMap<String, Vec<isize>> {
    let world_height = usize::try_from(world_y_coords.end - world_y_coords.start).unwrap();
    debug!("World height={}", world_height);

    let mut region = RegionBuffer::new(file);
    let mut region_counts: BTreeMap<String, Vec<isize>> = BTreeMap::new();
    let mut max_y_range = 0..0;

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
        if chunk.status() == CHUNK_FULL || full_chunks_only {
            let y_range = range_intersect(&world_y_coords, &chunk.y_range());
            // TODO: use maximum y_range to further limit dimensions of result set
            max_y_range = range_union(&y_range, &max_y_range);
            for chunk_y in y_range {
                let counter_idx = usize::try_from(chunk_y - world_y_coords.start).unwrap();
                for chunk_x in 0..16 {
                    for chunk_z in 0..16 {
                        let block_type = match chunk.block(chunk_x, chunk_y, chunk_z) {
                            Some(block) => block.name(),
                            None => continue,
                        };
                        // skip blocks we're not interested in
                        if !IGNORE_BLOCKS.iter().any(|&i| i == block_type) {
                            // can't use the entry API without changing ownership, which is expensive
                            if !region_counts.contains_key(block_type) {
                                region_counts.insert(block_type.to_string(), vec![0; world_height]);
                            }
                            (*region_counts.get_mut(block_type).unwrap())[counter_idx] += 1;
                        }
                    }
                }
            }
        }
    });
    region_counts
}

fn range_intersect(r1: &Range<isize>, r2: &Range<isize>) -> Range<isize> {
    let min = cmp::max(r1.start, r2.start);
    let max = cmp::min(r1.end, r2.end);
    min..max
}

fn range_union(r1: &Range<isize>, r2: &Range<isize>) -> Range<isize> {
    let min = cmp::min(r1.start, r2.start);
    let max = cmp::max(r1.end, r2.end);
    min..max
}
