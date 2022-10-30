use std::{error::Error, fs::File, ops::Range, path::PathBuf, sync::mpsc::channel};

use fastanvil::{Chunk, JavaChunk, Region};
use log::*;
use structopt::StructOpt;
use threadpool::ThreadPool;

use block_count::BlockCount;

mod block_count;

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
    /// Expect high worlds; use this for Minecraft 1.18 and later: -64 <= y < 320
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

            match gather_region_stats(region_file, world_y_coords, opt.all_chunks) {
                Ok(region_counts) => tx.send(region_counts).expect("Couldn't not send data!"),
                Err(e) => error!("Couldn't process region file: {}", e),
            };
        });
    }
    drop(tx);

    // REDUCE: process results coming in from each region file
    let mut final_counts = BlockCount::new(&world_y_coords);
    for region_counts in rx.iter() {
        debug!("Got result for region");
        match final_counts.add_block_count(region_counts) {
            Ok(()) => info!(
                "Added counts for region. [{} threads active]",
                pool.active_count()
            ),
            Err(e) => error!("Couldn't add counts for region! {}", e),
        }
    }

    // print CSV header
    print!("block_type");
    for y in final_counts.world_y_range() {
        print!(",y_{}", y);
    }
    println!();
    // print CSV rows
    for (k, v) in final_counts.block_count() {
        print!("\"{}\"", k);
        for count in v {
            print!(",{}", count);
        }
        println!();
    }
}

fn gather_region_stats(
    region_file: PathBuf,
    world_y_coords: Range<isize>,
    process_all_chunks: bool,
) -> Result<BlockCount, Box<dyn Error>> {
    let world_height = usize::try_from(world_y_coords.end - world_y_coords.start).unwrap();
    debug!("World height={}", world_height);

    let file = File::open(region_file)?;
    let mut region = Region::from_stream(file).unwrap();
    let mut region_counts = BlockCount::new(&world_y_coords);

    // process chunks in region file sequentially
    for z in 0..32 {
        for x in 0..32 {
            match region.read_chunk(x, z) {
                Ok(Some(data)) => {
                    let chunk = JavaChunk::from_bytes(&data).unwrap();

                    debug!(
                        "Processing Chunk( x={}, z={}, status={} ) -> y = [{}..{}]",
                        x,
                        z,
                        &chunk.status(),
                        chunk.y_range().start,
                        chunk.y_range().end
                    );

                    // skip incomplete chunks
                    if process_all_chunks || chunk.status() == CHUNK_FULL {
                        // only process Y coordinates that are within range
                        let y_range = range_intersect(&world_y_coords, &chunk.y_range());
                        for chunk_y in y_range {
                            for chunk_x in 0..16 {
                                for chunk_z in 0..16 {
                                    let block_type = match chunk.block(chunk_x, chunk_y, chunk_z) {
                                        Some(block) => block.name(),
                                        None => continue,
                                    };
                                    // skip blocks that we're not interested in
                                    if !IGNORE_BLOCKS.iter().any(|&i| i == block_type) {
                                        region_counts.count_block(chunk_y, block_type);
                                    }
                                }
                            }
                        }
                    }
                }
                Ok(None) => {}
                Err(e) => return Err(e.into()),
            }
        }
    }
    Ok(region_counts)
}

fn range_intersect(r1: &Range<isize>, r2: &Range<isize>) -> Range<isize> {
    let min = if r1.start > r2.start {
        r1.start
    } else {
        r2.start
    };
    let max = if r1.end < r2.end { r1.end } else { r2.end };
    min..max
}
