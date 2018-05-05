use super::*;

pub struct LineScan {}

impl LineScan {
    pub fn new() -> LineScan {
        LineScan {}
    }
}

impl Locate<GrayImage> for LineScan {
    fn locate(&self, threshold: &GrayImage) -> Vec<QRFinderPosition> {
        //let locations = vec![];
        let mut candidates = vec![];

        let mut last_pixel = 127;
        let mut pattern = QRFinderPattern::new();
        for (x, y, p) in threshold.enumerate_pixels() {
            if x == 0 {
                last_pixel = 127;
                pattern = QRFinderPattern::new();
            }

            if p.data[0] == last_pixel {
                pattern.4 += 1;
            } else {
                if pattern.looks_like_finder() {
                    let module_size =
                        (pattern.0 + pattern.1 + pattern.2 + pattern.3 + pattern.4) as f64 / 7.0;

                    let finder_x = x - (3.5 * module_size) as u32;
                    let finder_y = y;

                    if self.verify_vertical(threshold, finder_x, finder_y, module_size)
                        && self.verify_diagonal(threshold, finder_x, finder_y, module_size)
                    {
                        candidates.push(QRFinderPosition {
                            x: finder_x,
                            y: finder_y,
                        });
                    }
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
    fn verify_vertical(
        &self,
        threshold: &GrayImage,
        finder_x: u32,
        finder_y: u32,
        module_size: f64,
    ) -> bool {
        let dims = threshold.dimensions();

        if finder_y < 7 * module_size as u32 {
            return false;
        }

        if dims.1 - finder_y < 7 * module_size as u32 {
            return false;
        }

        let mut last_pixel = 127;
        let mut pattern = QRFinderPattern::new();
        for y in finder_y.saturating_sub(5 * module_size as u32)..finder_y + 5 * module_size as u32
        {
            let p = threshold.get_pixel(finder_x, y)[0];
            if p == last_pixel {
                pattern.4 += 1;
            } else {
                if pattern.looks_like_finder() {
                    return true;
                }

                last_pixel = p;
                pattern.slide();
            }
        }

        false
    }

    fn verify_diagonal(
        &self,
        threshold: &GrayImage,
        finder_x: u32,
        finder_y: u32,
        module_size: f64,
    ) -> bool {
        let dims = threshold.dimensions();

        if finder_x < 7 * module_size as u32 {
            return false;
        }

        if dims.0 - finder_x < 7 * module_size as u32 {
            return false;
        }

        let mut last_pixel = 127;
        let mut pattern = QRFinderPattern::new();
        for x in finder_x.saturating_sub(5 * module_size as u32)..finder_x + 5 * module_size as u32
        {
            for y in
                finder_y.saturating_sub(5 * module_size as u32)..finder_y + 5 * module_size as u32
            {
                let p = threshold.get_pixel(x, y)[0];
                if p == last_pixel {
                    pattern.4 += 1;
                } else {
                    if pattern.looks_like_finder() {
                        return true;
                    }

                    last_pixel = p;
                    pattern.slide();
                }
            }
        }

        false
    }
}

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

    fn looks_like_finder(&self) -> bool {
        let total_size: i64 = (self.0 + self.1 + self.2 + self.3 + self.4) as i64;

        if total_size < 7 {
            return false;
        }

        let module_size: i64 = total_size / 7;
        let max_variance = module_size as f64 / 2.0;

        if (module_size - self.0 as i64).abs() as f64 > max_variance {
            return false;
        }

        if (module_size - self.1 as i64).abs() as f64 > max_variance {
            return false;
        }

        if (module_size * 3 - self.2 as i64).abs() as f64 > max_variance {
            return false;
        }

        if (module_size - self.3 as i64).abs() as f64 > max_variance {
            return false;
        }

        if (module_size - self.4 as i64).abs() as f64 > max_variance {
            return false;
        }

        true
    }
}
