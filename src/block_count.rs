use core::fmt;
use std::collections::HashMap;
use std::ops::Range;

use BlockCountError::*;

pub struct BlockCount {
    count: HashMap<String, Vec<isize>>,
    world_y_range: Range<isize>,
    capacity: usize,
}

impl BlockCount {
    pub fn new(world_y_range: &Range<isize>) -> BlockCount {
        BlockCount {
            count: HashMap::new(),
            world_y_range: world_y_range.clone(),
            capacity: usize::try_from(world_y_range.end - world_y_range.start).unwrap(),
        }
    }

    pub fn count_block(&mut self, y_coord: isize, block_type: &str) {
        let counter_idx = usize::try_from(y_coord - self.world_y_range.start).unwrap();
        if let Some(counter) = self.count.get_mut(block_type) {
            // this case is much much more common
            counter[counter_idx] += 1;
        } else {
            let mut counter = vec![0isize; self.capacity];
            counter[counter_idx] = 1;
            self.count.insert(block_type.to_string(), counter);
        }
    }

    pub fn add_block_count(&mut self, other: BlockCount) -> Result<(), BlockCountError> {
        if self.world_y_range != other.world_y_range {
            return Err(MismatchingYRange {
                this: self.world_y_range.clone(),
                other: other.world_y_range,
            });
        }
        for (block_type, other_count) in other.count {
            match self.count.get_mut(&block_type) {
                Some(this_count) => {
                    // use zip for efficient iteration without indexing
                    for (this, other) in this_count.iter_mut().zip(other_count.iter()) {
                        *this += other;
                    }
                }
                None => {
                    self.count.insert(block_type, other_count);
                }
            }
        }
        Ok(())
    }

    pub fn world_y_range(&self) -> Range<isize> {
        self.world_y_range.clone()
    }

    pub fn block_count(&self) -> &HashMap<String, Vec<isize>> {
        &self.count
    }
}

#[derive(PartialEq, Debug)]
pub enum BlockCountError {
    MismatchingYRange {
        this: Range<isize>,
        other: Range<isize>,
    },
}

impl fmt::Display for BlockCountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            MismatchingYRange { this, other } => write!(
                f,
                "Y ranges don't match: expected {}..{}, but got {}..{}",
                this.start, this.end, other.start, other.end
            ),
        }
    }
}
