use super::Prepare;

use image::{DynamicImage, GrayImage};

use std::cmp::{max, min};

/// Reduce the image to black/white by calculating local thresholds
///
/// The algorithm runs the following steps:
/// 1. Divide the image into blocks and count the cumulative grayscale value of all pixels in the block
/// 2. For each block of blocks, take mean grayscale value by adding each block's value and dividing by total number of pixels
/// 3. For each pixel in the image, see if the grayscale value of that pixel exceeds the mean of its corresponding block.
///    If so, output a white pixel. If not, output a black pixel
pub struct BlockedMean {
    block_size: BlockSize,
    block_mean_size: BlockSize,
}

impl BlockedMean {
    /// Construct a new BlockedMean
    ///
    /// # Arguments
    ///
    /// * `block_size`: width in pixels of each block
    /// * `block_mean_size`: width in blocks of each block of blocks
    pub fn new(block_size: u32, block_mean_size: u32) -> BlockedMean {
        BlockedMean {
            block_size: BlockSize(block_size),
            block_mean_size: BlockSize(block_mean_size),
        }
    }
}

impl Prepare<DynamicImage, GrayImage> for BlockedMean {
    fn prepare(&self, input: DynamicImage) -> GrayImage {
        let grayscale = input.to_luma();

        let dimensions = grayscale.dimensions();
        let width = ImageCoord(dimensions.0);
        let height = ImageCoord(dimensions.1);

        let block_map = self.as_block_map(&grayscale, width, height);
        let block_mean_map = self.to_block_mean_map(&block_map, width, height);

        self.to_threshold(grayscale, &block_mean_map, width, height)
    }
}

impl BlockedMean {
    fn as_block_map(
        &self,
        grayscale: &GrayImage,
        width: ImageCoord,
        height: ImageCoord,
    ) -> Vec<Stats> {
        let (block_width, block_height) = as_block_coords(width, height, self.block_size);

        let mut blocks = vec![
            Stats {
                total: 0,
                count: 0,
                mean: 0.0
            };
            ((block_width.0 + 1) * (block_height.0 + 1)) as usize
        ];

        for (x, y, p) in grayscale.enumerate_pixels() {
            let coords = as_block_coords(ImageCoord(x), ImageCoord(y), self.block_size);
            let mut stats = &mut blocks[to_index(coords, block_width)];

            stats.total += u64::from(p.data[0]);
            stats.count += 1;
        }

        for mut stat in &mut blocks {
            stat.mean = stat.total as f64 / stat.count as f64;
        }

        blocks
    }

    fn to_block_mean_map(
        &self,
        blocks: &[Stats],
        width: ImageCoord,
        height: ImageCoord,
    ) -> Vec<Stats> {
        let block_stride = BlockCoord((self.block_mean_size.0 - 1) / 2);
        let (block_width, block_height) = as_block_coords(width, height, self.block_size);

        let mut block_means = vec![
            Stats {
                total: 0,
                count: 0,
                mean: 0.0
            };
            ((block_width + BlockCoord(1)) * (block_height + BlockCoord(1))).0
                as usize
        ];

        for block_x in range_inc(BlockCoord(0), block_width) {
            for block_y in range_inc(BlockCoord(0), block_height) {
                let x_start = max(BlockCoord(0), block_x.saturating_sub(block_stride));
                let x_end = min(block_width, block_x + block_stride);
                let y_start = max(BlockCoord(0), block_y.saturating_sub(block_stride));
                let y_end = min(block_height, block_y + block_stride);

                let mut total = 0;
                let mut count = 0;

                for x in range(x_start, x_end) {
                    for y in range(y_start, y_end) {
                        // Make sure to take the pixel counts from each of the blocks directly
                        // Because the size of the image does not have to be an exact multiple of the size in blocks,
                        // some blocks can have differing pixel counts
                        let stats = &blocks[to_index((x, y), block_width)];
                        total += stats.total;
                        count += stats.count;
                    }
                }

                block_means[to_index((block_x, block_y), block_width)].mean =
                    total as f64 / count as f64;
            }
        }

        block_means
    }

    fn to_threshold(
        &self,
        mut grayscale: GrayImage,
        block_means: &[Stats],
        width: ImageCoord,
        height: ImageCoord,
    ) -> GrayImage {
        for (x, y, p) in grayscale.enumerate_pixels_mut() {
            let (block_width, _) = as_block_coords(width, height, self.block_size);
            let coords = as_block_coords(ImageCoord(x), ImageCoord(y), self.block_size);

            let mean = block_means[to_index(coords, block_width)].mean;

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
fn to_index(coords: (BlockCoord, BlockCoord), width: BlockCoord) -> usize {
    ((coords.1).0 * (width.0 + 1) + (coords.0).0) as usize
}

#[inline]
fn as_block_coords(
    x: ImageCoord,
    y: ImageCoord,
    block_size: BlockSize,
) -> (BlockCoord, BlockCoord) {
    let x = x / block_size;
    let y = y / block_size;

    (x, y)
}

// Helper newtypes

#[derive(Copy, Clone, Debug)]
struct ImageCoord(u32);

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)]
struct BlockCoord(u32);
NewtypeAdd! { () struct BlockCoord(u32); }
NewtypeMul! { () struct BlockCoord(u32); }

#[derive(Copy, Clone, Debug)]
struct BlockSize(u32);

use std::ops::{Div, Mul};
impl Mul<BlockSize> for BlockCoord {
    type Output = ImageCoord;

    fn mul(self, other: BlockSize) -> ImageCoord {
        ImageCoord(self.0 * other.0)
    }
}

impl Div<BlockSize> for ImageCoord {
    type Output = BlockCoord;

    fn div(self, other: BlockSize) -> BlockCoord {
        BlockCoord(self.0 / other.0)
    }
}

impl BlockCoord {
    fn saturating_sub(self, other: BlockCoord) -> BlockCoord {
        BlockCoord(self.0.saturating_sub(other.0))
    }
}

struct BlockCoordRangeInclusive {
    current: u32,
    end: u32,
}

impl Iterator for BlockCoordRangeInclusive {
    type Item = BlockCoord;

    fn next(&mut self) -> Option<BlockCoord> {
        if self.current > self.end {
            None
        } else {
            let result = Some(BlockCoord(self.current));
            self.current += 1;
            result
        }
    }
}

fn range(start: BlockCoord, end: BlockCoord) -> BlockCoordRangeInclusive {
    BlockCoordRangeInclusive {
        current: start.0,
        end: end.0 - 1,
    }
}

fn range_inc(start: BlockCoord, end: BlockCoord) -> BlockCoordRangeInclusive {
    BlockCoordRangeInclusive {
        current: start.0,
        end: end.0,
    }
}
