use std::{fs::File, io::Error, ops::Range, path::PathBuf, sync::mpsc::channel};

use fastanvil::{Chunk, JavaChunk, RegionBuffer};
use log::*;
use structopt::StructOpt;
use threadpool::ThreadPool;

use crate::stats::BlockCounts;

const IGNORE_BLOCKS: &[&str] = &["minecraft:air", "minecraft:cave_air"];
const CHUNK_FULL: &str = "full";

mod stats {
    use core::fmt;
    use std::collections::BTreeMap;
    use std::ops::Range;

    pub struct BlockCounts {
        counts: BTreeMap<String, Vec<isize>>,
        world_y_range: Range<isize>,
        capacity: usize,
    }

    impl BlockCounts {
        pub fn new(world_y_range: &Range<isize>) -> BlockCounts {
            BlockCounts {
                counts: BTreeMap::new(),
                world_y_range: world_y_range.clone(),
                capacity: usize::try_from(world_y_range.end - world_y_range.start).unwrap(),
            }
        }

        pub fn count_block(&mut self, y_coord: isize, block_type: &str) {
            let counter_idx = usize::try_from(y_coord - self.world_y_range.start).unwrap();
            if !self.counts.contains_key(block_type) {
                self.counts
                    .insert(block_type.to_string(), vec![0; self.capacity]);
            }
            (*self.counts.get_mut(block_type).unwrap())[counter_idx] += 1;
        }

        pub fn add_block_counts(&mut self, other: BlockCounts) -> Result<(), MismatchingYRange> {
            if self.world_y_range != other.world_y_range {
                return Err(MismatchingYRange);
            }
            for (block_type, other_counts) in other.counts {
                self.counts
                    .entry(block_type)
                    .and_modify(|my_block_counts| {
                        for (i, my_block_count) in my_block_counts.iter_mut().enumerate() {
                            *my_block_count += other_counts[i]
                        }
                    })
                    .or_insert(other_counts);
            }
            Ok(())
        }

        pub fn world_y_range(&self) -> Range<isize> {
            self.world_y_range.clone()
        }

        pub fn block_counts(&self) -> &BTreeMap<String, Vec<isize>> {
            &self.counts
        }
    }

    #[derive(Debug, Clone)]
    pub struct MismatchingYRange;

    impl fmt::Display for MismatchingYRange {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Y ranges don't match")
        }
    }
}

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

            match gather_region_stats(region_file, world_y_coords, opt.all_chunks) {
                Ok(region_counts) => tx.send(region_counts).expect("Couldn't not send data!"),
                Err(e) => error!("Couldn't process region file: {}", e),
            };
        });
    }
    drop(tx);

    // REDUCE: process results coming in from each region file
    let mut final_counts = BlockCounts::new(&world_y_coords);
    for region_counts in rx.iter() {
        debug!("Got result for region");
        match final_counts.add_block_counts(region_counts) {
            Ok(()) => info!(
                "Added counts for region. [{} threads active]",
                pool.active_count()
            ),
            Err(e) => error!("Couldn't add counts for region: {}", e),
        }
    }

    // print CSV header
    print!("block_type");
    for y in final_counts.world_y_range() {
        print!(",y_{}", y);
    }
    println!();
    // print CSV rows
    for (k, v) in final_counts.block_counts() {
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
) -> Result<BlockCounts, Error> {
    let world_height = usize::try_from(world_y_coords.end - world_y_coords.start).unwrap();
    debug!("World height={}", world_height);

    let file = File::open(region_file)?;
    let mut region = RegionBuffer::new(file);
    let mut region_counts = BlockCounts::new(&world_y_coords);

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
    });
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
