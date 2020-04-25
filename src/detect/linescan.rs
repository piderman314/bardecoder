use super::{Detect, Location};

use std::cmp::min;
use std::iter::repeat;
use std::iter::Iterator;

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

struct Candidate {
    p: Point,
    module_size: f64,
}

/*impl std::fmt::Display for Candidate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.p.x, self.p.y, self.module_size)
    }
}*/

impl std::fmt::Debug for Candidate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.p.x, self.p.y, self.module_size)
    }
}

struct LineProcessingClosure<'a> {
    prepared: &'a GrayImage,
    start_y: u32,
    end_y: u32,
}

impl<'a> LineProcessingClosure<'a> {
    fn process(&self) -> Vec<Candidate> {
        let mut points_out = vec![];

        let prepared = self.prepared;
        for y in self.start_y..=self.end_y {
            let mut last_pixel = 127;
            let mut pattern = QRFinderPattern::new();
            'pixels: for x in 0..prepared.width() {
                // A pixel of the same color, add to the count in the last position
                let p = prepared.get_pixel(x, y);
                if p.channels()[0] == last_pixel {
                    pattern.6 += 1;

                    if x != prepared.dimensions().0 - 1 {
                        continue 'pixels;
                    }
                }

                if pattern.looks_like_finder() {
                    let module_size = pattern.est_mod_size();

                    // A finder pattern is 1-1-3-1-1 modules wide, so subtract 3.5 modules to get the x coordinate in the center
                    points_out.push(Candidate {
                        p: Point {
                            x: f64::from(x) - module_size * 3.5,
                            y: f64::from(y),
                        },
                        module_size,
                    });
                }
                // A pixel color switch, but the current pattern does not look like a finder
                // Slide the pattern and continue searching
                last_pixel = p.channels()[0];
                pattern.slide();
            }
        }

        // Return the candidates we've found.
        points_out
    }
}

struct CandidateRefiningClosure<'a> {
    prepared: &'a GrayImage,
}

impl<'a> CandidateRefiningClosure<'a> {
    fn process(&self, unfiltered: &[Candidate]) -> Vec<QRFinderPosition> {
        // The order of refinement is important.
        // The candidate is found in horizontal direction, so the first refinement is vertical
        let refine_func: Vec<(Box<Refine>, f64, f64, bool)> = vec![
            (Box::new(LineScan::refine_vertical), 0.0, 1.0, false),
            (Box::new(LineScan::refine_horizontal), 1.0, 0.0, false),
            (Box::new(LineScan::refine_diagonal), 1.0, 1.0, true),
        ];

        let prepared = self.prepared;
        let mut filtered = vec![];

        'filter: for u in unfiltered {
            // Step 2
            // Run the refinement functions on the candidate location
            let mut finder = u.p;
            let mut module_size = u.module_size;
            for (refine_func, dx, dy, is_diagonal) in &refine_func {
                let vert = refine_func(prepared, &finder, module_size);

                if vert.is_none() {
                    continue 'filter;
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

            filtered.push(QRFinderPosition {
                location: finder,
                module_size,
                last_module_size: 0.0,
            });
        }

        filtered
    }
}

impl Detect<GrayImage> for LineScan {
    fn detect(&self, prepared: &GrayImage) -> Vec<Location> {
        let mut candidates_unfiltered: Vec<Candidate> = vec![];

        #[cfg(feature = "multithreaded")]
        let num_threads = 16;

        // Loop over each row.
        #[cfg(feature = "multithreaded")]
        {
            crossbeam::scope(|scope| {
                let rows_per_section =
                    (prepared.height() as f64 / num_threads as f64).ceil() as u32;
                debug!("Rows per section: {}", rows_per_section);
                let mut thread_handles = vec![];

                for start_y in (0..prepared.height()).step_by(rows_per_section as usize) {
                    let end_y = min(start_y + rows_per_section, prepared.height() - 1);
                    let process_line = LineProcessingClosure {
                        prepared,
                        start_y,
                        end_y,
                    };
                    debug!(
                        "Processing lines {} through {} (out of {} lines) as one chunk",
                        start_y,
                        end_y,
                        prepared.height()
                    );
                    thread_handles.push(scope.spawn(move |_| process_line.process()));
                }

                // Join the threads back together.
                for h in thread_handles {
                    candidates_unfiltered.extend(h.join().unwrap());
                }
            })
            .unwrap();
        }
        #[cfg(not(feature = "multithreaded"))]
        {
            let process_line = LineProcessingClosure {
                prepared,
                start_y: 0,
                end_y: prepared.height() - 1,
            };
            candidates_unfiltered.extend(process_line.process());
        }

        /*println!(
            "Candidate QR Locators unfiltered {:#?}",
            candidates_unfiltered
        );*/

        // Filter similar points in a single thread so we get them in the correct order.
        if candidates_unfiltered.len() > 1 {
            // Looping backwards because it's more likely we'll find matches quicker.
            let mut x = candidates_unfiltered.len() - 2;
            loop {
                // Looping backwards so can remove from the end of the vector and continue without invalidating our range.
                for y in (x + 1..candidates_unfiltered.len()).rev() {
                    // Only remove "duplicates" if the module size is similar.
                    if (candidates_unfiltered[x].module_size - candidates_unfiltered[y].module_size)
                        .abs()
                        >= 0.1
                    {
                        continue;
                    }
                    // Skip the square roots.
                    // Permit some similarities until we get to the refinement stage.
                    let dist_limit = 3.5 * candidates_unfiltered[y].module_size;
                    let dist_limit_sqr = dist_limit * dist_limit;
                    let candidate_dist_sqr =
                        dist_sqr(&candidates_unfiltered[y].p, &candidates_unfiltered[x].p);
                    if candidate_dist_sqr < dist_limit_sqr {
                        // The candidate location we have found was already detected and stored on a previous line.
                        // Remove this one.
                        /*println!(
                            "Removing {} ({}, {}) for distance {} and limit {} from ({}, {})",
                            y,
                            candidates_unfiltered[y].p.x,
                            candidates_unfiltered[y].p.y,
                            candidate_dist_sqr,
                            dist_limit_sqr,
                            candidates_unfiltered[x].p.x,
                            candidates_unfiltered[x].p.y
                        );*/
                        candidates_unfiltered.remove(y);
                    }
                }
                if x == 0 {
                    break;
                }
                x -= 1;
            }
        }
        let mut candidates: Vec<QRFinderPosition> = vec![];

        // Loop over each candidate.
        #[cfg(feature = "multithreaded")]
        {
            crossbeam::scope(|scope| {
                let candidates_per_section =
                    (candidates_unfiltered.len() as f64 / num_threads as f64).ceil() as u32;
                let mut thread_handles = vec![];

                for start_index in
                    (0..candidates_unfiltered.len()).step_by(candidates_per_section as usize)
                {
                    let end_index = min(
                        start_index + candidates_per_section as usize,
                        candidates_unfiltered.len() - 1,
                    );
                    let candidate_slice = &candidates_unfiltered[start_index..=end_index];
                    let process_candidate = CandidateRefiningClosure { prepared };
                    debug!(
                        "Processing candidates {} through {} (out of {} candidates) as one chunk",
                        start_index,
                        end_index,
                        candidates_unfiltered.len()
                    );
                    thread_handles
                        .push(scope.spawn(move |_| process_candidate.process(candidate_slice)));
                }

                // Join the threads back together.
                for h in thread_handles {
                    candidates.extend(h.join().unwrap());
                }
            })
            .unwrap();
        }
        #[cfg(not(feature = "multithreaded"))]
        {
            let process_candidate = CandidateRefiningClosure { prepared };
            candidates.extend(process_candidate.process(candidates_unfiltered.as_slice()));
        }

        // Now that the candidates have been refined, remove duplicates again.
        if candidates.len() > 1 {
            // Looping backwards because it's more likely we'll find matches quicker.
            let mut x = candidates.len() - 2;
            loop {
                // Looping backwards so can remove from the end of the vector and continue without invalidating our range.
                for y in (x + 1..candidates.len()).rev() {
                    // Skip the square roots.
                    let dist_limit = 7.0 * candidates[y].module_size;
                    let dist_limit_sqr = dist_limit * dist_limit;
                    let candidate_dist_sqr =
                        dist_sqr(&candidates[y].location, &candidates[x].location);
                    if candidate_dist_sqr < dist_limit_sqr {
                        // The candidate location we have found was already detected and stored on a previous line.
                        // Remove this one.
                        candidates.remove(y);
                    }
                }
                if x == 0 {
                    break;
                }
                x -= 1;
            }
        }
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

                        if x >= img.width() || y >= img.height() {
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
fn dist_sqr(one: &Point, other: &Point) -> f64 {
    ((one.x - other.x) * (one.x - other.x)) + ((one.y - other.y) * (one.y - other.y))
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
