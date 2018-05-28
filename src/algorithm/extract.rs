use qr::{QRData, QRError, QRLocation};

use point::{Delta, Point};

use image::GrayImage;

#[cfg(feature = "debug-images")]
use image::{DynamicImage, Rgb};

#[cfg(feature = "debug-images")]
use std::{
    cmp::{max, min}, env::temp_dir, fs::create_dir_all,
};

pub trait Extract<T> {
    fn extract(&self, threshold: &T, locs: Vec<QRLocation>) -> Vec<Result<QRData, QRError>>;
}

pub struct QRExtractor {}

impl QRExtractor {
    pub fn new() -> QRExtractor {
        QRExtractor {}
    }
}

impl Extract<GrayImage> for QRExtractor {
    fn extract(
        &self,
        threshold: &GrayImage,
        locs: Vec<QRLocation>,
    ) -> Vec<Result<QRData, QRError>> {
        let mut qr_data = vec![];

        for loc in locs {
            let size = 17 + loc.version * 4;
            let p = determine_perspective(threshold, loc.version, size, &loc);

            if p.is_err() {
                qr_data.push(Err(p.err().unwrap()));
                continue;
            }

            let p = p.unwrap();

            let mut start = loc.top_left - 3.0 * p.dx - 3.0 * p.ddx;
            start = start - 3.0 * p.dy - 3.0 * p.ddy;

            let mut dx = p.dx - 2.0 * p.ddx;
            let mut dy = p.dy - 2.0 * p.ddy;

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
                    dx = dx + p.ddx;
                }

                start = start + dy;
                dy = dy + p.ddy;
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

            qr_data.push(Ok(QRData::new(data, loc.version)));
        }

        qr_data
    }
}

fn determine_perspective(
    threshold: &GrayImage,
    version: u32,
    size: u32,
    loc: &QRLocation,
) -> Result<Perspective, QRError> {
    let mut dx = loc.top_right - loc.top_left;
    dx = dx / (size - 7) as f64;

    let mut dy = loc.bottom_left - loc.top_left;
    dy = dy / (size - 7) as f64;

    if version == 1 {
        return Ok(Perspective::new(
            dx,
            Delta { dx: 0.0, dy: 0.0 },
            dy,
            Delta { dx: 0.0, dy: 0.0 },
        ));
    }

    let mut est_alignment = Point {
        x: (loc.top_right - 3.0 * dx).x,
        y: (loc.bottom_left - 3.0 * dy).y,
    };

    let mut found = false;

    'distance: for i in (0..8).rev() {
        if i == 0 {
            if is_alignment(threshold, est_alignment, dx, dy) {
                found = true;
                break 'distance;
            }

            continue 'distance;
        }

        for x in -i..i + 1 {
            let alignment = est_alignment + x as f64 / 2.0 * dx - i as f64 / 2.0 * dy;
            if is_alignment(threshold, alignment, dx, dy) {
                est_alignment = alignment;
                found = true;
                break 'distance;
            }

            let alignment = est_alignment + x as f64 / 2.0 * dx + i as f64 / 2.0 * dy;
            if is_alignment(threshold, alignment, dx, dy) {
                est_alignment = alignment;
                found = true;
                break 'distance;
            }
        }

        for y in -i + 1..i {
            let alignment = est_alignment - i as f64 / 2.0 * dx + y as f64 / 2.0 * dy;
            if is_alignment(threshold, alignment, dx, dy) {
                est_alignment = alignment;
                found = true;
                break 'distance;
            }

            let alignment = est_alignment + i as f64 / 2.0 * dx + y as f64 / 2.0 * dy;
            if is_alignment(threshold, alignment, dx, dy) {
                est_alignment = alignment;
                found = true;
                break 'distance;
            }
        }
    }

    if !found {
        return Err(QRError {
            msg: String::from("Unable to find alignment pattern"),
        });
    }

    #[cfg(feature = "debug-images")]
    {
        let mut img = DynamicImage::ImageLuma8(threshold.clone()).to_rgb();

        let x_start = max(0, (est_alignment.x - 2.5 * loc.module_size) as u32);
        let x_end = min(
            img.dimensions().0,
            (est_alignment.x + 2.5 * loc.module_size) as u32,
        );
        let y_start = max(0, (est_alignment.y - 2.5 * loc.module_size) as u32);
        let y_end = min(
            img.dimensions().0,
            (est_alignment.y + 2.5 * loc.module_size) as u32,
        );

        for x in x_start..x_end {
            for y in y_start..y_end {
                if x > x_start + 2 && x < x_end - 2 && y > y_start + 2 && y < y_end - 2 {
                    continue;
                }

                img.put_pixel(x, y, Rgb { data: [255, 0, 0] });
            }
        }

        let mut tmp = temp_dir();
        tmp.push("bardecoder-debug-images");

        if let Ok(_) = create_dir_all(tmp.clone()) {
            tmp.push("alignment.png");

            if let Ok(_) = DynamicImage::ImageRgb8(img).save(tmp.clone()) {
                debug!("Debug image with data pixels saved to {:?}", tmp);
            }
        }
    }

    Ok(Perspective::new(
        dx,
        Delta { dx: 0.0, dy: 0.0 },
        dy,
        Delta { dx: 0.0, dy: 0.0 },
    ))
}

fn is_alignment(threshold: &GrayImage, p: Point, dx: Delta, dy: Delta) -> bool {
    let top_left = p - 2.0 * dx - 2.0 * dy;
    if top_left.x < 0.0 || top_left.y < 0.0 {
        return false;
    }

    let bottom_right = p + 2.0 * dx + 2.0 * dy;
    let dims = threshold.dimensions();
    if bottom_right.x > dims.0 as f64 || bottom_right.y > dims.1 as f64 {
        return false;
    }

    for x in -2..2 {
        let twice_up = p - x as f64 * dx - 2.0 * dy;
        if threshold.get_pixel(twice_up.x.round() as u32, twice_up.y.round() as u32)[0] == 255 {
            return false;
        }

        let twice_down = p - x as f64 * dx + 2.0 * dy;
        if threshold.get_pixel(twice_down.x.round() as u32, twice_down.y.round() as u32)[0] == 255 {
            return false;
        }
    }

    for y in -1..1 {
        let twice_left = p - 2.0 * dx - y as f64 * dy;
        if threshold.get_pixel(twice_left.x.round() as u32, twice_left.y.round() as u32)[0] == 255 {
            return false;
        }

        let twice_right = p + 2.0 * dx - y as f64 * dy;
        if threshold.get_pixel(twice_right.x.round() as u32, twice_right.y.round() as u32)[0] == 255
        {
            return false;
        }

        let left = p - dx - y as f64 * dy;
        if threshold.get_pixel(left.x.round() as u32, left.y.round() as u32)[0] == 0 {
            return false;
        }

        let right = p - dx - y as f64 * dy;
        if threshold.get_pixel(right.x.round() as u32, right.y.round() as u32)[0] == 0 {
            return false;
        }
    }

    let up = p - dy;
    if threshold.get_pixel(up.x.round() as u32, up.y.round() as u32)[0] == 0 {
        return false;
    }

    let down = p + dy;
    if threshold.get_pixel(down.x.round() as u32, down.y.round() as u32)[0] == 0 {
        return false;
    }

    threshold.get_pixel(p.x.round() as u32, p.y.round() as u32)[0] == 0
}

struct Perspective {
    dx: Delta,
    ddx: Delta,
    dy: Delta,
    ddy: Delta,
}

impl Perspective {
    fn new(dx: Delta, ddx: Delta, dy: Delta, ddy: Delta) -> Perspective {
        Perspective { dx, ddx, dy, ddy }
    }
}
