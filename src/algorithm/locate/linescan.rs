use super::*;

use std::cmp::{max, min};
use std::iter::repeat;
use std::iter::Iterator;

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
                pattern.4 += 1;
                continue 'pixels;
            }

            if !pattern.looks_like_finder() {
                last_pixel = p.data[0];
                pattern.slide();
                continue 'pixels;
            }

            let mut module_size = pattern.est_mod_size();

            let mut finder = Point {
                x: x as f64 - (pattern.0 + pattern.1 + pattern.2) as f64 / 2.0,
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
                let half_finder = 3.5 * vert.module_size;
                finder.x = vert.location.x - dx * half_finder;
                finder.y = vert.location.y - dy * half_finder;
                module_size = vert.module_size;
            }

            candidates.push(QRFinderPosition {
                location: finder,
                module_size,
            });

            last_pixel = p.data[0];
            pattern.slide();
        }

        let mut locations: Vec<QRLocation> = vec![];

        let mut loc_candidates: Vec<LocationCandidate> = vec![];

        'candidates: for candidate in candidates {
            for loc_candidate in loc_candidates.iter_mut() {
                if diff(candidate.module_size, loc_candidate.module_size) < 0.05 {
                    if loc_candidate.positions.len() == 1 {
                        let candidate_pos = loc_candidate.positions[0];

                        let dist = ((dist(&candidate.location, &candidate_pos)
                            / candidate.module_size) + 7.0)
                            as u32;

                        if dist >= 21 && dist % 4 == 1
                            && !is_diagonal(&candidate.location, &candidate_pos)
                        {
                            loc_candidate.positions.push(candidate.location);
                            loc_candidate.version = (dist - 1) / 4;

                            continue 'candidates;
                        }
                    } else if loc_candidate.positions.len() == 2 && loc_candidate.version != 0 {
                        let mut add = false;
                        for candidate_pos in loc_candidate.positions.iter() {
                            let dist = ((dist(&candidate.location, &candidate_pos)
                                / candidate.module_size)
                                + 7.0) as u32;

                            let version = (dist - 1) / 4;
                            if version == loc_candidate.version
                                && !is_diagonal(&candidate.location, &candidate_pos)
                            {
                                add = true;
                            }
                        }

                        if add {
                            loc_candidate.positions.push(candidate.location);

                            continue 'candidates;
                        }
                    }
                }
            }

            loc_candidates.push(LocationCandidate {
                positions: vec![candidate.location],
                module_size: candidate.module_size,
                version: 0,
            });
        }

        for loc_candidate in loc_candidates {
            if loc_candidate.positions.len() == 3 {
                let pos = &loc_candidate.positions;

                if is_diagonal(&pos[0], &pos[1]) {
                    let ax = pos[2].x - pos[0].x;
                    let ay = pos[2].y - pos[0].y;
                    let bx = pos[2].x - pos[1].x;
                    let by = pos[2].y - pos[1].y;

                    if ax * by - ay * bx > 0.0 {
                        locations.push(QRLocation {
                            top_left: pos[2],
                            top_right: pos[1],
                            bottom_left: pos[0],
                            module_size: loc_candidate.module_size,
                            version: loc_candidate.version,
                        });
                    } else {
                        locations.push(QRLocation {
                            top_left: pos[2],
                            top_right: pos[0],
                            bottom_left: pos[1],
                            module_size: loc_candidate.module_size,
                            version: loc_candidate.version,
                        });
                    }
                } else if is_diagonal(&pos[0], &pos[2]) {
                    let ax = pos[1].x - pos[0].x;
                    let ay = pos[1].y - pos[0].y;
                    let bx = pos[1].x - pos[2].x;
                    let by = pos[1].y - pos[2].y;

                    if ax * by - ay * bx > 0.0 {
                        locations.push(QRLocation {
                            top_left: pos[1],
                            top_right: pos[2],
                            bottom_left: pos[0],
                            module_size: loc_candidate.module_size,
                            version: loc_candidate.version,
                        });
                    } else {
                        locations.push(QRLocation {
                            top_left: pos[1],
                            top_right: pos[0],
                            bottom_left: pos[2],
                            module_size: loc_candidate.module_size,
                            version: loc_candidate.version,
                        });
                    }
                } else {
                    let ax = pos[0].x - pos[2].x;
                    let ay = pos[0].y - pos[2].y;
                    let bx = pos[0].x - pos[1].x;
                    let by = pos[0].y - pos[1].y;

                    if ax * by - ay * bx > 0.0 {
                        locations.push(QRLocation {
                            top_left: pos[0],
                            top_right: pos[1],
                            bottom_left: pos[2],
                            module_size: loc_candidate.module_size,
                            version: loc_candidate.version,
                        });
                    } else {
                        locations.push(QRLocation {
                            top_left: pos[0],
                            top_right: pos[2],
                            bottom_left: pos[1],
                            module_size: loc_candidate.module_size,
                            version: loc_candidate.version,
                        });
                    }
                }
            }
        }

        locations
    }
}

#[derive(Debug)]
struct LocationCandidate {
    positions: Vec<Point>,
    module_size: f64,
    version: u32,
}

impl LineScan {
    fn refine_horizontal(
        &self,
        threshold: &GrayImage,
        finder: &Point,
        module_size: f64,
    ) -> Option<QRFinderPosition> {
        let start_x = max(0, (finder.x - 7.0 * module_size).round() as u32);
        let end_x = min(
            (finder.x + 7.0 * module_size).round() as u32,
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
        let start_y = max(0, (finder.y - 7.0 * module_size).round() as u32);
        let end_y = min(
            (finder.y + 7.0 * module_size).round() as u32,
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
        let side = 7.0 * module_size;
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
                (finder.x + 7.0 * module_size).round() as u32,
                threshold.dimensions().0,
            );
        let range_y = start_y.round() as u32
            ..min(
                (finder.y + 7.0 * module_size).round() as u32,
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
                pattern.4 += 1;
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
        let total_size = self.0 + self.1 + self.2 + self.3 + self.4;

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
fn dist(one: &Point, other: &Point) -> f64 {
    let dist = (one.x - other.x).powf(2.0) + (one.y - other.y).powf(2.0);
    dist.sqrt()
}

#[inline]
fn is_diagonal(one: &Point, other: &Point) -> bool {
    let dx = (one.x - other.x).abs();
    let dy = (one.y - other.y).abs();

    if dx > dy {
        dx / 2.0 < dy
    } else {
        dy / 2.0 < dx
    }
}
