use super::Prepare;

use image::{DynamicImage, GrayImage};

use std::cmp::{max, min};

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

impl Prepare<DynamicImage, GrayImage> for BlockedMean {
    fn prepare(&self, input: DynamicImage) -> GrayImage {
        let grayscale = input.to_luma();

        let dimensions = grayscale.dimensions();
        let width = dimensions.0;
        let height = dimensions.1;

        let block_map = self.as_block_map(&grayscale, width, height);
        let block_mean_map = self.to_block_mean_map(&block_map, width, height);

        self.to_threshold(grayscale, &block_mean_map, width, height)
    }
}

impl BlockedMean {
    fn as_block_map(&self, grayscale: &GrayImage, width: u32, height: u32) -> Vec<Stats> {
        let (block_width, block_height) = as_block_coords(width, height, self.block_size);

        let mut blocks = vec![
            Stats {
                total: 0,
                count: 0,
                mean: 0.0
            };
            ((block_width + 1) * (block_height + 1)) as usize
        ];

        for (x, y, p) in grayscale.enumerate_pixels() {
            let coords = as_block_coords(x, y, self.block_size);
            let idx: usize = (coords.1 * (block_width + 1) + coords.0) as usize;

            let mut stats = &mut blocks[idx];

            stats.total += u64::from(p.data[0]);
            stats.count += 1;
        }

        for mut stat in &mut blocks {
            stat.mean = stat.total as f64 / stat.count as f64;
        }

        blocks
    }

    fn to_block_mean_map(&self, blocks: &[Stats], width: u32, height: u32) -> Vec<Stats> {
        let block_stride = (self.block_mean_size - 1) / 2;
        let (block_width, block_height) = as_block_coords(width, height, self.block_size);

        let mut block_means = vec![
            Stats {
                total: 0,
                count: 0,
                mean: 0.0
            };
            ((block_width + 1) * (block_height + 1)) as usize
        ];

        for block_x in 0..block_width + 1 {
            for block_y in 0..block_height + 1 {
                let x_start = max(0, block_x.saturating_sub(block_stride));
                let x_end = min(block_width, block_x + block_stride);
                let y_start = max(0, block_y.saturating_sub(block_stride));
                let y_end = min(block_height, block_y + block_stride);

                let mut total = 0;
                let mut count = 0;

                for x in x_start..x_end {
                    for y in y_start..y_end {
                        let idx: usize = (y * (block_width + 1) + x) as usize;
                        let mut stats = &blocks[idx];
                        total += stats.total;
                        count += stats.count;
                    }
                }

                let idx: usize = (block_y * (block_width + 1) + block_x) as usize;
                block_means[idx].mean = total as f64 / count as f64;
            }
        }

        block_means
    }

    fn to_threshold(
        &self,
        mut grayscale: GrayImage,
        block_means: &[Stats],
        width: u32,
        height: u32,
    ) -> GrayImage {
        for (x, y, p) in grayscale.enumerate_pixels_mut() {
            let (block_width, _) = as_block_coords(width, height, self.block_size);

            let coords = as_block_coords(x, y, self.block_size);
            let idx: usize = (coords.1 * (block_width + 1) + coords.0) as usize;

            let mean = block_means[idx].mean;

            p.data[0] = if f64::from(p.data[0]) > mean { 255 } else { 0 };
        }

        grayscale
    }
}

#[derive(Debug, Copy, Clone)]
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
