use qr::QRData;
use qr::QRLocation;

use image::GrayImage;

#[cfg(feature = "debug-images")]
use image::{DynamicImage, Rgb};

#[cfg(feature = "debug-images")]
use std::{
    cmp::{max, min}, env::temp_dir, fs::create_dir_all,
};

pub trait Extract<T> {
    fn extract(&self, threshold: &T, locs: Vec<QRLocation>) -> Vec<QRData>;
}

pub struct QRExtractor {}

impl QRExtractor {
    pub fn new() -> QRExtractor {
        QRExtractor {}
    }
}

impl Extract<GrayImage> for QRExtractor {
    fn extract(&self, threshold: &GrayImage, locs: Vec<QRLocation>) -> Vec<QRData> {
        let mut qr_data = vec![];

        for loc in locs {
            let size = 17 + loc.version * 4;

            let mut dx = loc.top_right - loc.top_left;
            dx = dx / (size - 7) as f64;

            let mut dy = loc.bottom_left - loc.top_left;
            dy = dy / (size - 7) as f64;

            let mut start = loc.top_left - 3.0 * dx;
            start = start - 3.0 * dy;

            let mut data = vec![];

            #[cfg(feature = "debug-images")]
            let mut img = DynamicImage::ImageLuma8(threshold.clone()).to_rgb();

            for _ in 0..size {
                let mut line = start.clone();

                for _ in 0..size {
                    let x = line.x.round() as u32;
                    let y = line.y.round() as u32;
                    let pixel = threshold.get_pixel(x, y)[0];

                    #[cfg(feature = "debug-images")]
                    {
                        if pixel == 0 {
                            for i in max(0, x.saturating_sub(2))..min(img.dimensions().0, x + 2) {
                                for j in max(0, y.saturating_sub(2))..min(img.dimensions().0, y + 2)
                                {
                                    img.put_pixel(i, j, Rgb { data: [255, 0, 0] });
                                }
                            }
                        }
                    }

                    data.push(pixel);
                    line = line + dx;
                }

                start = start + dy;
            }

            #[cfg(feature = "debug-images")]
            {
                let mut tmp = temp_dir();
                tmp.push("bardecoder-debug-images");

                if let Ok(_) = create_dir_all(tmp.clone()) {
                    tmp.push("extract.png");

                    if let Ok(_) = DynamicImage::ImageRgb8(img).save(tmp.clone()) {
                        debug!("Debug image with data pixels saved to {:?}", tmp);
                    }
                }
            }

            qr_data.push(QRData::new(data, loc.version));
        }

        qr_data
    }
}
