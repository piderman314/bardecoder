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
                    candidates.push(QRFinderPosition { x, y });
                }

                last_pixel = p.data[0];
                pattern.slide();
            }
        }

        candidates

        //locations
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
