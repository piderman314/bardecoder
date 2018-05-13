use qr::QRData;
use qr::QRLocation;

use image::DynamicImage;
use image::GrayImage;
use image::Rgb;

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

            let mut i = DynamicImage::ImageLuma8(threshold.clone()).to_rgb();

            for _ in 0..size {
                let mut line = start.clone();

                for _ in 0..size {
                    let pixel =
                        threshold.get_pixel(line.x.round() as u32, line.y.round() as u32)[0];

                    if pixel == 0 {
                        i.put_pixel(
                            line.x.round() as u32,
                            line.y.round() as u32,
                            Rgb { data: [255, 0, 0] },
                        );
                    }

                    data.push(pixel);
                    line = line + dx;
                }

                start = start + dy;
            }

            i.save("D:/prog/rust/qrs_bin/testimg/out.png").unwrap();

            qr_data.push(QRData::new(data, loc.version));
        }

        qr_data
    }
}
