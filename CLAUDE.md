# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`mc-block-stats` is a CLI tool that reads Minecraft region files (`.mca`) and outputs CSV statistics counting each block type at every Y coordinate level. Output goes to stdout; progress/errors go to stderr.

## Commands

```bash
cargo build --release          # build optimised binary
cargo check                    # fast type-check without linking
cargo clippy                   # lint
cargo fmt                      # format
cargo install --path .         # install binary to ~/.cargo/bin

# Run (after install or using cargo run --)
mc-block-stats [OPTIONS] <FILE>...
mc-block-stats -h r.0.0.mca    # 1.18+ high world
mc-block-stats -vv *.mca       # debug verbosity
```

No test suite exists in the project.

## Architecture

Two source files:

- **`src/main.rs`** — CLI parsing (`structopt`), logging init, Rayon thread pool, parallel region processing, CSV output.
- **`src/block_count.rs`** — `BlockCount` struct: a `HashMap<String, Vec<isize>>` mapping block name → per-Y-level count vector. Supports `count_block()` and `add_block_count()` (merge).

### Data flow

```
CLI args → world Y range + file list
  → Rayon par_iter over region files
      → for each region: sequential 32×32 chunk loop (fastanvil requires sequential I/O)
          → skip non-"full" chunks (unless --all-chunks)
          → skip air blocks
          → BlockCount per region
  → Rayon reduce: merge all BlockCounts via add_block_count()
→ CSV to stdout (rows sorted by block name, columns = Y levels)
```

### Key design decisions

- **Region files are parallel; chunks within a region are sequential** — fastanvil's `Region` type requires sequential access.
- **Y-coordinate versions**: pre-1.18 worlds use Y 0–255; 1.18+ worlds use Y −64–319. The `--high-worlds` (`-h`) flag switches the range. Stored as `Range<isize>`; array index = `y - range.start`.
- **`range_intersect()`** handles chunks whose own Y range is smaller than the world range (common at world edges).
- **Air blocks are always excluded** via `IGNORE_BLOCKS` constant — avoids inflating stats with the overwhelmingly common empty block.
- **Sorting happens only at output** (not during accumulation) — an earlier performance fix; don't reintroduce sorting in the hot path.
- **Error handling**: region/chunk errors are logged as warnings and skipped; processing always continues on partial data.

## Dependencies

| Crate | Purpose |
|---|---|
| `fastanvil` | Parse `.mca` region and chunk files |
| `fastnbt` | NBT format (used internally by fastanvil) |
| `rayon` | Data-parallel iteration |
| `structopt` | CLI argument parsing |
| `log` + `stderrlog` | Levelled logging to stderr |
