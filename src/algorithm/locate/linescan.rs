use super::*;

use qr::QRFinderPosition;

use std::cmp::{max, min};
use std::iter::repeat;
use std::iter::Iterator;

#[cfg(feature = "debug-images")]
use image::{DynamicImage, Rgb};

#[cfg(feature = "debug-images")]
use std::{env::temp_dir, fs::create_dir_all};

pub struct LineScan {}

impl LineScan {
    pub fn new() -> LineScan {
        LineScan {}
    }
}

type Refine = Fn(&LineScan, &GrayImage, &Point, f64) -> Option<QRFinderPosition>;

impl Locate<GrayImage> for LineScan {
    fn locate(&self, threshold: &GrayImage) -> Vec<QRLocation> {
        // The order of refinement is important.
        // The candidate is found in horizontal direction, so the first refinement is vertical
        let refine_func: Vec<(Box<Refine>, f64, f64)> = vec![
            (Box::new(LineScan::refine_vertical), 0.0, 1.0),
            (Box::new(LineScan::refine_horizontal), 1.0, 0.0),
            (Box::new(LineScan::refine_diagonal), 1.0, 1.0),
        ];

        let mut candidates: Vec<QRFinderPosition> = vec![];

        let mut last_pixel = 127;
        let mut pattern = QRFinderPattern::new();
        'pixels: for (x, y, p) in threshold.enumerate_pixels() {
            if x == 0 {
                last_pixel = 127;
                pattern = QRFinderPattern::new();
            }

            if p.data[0] == last_pixel {
                pattern.6 += 1;
                continue 'pixels;
            }

            if !pattern.looks_like_finder() {
                last_pixel = p.data[0];
                pattern.slide();
                continue 'pixels;
            }

            let mut module_size = pattern.est_mod_size();

            let mut finder = Point {
                x: x as f64 - module_size * 3.5,
                y: y as f64,
            };

            for candidate in &candidates {
                if dist(&finder, &candidate.location) < 7.0 * module_size {
                    last_pixel = p.data[0];
                    pattern.slide();

                    continue 'pixels;
                }
            }

            for (refine_func, dx, dy) in refine_func.iter() {
                let vert = refine_func(&self, threshold, &finder, module_size);

                if vert.is_none() {
                    last_pixel = p.data[0];
                    pattern.slide();

                    continue 'pixels;
                }

                let vert = vert.unwrap();
                let half_finder = 3.5 * vert.last_module_size;
                finder.x = vert.location.x - dx * half_finder;
                finder.y = vert.location.y - dy * half_finder;
                module_size = vert.module_size;
            }

            candidates.push(QRFinderPosition {
                location: finder,
                module_size,
                last_module_size: 0.0,
            });

            last_pixel = p.data[0];
            pattern.slide();
        }

        debug!("Candidate QR Locators {:#?}", candidates);

        #[cfg(feature = "debug-images")]
        {
            #[cfg(feature = "debug-images")]
            let mut img = DynamicImage::ImageLuma8(threshold.clone()).to_rgb();

            for c in candidates.iter() {
                let loc = c.location;
                let x_start = max(0, (loc.x - 3.5 * c.module_size) as u32);
                let x_end = min(img.dimensions().0, (loc.x + 3.5 * c.module_size) as u32);
                let y_start = max(0, (loc.y - 3.5 * c.module_size) as u32);
                let y_end = min(img.dimensions().0, (loc.y + 3.5 * c.module_size) as u32);

                for x in x_start..x_end {
                    for y in y_start..y_end {
                        if x > x_start + 3 && x < x_end - 3 && y > y_start + 3 && y < y_end - 3 {
                            continue;
                        }

                        img.put_pixel(x, y, Rgb { data: [255, 0, 0] });
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

        let mut locations: Vec<QRLocation> = vec![];

        let max_candidates = candidates.len();

        for candidate1 in 0..max_candidates {
            for candidate2 in candidate1 + 1..max_candidates {
                if diff(
                    candidates[candidate1].module_size,
                    candidates[candidate2].module_size,
                ) > 0.05
                {
                    continue;
                }

                for candidate3 in candidate2 + 1..max_candidates {
                    if diff(
                        candidates[candidate1].module_size,
                        candidates[candidate3].module_size,
                    ) > 0.05
                    {
                        continue;
                    }

                    if let Some(qr) = find_qr(
                        &candidates[candidate1].location,
                        &candidates[candidate2].location,
                        &candidates[candidate3].location,
                        candidates[candidate1].module_size,
                    ) {
                        locations.push(qr);
                    }
                }
            }
        }

        locations
    }
}

impl LineScan {
    fn refine_horizontal(
        &self,
        threshold: &GrayImage,
        finder: &Point,
        module_size: f64,
    ) -> Option<QRFinderPosition> {
        let start_x = max(0, (finder.x - 5.0 * module_size).round() as u32);
        let end_x = min(
            (finder.x + 5.0 * module_size).round() as u32,
            threshold.dimensions().0,
        );

        let range_x = start_x..end_x;
        let range_y = repeat(finder.y.round() as u32);

        self.refine(threshold, module_size, range_x, range_y)
    }

    fn refine_vertical(
        &self,
        threshold: &GrayImage,
        finder: &Point,
        module_size: f64,
    ) -> Option<QRFinderPosition> {
        let start_y = max(0, (finder.y - 5.0 * module_size).round() as u32);
        let end_y = min(
            (finder.y + 5.0 * module_size).round() as u32,
            threshold.dimensions().1,
        );

        let range_x = repeat(finder.x.round() as u32);
        let range_y = start_y..end_y;

        self.refine(threshold, module_size, range_x, range_y)
    }

    fn refine_diagonal(
        &self,
        threshold: &GrayImage,
        finder: &Point,
        module_size: f64,
    ) -> Option<QRFinderPosition> {
        let side = 5.0 * module_size;
        let mut start_x = 0.0;
        let mut start_y = 0.0;
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

        let range_x = start_x.round() as u32
            ..min(
                (finder.x + 5.0 * module_size).round() as u32,
                threshold.dimensions().0,
            );
        let range_y = start_y.round() as u32
            ..min(
                (finder.y + 5.0 * module_size).round() as u32,
                threshold.dimensions().1,
            );

        self.refine(threshold, module_size, range_x, range_y)
    }

    fn refine(
        &self,
        threshold: &GrayImage,
        module_size: f64,
        range_x: impl Iterator<Item = u32>,
        range_y: impl Iterator<Item = u32>,
    ) -> Option<QRFinderPosition> {
        let mut last_pixel = 127;
        let mut pattern = QRFinderPattern::new();
        let mut last_x = 0;
        let mut last_y = 0;
        for (x, y) in range_x.zip(range_y) {
            let p = threshold.get_pixel(x, y)[0];
            if p == last_pixel {
                pattern.6 += 1;
            } else {
                if pattern.looks_like_finder() {
                    if diff(module_size, pattern.est_mod_size()) < 0.2 {
                        let new_est_mod_size = (module_size + pattern.est_mod_size()) / 2.0;
                        return Some(QRFinderPosition {
                            location: Point {
                                x: x as f64,
                                y: y as f64,
                            },
                            module_size: new_est_mod_size,
                            last_module_size: pattern.est_mod_size(),
                        });
                    }
                }

                last_pixel = p;
                pattern.slide();
            }

            last_x = x;
            last_y = y;
        }

        if pattern.looks_like_finder() {
            if diff(module_size, pattern.est_mod_size()) < 0.2 {
                let new_est_mod_size = (module_size + pattern.est_mod_size()) / 2.0;
                return Some(QRFinderPosition {
                    location: Point {
                        x: last_x as f64,
                        y: last_y as f64,
                    },
                    module_size: new_est_mod_size,
                    last_module_size: pattern.est_mod_size(),
                });
            }
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
        if (self.6 as f64) < self.5 as f64 / 10.0 {
            self.6 += self.5;
            self.5 = self.4;
            self.4 = self.3;
            self.3 = self.2;
            self.2 = self.1;
            self.1 = self.0;
            self.0 = 0;
        } else {
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
        (self.2 + self.3 + self.4 + self.5 + self.6) as f64 / 7.0
    }

    fn looks_like_finder(&self) -> bool {
        let total_size = self.2 + self.3 + self.4 + self.5 + self.6;

        if total_size < 7 {
            return false;
        }

        let module_size: f64 = total_size as f64 / 7.0;
        let max_variance = module_size as f64 / 1.5;

        if (module_size - self.2 as f64).abs() > max_variance {
            return false;
        }

        if (module_size - self.3 as f64).abs() > max_variance {
            return false;
        }

        if (module_size * 3.0 - self.4 as f64).abs() > max_variance {
            return false;
        }

        if (module_size - self.5 as f64).abs() > max_variance {
            return false;
        }

        if (module_size - self.6 as f64).abs() > max_variance {
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
    if let Some(qr) = find_qr_internal(one, two, three, module_size) {
        return Some(qr);
    } else if let Some(qr) = find_qr_internal(two, one, three, module_size) {
        return Some(qr);
    } else if let Some(qr) = find_qr_internal(three, one, two, module_size) {
        return Some(qr);
    } else {
        return None;
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

    if diff(len_a, len_b) > 0.06 {
        return None;
    }

    let perpendicular = cross_product / len_a / len_b;

    trace!("PERPENDICULAR {}", perpendicular);

    if (perpendicular.abs() - 1.0).abs() > 0.05 {
        return None;
    }

    let mut dist = ((dist(one, three) / module_size) + 7.0) as u32;

    trace!("DIST {}", dist);

    if dist < 20 {
        return None;
    }

    dist = match dist % 4 {
        0 => dist + 1,
        1 => dist,
        2 => dist - 1,
        3 => dist - 2,
        _ => return None,
    };

    if perpendicular > 0.0 {
        Some(QRLocation {
            top_left: one.clone(),
            top_right: three.clone(),
            bottom_left: two.clone(),
            module_size,
            version: (dist - 17) / 4,
        })
    } else {
        Some(QRLocation {
            top_left: one.clone(),
            top_right: two.clone(),
            bottom_left: three.clone(),
            module_size,
            version: (dist - 17) / 4,
        })
    }
}
