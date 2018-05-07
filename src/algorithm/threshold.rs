use image::GrayImage;

use std::cmp::{max, min};
use std::collections::HashMap;

pub trait Threshold<G, T> {
    fn to_threshold(&self, grayscale: G) -> T;
}

pub struct BlockedMean {
    block_size: u32,
    block_mean_size: u32,
}

impl BlockedMean {
    pub fn new(block_size: u32, block_mean_size: u32) -> BlockedMean {
        BlockedMean {
            block_size,
            block_mean_size,
        }
    }
}

impl Threshold<GrayImage, GrayImage> for BlockedMean {
    fn to_threshold(&self, grayscale: GrayImage) -> GrayImage {
        let dimensions = grayscale.dimensions();
        let width = dimensions.0;
        let height = dimensions.1;

        let block_map = self.as_block_map(&grayscale, width, height);
        let block_mean_map = self.to_block_mean_map(block_map, width, height);

        self.to_threshold(grayscale, block_mean_map)
    }
}

impl BlockedMean {
    fn as_block_map(
        &self,
        grayscale: &GrayImage,
        width: u32,
        height: u32,
    ) -> HashMap<(u32, u32), Stats> {
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

        block_map
    }

    fn to_block_mean_map(
        &self,
        block_map: HashMap<(u32, u32), Stats>,
        width: u32,
        height: u32,
    ) -> HashMap<(u32, u32), Stats> {
        let mut block_mean_map: HashMap<(u32, u32), Stats> =
            HashMap::with_capacity((width * height / self.block_size) as usize);

        let block_stride = (self.block_mean_size - 1) / 2;
        let (block_width, block_height) = as_block_coords(width, height, self.block_size);

        for coords in block_map.keys() {
            let x_start = max(0, coords.0.saturating_sub(block_stride));
            let x_end = min(block_width, coords.0 + block_stride);
            let y_start = max(0, coords.1.saturating_sub(block_stride));
            let y_end = min(block_height, coords.1 + block_stride);

            let mut total = 0;
            let mut count = 0;

            for x in x_start..x_end {
                for y in y_start..y_end {
                    let stats = block_map.get(&(x, y)).unwrap();
                    total += stats.total;
                    count += stats.count;
                }
            }

            block_mean_map.insert(
                (coords.0, coords.1),
                Stats {
                    total,
                    count,
                    mean: total as f64 / count as f64,
                },
            );
        }

        block_mean_map
    }

    fn to_threshold(
        &self,
        mut grayscale: GrayImage,
        block_mean_map: HashMap<(u32, u32), Stats>,
    ) -> GrayImage {
        for (x, y, p) in grayscale.enumerate_pixels_mut() {
            let coords = as_block_coords(x, y, self.block_size);
            let mean = block_mean_map.get(&coords).unwrap().mean;

            p.data[0] = if p.data[0] as f64 > mean { 255 } else { 0 };
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
