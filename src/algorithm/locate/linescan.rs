use super::*;

use std::cmp::min;
use std::iter::repeat;
use std::iter::Iterator;

pub struct LineScan {}

impl LineScan {
    pub fn new() -> LineScan {
        LineScan {}
    }
}

type Refine = Fn(&LineScan, &GrayImage, u32, u32, f64) -> Option<QRFinderPosition>;

impl Locate<GrayImage> for LineScan {
    fn locate(&self, threshold: &GrayImage) -> Vec<QRFinderPosition> {
        // The order of refinement is important.
        // The candidate is found in horizontal direction, so the first refinement is vertical
        let refine_func: Vec<(Box<Refine>, u32, u32)> = vec![
            (Box::new(LineScan::refine_vertical), 0, 1),
            (Box::new(LineScan::refine_horizontal), 1, 0),
            (Box::new(LineScan::refine_diagonal), 1, 1),
        ];

        //let locations = vec![];
        let mut candidates: Vec<QRFinderPosition> = vec![];

        let mut last_pixel = 127;
        let mut pattern = QRFinderPattern::new();
        'pixels: for (x, y, p) in threshold.enumerate_pixels() {
            if x == 0 {
                last_pixel = 127;
                pattern = QRFinderPattern::new();
            }

            if p.data[0] == last_pixel {
                pattern.4 += 1;
            } else {
                if pattern.looks_like_finder() {
                    let mut module_size = pattern.est_mod_size();

                    let mut finder_x = x - (pattern.0 + pattern.1 + pattern.2 / 2) as u32;
                    let mut finder_y = y;

                    for candidate in &candidates {
                        if dist(finder_x, finder_y, candidate.x, candidate.y) < 7.0 * module_size {
                            last_pixel = p.data[0];
                            pattern.slide();

                            continue 'pixels;
                        }
                    }

                    for (refine_func, dx, dy) in refine_func.iter() {
                        let vert = refine_func(&self, threshold, finder_x, finder_y, module_size);

                        if vert.is_none() {
                            last_pixel = p.data[0];
                            pattern.slide();

                            continue 'pixels;
                        }

                        let vert = vert.unwrap();
                        let half_finder = (3.5 * vert.module_size) as u32;
                        finder_x = vert.x - dx * half_finder;
                        finder_y = vert.y - dy * half_finder;
                        module_size = vert.module_size;
                    }

                    candidates.push(QRFinderPosition {
                        x: finder_x,
                        y: finder_y,
                        module_size,
                    });
                }

                last_pixel = p.data[0];
                pattern.slide();
            }
        }

        candidates

        //locations
    }
}

impl LineScan {
    fn refine_horizontal(
        &self,
        threshold: &GrayImage,
        finder_x: u32,
        finder_y: u32,
        module_size: f64,
    ) -> Option<QRFinderPosition> {
        let range_x = finder_x.saturating_sub(7 * module_size as u32)
            ..min(finder_x + 7 * module_size as u32, threshold.dimensions().0);
        let range_y = repeat(finder_y);

        self.refine(threshold, finder_x, finder_y, module_size, range_x, range_y)
    }

    fn refine_vertical(
        &self,
        threshold: &GrayImage,
        finder_x: u32,
        finder_y: u32,
        module_size: f64,
    ) -> Option<QRFinderPosition> {
        let range_x = repeat(finder_x);
        let range_y = finder_y.saturating_sub(7 * module_size as u32)
            ..min(finder_y + 7 * module_size as u32, threshold.dimensions().1);

        self.refine(threshold, finder_x, finder_y, module_size, range_x, range_y)
    }

    fn refine_diagonal(
        &self,
        threshold: &GrayImage,
        finder_x: u32,
        finder_y: u32,
        module_size: f64,
    ) -> Option<QRFinderPosition> {
        let side = 7 * module_size as u32;
        let mut start_x = 0;
        let mut start_y = 0;
        if finder_x < side && finder_y < side {
            if finder_x < finder_y {
                start_y = finder_y - finder_x;
            } else {
                start_x = finder_x - finder_y;
            }
        } else if finder_x < side {
            start_y = finder_y - finder_x;
        } else if finder_y < side {
            start_x = finder_x - finder_y;
        } else {
            start_x = finder_x.saturating_sub(side);
            start_y = finder_y.saturating_sub(side);
        }

        let range_x = start_x..min(finder_x + 7 * module_size as u32, threshold.dimensions().0);
        let range_y = start_y..min(finder_y + 7 * module_size as u32, threshold.dimensions().1);

        self.refine(threshold, finder_x, finder_y, module_size, range_x, range_y)
    }

    fn refine(
        &self,
        threshold: &GrayImage,
        finder_x: u32,
        finder_y: u32,
        module_size: f64,
        range_x: impl Iterator<Item = u32>,
        range_y: impl Iterator<Item = u32>,
    ) -> Option<QRFinderPosition> {
        let dims = threshold.dimensions();

        if finder_x < module_size as u32 || finder_y < module_size as u32 {
            return None;
        }

        if dims.0 - finder_x < module_size as u32 || dims.1 - finder_y < module_size as u32 {
            return None;
        }

        let mut last_pixel = 127;
        let mut pattern = QRFinderPattern::new();
        let mut last_x = 0;
        let mut last_y = 0;
        for (x, y) in range_x.zip(range_y) {
            let p = threshold.get_pixel(x, y)[0];
            if p == last_pixel {
                pattern.4 += 1;
            } else {
                if pattern.looks_like_finder() {
                    if diff(module_size, pattern.est_mod_size()) < 0.2 {
                        let new_est_mod_size = (module_size + pattern.est_mod_size()) / 2.0;
                        return Some(QRFinderPosition {
                            x: x,
                            y: y,
                            module_size: new_est_mod_size,
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
                    x: last_x,
                    y: last_y,
                    module_size: new_est_mod_size,
                });
            }
        }

        None
    }
}

#[derive(Debug)]
struct QRFinderPattern(u32, u32, u32, u32, u32);

impl QRFinderPattern {
    fn new() -> QRFinderPattern {
        QRFinderPattern(0, 0, 0, 0, 0)
    }

    fn slide(&mut self) {
        self.0 = self.1;
        self.1 = self.2;
        self.2 = self.3;
        self.3 = self.4;
        self.4 = 1;
    }

    fn est_mod_size(&self) -> f64 {
        (self.0 + self.1 + self.2 + self.3 + self.4) as f64 / 7.0
    }

    fn looks_like_finder(&self) -> bool {
        let total_size: i64 = (self.0 + self.1 + self.2 + self.3 + self.4) as i64;

        if total_size < 7 {
            return false;
        }

        let module_size: f64 = total_size as f64 / 7.0;
        let max_variance = module_size as f64 / 1.5;

        if (module_size - self.0 as f64).abs() > max_variance {
            return false;
        }

        if (module_size - self.1 as f64).abs() > max_variance {
            return false;
        }

        if (module_size * 3.0 - self.2 as f64).abs() > max_variance {
            return false;
        }

        if (module_size - self.3 as f64).abs() > max_variance {
            return false;
        }

        if (module_size - self.4 as f64).abs() > max_variance {
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
fn dist(x1: u32, y1: u32, x2: u32, y2: u32) -> f64 {
    let mut dist = 0;
    if x1 > x2 {
        dist += (x1 - x2) * (x1 - x2);
    } else if x1 <= x2 {
        dist += (x2 - x1) * (x2 - x1);
    }

    if y1 > y2 {
        dist += (y1 - y2) * (y1 - y2);
    } else if y1 <= y2 {
        dist += (y2 - y1) * (y2 - y1);
    }

    (dist as f64).sqrt()
}
