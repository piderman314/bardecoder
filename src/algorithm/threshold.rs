use image::GrayImage;

use std::collections::HashMap;

pub trait Threshold<G, T> {
    fn to_threshold(&self, grayscale: G) -> T;
}

pub struct BlockedMean {
    block_size: u32,
}

impl BlockedMean {
    pub fn new(block_size: u32) -> BlockedMean {
        BlockedMean { block_size }
    }
}

impl Threshold<GrayImage, GrayImage> for BlockedMean {
    fn to_threshold(&self, grayscale: GrayImage) -> GrayImage {
        let dimensions = grayscale.dimensions();
        let width = dimensions.0;
        let height = dimensions.1;

        let mut block_map: HashMap<(u32, u32), Stats> =
            HashMap::with_capacity((width * height / self.block_size) as usize);

        for (x, y, p) in grayscale.enumerate_pixels() {
            let block_coords = as_block_coords(x, y, self.block_size);
            block_map
                .entry(block_coords)
                .and_modify(|s| {
                    s.total += p.data[0] as u64;
                    s.count += 1
                })
                .or_insert(Stats {
                    total: p.data[0] as u64,
                    count: 1,
                    mean: 0.0,
                });
        }

        for stat in block_map.values_mut() {
            stat.mean = stat.total as f64 / stat.count as f64;
        }

        grayscale
    }
}

struct Stats {
    total: u64,
    count: u64,
    mean: f64,
}

#[inline]
fn as_block_coords(x: u32, y: u32, block_size: u32) -> (u32, u32) {
    let x = x / block_size;
    let y = y / block_size;

    (x, y)
}
