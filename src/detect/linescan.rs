use super::{Detect, Location};

use std::cmp::min;
use std::iter::repeat;
use std::iter::Iterator;
use std::sync::{Arc, Mutex};

use crate::util::qr::QRLocation;
use crate::util::Point;

use image::{GrayImage, Pixel};

#[cfg(feature = "debug-images")]
use image::{DynamicImage, Rgb};

#[cfg(feature = "debug-images")]
use std::{env::temp_dir, fs::create_dir_all};

/// Scan a prepared image for QR Codes
///
/// The general idea of this method is as follows:
/// 1. Scan line by line horizontally for possible QR Finder patterns (the three squares)
/// 2. If a possible pattern is found, check vertically and diagonally to confirm it is indeed a pattern
/// 3. Try to find combinations of three patterns that are perpendicular and with similar distance that form a complete QR Code
pub struct LineScan {}

impl LineScan {
    /// Constuct a new LineScan
    pub fn new() -> LineScan {
        LineScan {}
    }
}

type Refine = dyn Fn(&GrayImage, &Point, f64) -> Option<QRFinderPosition>;

struct LineProcessingClosure<'a> {
    prepared: &'a GrayImage,
    y: u32,
}

impl<'a> LineProcessingClosure<'a> {
    fn process(&self, data: Arc<Mutex<Vec<QRFinderPosition>>>) {
        // The order of refinement is important.
        // The candidate is found in horizontal direction, so the first refinement is vertical
        let refine_func: Vec<(Box<Refine>, f64, f64, bool)> = vec![
            (Box::new(LineScan::refine_vertical), 0.0, 1.0, false),
            (Box::new(LineScan::refine_horizontal), 1.0, 0.0, false),
            (Box::new(LineScan::refine_diagonal), 1.0, 1.0, true),
        ];

        let prepared = self.prepared;
        let mut candidates = data.lock().unwrap();
        let mut last_pixel = 127;
        let mut pattern = QRFinderPattern::new();
        'pixels: for x in 0..prepared.width() {
            // A pixel of the same color, add to the count in the last position
            let p = prepared.get_pixel(x, self.y);
            if p.channels()[0] == last_pixel {
                pattern.6 += 1;

                if x != prepared.dimensions().0 - 1 {
                    continue 'pixels;
                }
            }

            // A pixel color switch, but the current pattern does not look like a finder
            // Slide the pattern and continue searching
            if !pattern.looks_like_finder() {
                last_pixel = p.channels()[0];
                pattern.slide();
                continue 'pixels;
            }

            let mut module_size = pattern.est_mod_size();

            // A finder pattern is 1-1-3-1-1 modules wide, so subtract 3.5 modules to get the x coordinate in the center
            let mut finder = Point {
                x: f64::from(x) - module_size * 3.5,
                y: f64::from(self.y),
            };

            for candidate in candidates.iter() {
                if dist(&finder, &candidate.location) < 7.0 * module_size {
                    // The candidate location we have found was already detected and stored on a previous line.
                    last_pixel = p.channels()[0];
                    pattern.slide();

                    continue 'pixels;
                }
            }

            // Step 2
            // Run the refinement functions on the candidate location
            for (refine_func, dx, dy, is_diagonal) in &refine_func {
                let vert = refine_func(prepared, &finder, module_size);

                if vert.is_none() {
                    last_pixel = p.channels()[0];
                    pattern.slide();

                    continue 'pixels;
                }

                if !is_diagonal {
                    // Adjust the candidate location with the refined candidate and module size,
                    // exchept when refining the diagonal because that is unreliable on lower resolutions
                    let vert = vert.unwrap();
                    let half_finder = 3.5 * vert.last_module_size;
                    finder.x = vert.location.x - dx * half_finder;
                    finder.y = vert.location.y - dy * half_finder;
                    module_size = vert.module_size;
                }
            }

            candidates.push(QRFinderPosition {
                location: finder,
                module_size,
                last_module_size: 0.0,
            });

            last_pixel = p.channels()[0];
            pattern.slide();
        }
    }
}

impl Detect<GrayImage> for LineScan {
    fn detect(&self, prepared: &GrayImage) -> Vec<Location> {
        let candidates_thread_safe = Arc::new(Mutex::<Vec<QRFinderPosition>>::new(vec![]));

        // Start a new scope so the threads complete at the end.
        {
            #[cfg(feature = "multithreaded")]
            let num_threads = 4;
            #[cfg(feature = "multithreaded")]
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(num_threads)
                .build()
                .unwrap();

            // Loop over each row.
            for y in 0..prepared.height() {
                let candidates_reference = Arc::clone(&candidates_thread_safe);
                let process_line = LineProcessingClosure { prepared, y };
                #[cfg(feature = "multithreaded")]
                {
                    pool.install(move || {
                        process_line.process(candidates_reference);
                    });
                }
                #[cfg(not(feature = "multithreaded"))]
                {
                    process_line.process(candidates_reference);
                }
            }
        }

        let data = Arc::clone(&candidates_thread_safe);
        let candidates = data.lock().unwrap();
        debug!("Candidate QR Locators {:#?}", candidates);

        // Output a debug image by drawing red squares around all candidate locations
        #[cfg(feature = "debug-images")]
        {
            #[cfg(feature = "debug-images")]
            let mut img = DynamicImage::ImageLuma8(prepared.clone()).to_rgb();

            for c in candidates.iter() {
                let loc = c.location;
                let x_start = (loc.x - 3.5 * c.module_size).max(0.0_f64) as u32;
                let x_end = min(img.dimensions().0, (loc.x + 3.5 * c.module_size) as u32);
                let y_start = (loc.y - 3.5 * c.module_size).max(0.0_f64) as u32;
                let y_end = min(img.dimensions().0, (loc.y + 3.5 * c.module_size) as u32);

                for x in x_start..x_end {
                    for y in y_start..y_end {
                        if x > x_start + 3 && x < x_end - 3 && y > y_start + 3 && y < y_end - 3 {
                            continue;
                        }

                        img.put_pixel(x, y, Rgb([255, 0, 0]));
                    }
                }
            }

            let mut tmp = temp_dir();
            tmp.push("bardecoder-debug-images");

            if let Ok(_) = create_dir_all(tmp.clone()) {
                tmp.push("candidates.png");

                if let Ok(_) = DynamicImage::ImageRgb8(img).save(tmp.clone()) {
                    debug!("Debug image with locator candidates saved to {:?}", tmp);
                }
            }
        }

        let mut locations: Vec<Location> = vec![];

        let max_candidates = candidates.len();

        // Step 3
        // Loop through all candidates to see if any combination results in an actual QR
        for candidate1 in 0..max_candidates {
            for candidate2 in candidate1 + 1..max_candidates {
                let diff1 = diff(
                    candidates[candidate1].module_size,
                    candidates[candidate2].module_size,
                );

                trace!("DIFF 1 {}", diff1);

                if diff1 > 0.1 {
                    continue;
                }

                for candidate3 in candidate2 + 1..max_candidates {
                    let diff2 = diff(
                        candidates[candidate1].module_size,
                        candidates[candidate3].module_size,
                    );

                    trace!("DIFF 2 {}", diff2);

                    if diff2 > 0.1 {
                        continue;
                    }

                    if let Some(qr) = find_qr(
                        &candidates[candidate1].location,
                        &candidates[candidate2].location,
                        &candidates[candidate3].location,
                        candidates[candidate1].module_size,
                    ) {
                        locations.push(Location::QR(qr));
                    }
                }
            }
        }

        locations
    }
}

impl LineScan {
    // Refine horizontally
    fn refine_horizontal(
        prepared: &GrayImage,
        finder: &Point,
        module_size: f64,
    ) -> Option<QRFinderPosition> {
        // Bound x range to image dimensions
        let start_x = (finder.x - 5.0 * module_size).max(0.0_f64).round() as u32;
        let end_x = min(
            (finder.x + 5.0 * module_size).round() as u32,
            prepared.dimensions().0,
        );

        // Range in x direction, y is constant
        let range_x = start_x..end_x;
        let range_y = repeat(finder.y.round() as u32);

        Self::refine(prepared, module_size, range_x, range_y, false)
    }

    // Refine vertically
    fn refine_vertical(
        prepared: &GrayImage,
        finder: &Point,
        module_size: f64,
    ) -> Option<QRFinderPosition> {
        // Bound y range to image dimensions
        let start_y = (finder.y - 5.0 * module_size).max(0.0_f64).round() as u32;
        let end_y = min(
            (finder.y + 5.0 * module_size).round() as u32,
            prepared.dimensions().1,
        );

        // X is constant, range in y direction
        let range_x = repeat(finder.x.round() as u32);
        let range_y = start_y..end_y;

        Self::refine(prepared, module_size, range_x, range_y, false)
    }

    // Refine diagonally
    fn refine_diagonal(
        prepared: &GrayImage,
        finder: &Point,
        module_size: f64,
    ) -> Option<QRFinderPosition> {
        let side = 5.0 * module_size;
        let mut start_x = 0.0;
        let mut start_y = 0.0;

        // Bound both x and y ranges to image dimensions
        // Make sure not to do it independently so that the ranges keep being diagonal
        if finder.x < side && finder.y < side {
            if finder.x < finder.y {
                start_y = finder.y - finder.x;
            } else {
                start_x = finder.x - finder.y;
            }
        } else if finder.x < side {
            start_y = finder.y - finder.x;
        } else if finder.y < side {
            start_x = finder.x - finder.y;
        } else {
            start_x = finder.x - side;
            start_y = finder.y - side;
        }

        // Ranges in both x and y directions
        let range_x = start_x.round() as u32
            ..min(
                (finder.x + 5.0 * module_size).round() as u32,
                prepared.dimensions().0,
            );
        let range_y = start_y.round() as u32
            ..min(
                (finder.y + 5.0 * module_size).round() as u32,
                prepared.dimensions().1,
            );

        Self::refine(prepared, module_size, range_x, range_y, true)
    }

    fn refine(
        prepared: &GrayImage,
        module_size: f64,
        range_x: impl Iterator<Item = u32>,
        range_y: impl Iterator<Item = u32>,
        is_diagonal: bool,
    ) -> Option<QRFinderPosition> {
        let mut last_pixel = 127;
        let mut pattern = QRFinderPattern::new();
        let mut last_x = 0;
        let mut last_y = 0;

        // Loop over provided range and basically execute the same logic as above
        for (x, y) in range_x.zip(range_y) {
            let p = prepared.get_pixel(x, y)[0];
            if p == last_pixel {
                pattern.6 += 1;
            } else {
                // The current pattern needs to look like a finder (1-1-3-1-1)
                // Also the module size needs to be similar to the candidate we are refining,
                // except when checking the diagonal because that is unreliable on lower resolutions
                if pattern.looks_like_finder()
                    && (diff(module_size, pattern.est_mod_size()) < 0.2 || is_diagonal)
                {
                    let new_est_mod_size = (module_size + pattern.est_mod_size()) / 2.0;
                    return Some(QRFinderPosition {
                        location: Point {
                            x: f64::from(x),
                            y: f64::from(y),
                        },
                        module_size: new_est_mod_size,
                        last_module_size: pattern.est_mod_size(),
                    });
                }

                last_pixel = p;
                pattern.slide();
            }

            last_x = x;
            last_y = y;
        }

        // The current pattern needs to look like a finder (1-1-3-1-1)
        // Also the module size needs to be similar to the candidate we are refining,
        // except when checking the diagonal because that is unreliable on lower resolutions
        if pattern.looks_like_finder()
            && (diff(module_size, pattern.est_mod_size()) < 0.2 || is_diagonal)
        {
            let new_est_mod_size = (module_size + pattern.est_mod_size()) / 2.0;
            return Some(QRFinderPosition {
                location: Point {
                    x: f64::from(last_x),
                    y: f64::from(last_y),
                },
                module_size: new_est_mod_size,
                last_module_size: pattern.est_mod_size(),
            });
        }

        None
    }
}

#[derive(Debug)]
struct QRFinderPattern(u32, u32, u32, u32, u32, u32, u32);

impl QRFinderPattern {
    fn new() -> QRFinderPattern {
        QRFinderPattern(0, 0, 0, 0, 0, 0, 0)
    }

    fn slide(&mut self) {
        if f64::from(self.6) < f64::from(self.5) / 10.0 && self.4 != 0 {
            // we slid last time because the pixels inverted,
            // but it turned out that it was only for a few pixels
            // likely it was just some noise in the image
            // so revert the previous slide call and add the noise to the previous pattern
            //
            // Only ignore this as noise if this isn't the first shift, since we might just have a
            // large quiet zone (self.4 == 0).
            self.6 += self.5;
            self.5 = self.4;
            self.4 = self.3;
            self.3 = self.2;
            self.2 = self.1;
            self.1 = self.0;
            self.0 = 0;
        } else {
            // the pixels inverted so slide the pattern down and start a new count in the last position
            self.0 = self.1;
            self.1 = self.2;
            self.2 = self.3;
            self.3 = self.4;
            self.4 = self.5;
            self.5 = self.6;
            self.6 = 1;
        }
    }

    fn est_mod_size(&self) -> f64 {
        f64::from(self.2 + self.3 + self.4 + self.5 + self.6) / 7.0
    }

    // Determine if the candidate looks like a finder, with about 1-1-3-1-1 ratios
    fn looks_like_finder(&self) -> bool {
        let total_size = self.2 + self.3 + self.4 + self.5 + self.6;

        if total_size < 7 {
            return false;
        }

        let module_size: f64 = f64::from(total_size) / 7.0;
        let max_variance = module_size / 1.5;

        if (module_size - f64::from(self.2)).abs() > max_variance {
            return false;
        }

        if (module_size - f64::from(self.3)).abs() > max_variance {
            return false;
        }

        if (module_size * 3.0 - f64::from(self.4)).abs() > max_variance {
            return false;
        }

        if (module_size - f64::from(self.5)).abs() > max_variance {
            return false;
        }

        if (module_size - f64::from(self.6)).abs() > max_variance {
            return false;
        }

        true
    }
}

#[inline]
fn diff(a: f64, b: f64) -> f64 {
    if a > b {
        (a - b) / a
    } else {
        (b - a) / b
    }
}

#[inline]
fn dist(one: &Point, other: &Point) -> f64 {
    let dist = ((one.x - other.x) * (one.x - other.x)) + ((one.y - other.y) * (one.y - other.y));
    dist.sqrt()
}

#[inline]
fn find_qr(one: &Point, two: &Point, three: &Point, module_size: f64) -> Option<QRLocation> {
    // Try all three combinations of points to see if any of them are a QR
    if let Some(qr) = find_qr_internal(one, two, three, module_size) {
        Some(qr)
    } else if let Some(qr) = find_qr_internal(two, one, three, module_size) {
        Some(qr)
    } else if let Some(qr) = find_qr_internal(three, one, two, module_size) {
        Some(qr)
    } else {
        None
    }
}

fn find_qr_internal(
    one: &Point,
    two: &Point,
    three: &Point,
    module_size: f64,
) -> Option<QRLocation> {
    let ax = two.x - one.x;
    let ay = two.y - one.y;
    let bx = three.x - one.x;
    let by = three.y - one.y;

    // for images flip the cross product since y is positive towards the bottom
    let cross_product = -(ax * by - ay * bx);
    let len_a = (ax * ax + ay * ay).sqrt();
    let len_b = (bx * bx + by * by).sqrt();

    trace!("DIFF {}", diff(len_a, len_b));

    // The distance between the two finders needs to be similar
    if diff(len_a, len_b) > 0.06 {
        return None;
    }

    let perpendicular = cross_product / len_a / len_b;

    trace!("PERPENDICULAR {}", perpendicular);

    // The two sides need to be perpendicular
    if (perpendicular.abs() - 1.0).abs() > 0.05 {
        return None;
    }

    // Estimate distance between finders, in module count
    let mut dist = ((dist(one, three) / module_size) + 7.0) as u32;

    trace!("DIST {}", dist);

    // QR codes are at least 21 modules wide so discard any that are smaller
    if dist < 20 {
        return None;
    }

    // Since the distance in modules between finders needs to be a multiple of 4 plus one, adjust our estimate if it doesn't conform
    dist = match dist % 4 {
        0 => dist + 1,
        1 => dist,
        2 => dist - 1,
        3 => dist - 2,
        _ => return None,
    };

    // QR might be mirrored, in that case store the finders the other way around
    if perpendicular > 0.0 {
        Some(QRLocation {
            top_left: *one,
            top_right: *three,
            bottom_left: *two,
            module_size,
            version: (dist - 17) / 4,
        })
    } else {
        Some(QRLocation {
            top_left: *one,
            top_right: *two,
            bottom_left: *three,
            module_size,
            version: (dist - 17) / 4,
        })
    }
}

#[derive(Debug)]
pub struct QRFinderPosition {
    pub location: Point,
    pub module_size: f64,
    pub last_module_size: f64,
}
