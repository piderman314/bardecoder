use super::Extract;

use crate::util::qr::{QRData, QRError, QRLocation};
use crate::util::{Delta, Point};

use image::GrayImage;

#[cfg(feature = "debug-images")]
use image::{DynamicImage, Rgb};

#[cfg(feature = "debug-images")]
use std::{
    cmp::{max, min},
    env::temp_dir,
    fs::create_dir_all,
};

/// Extract QR Data from a preprocessed image
///
/// If the version of the QR is higher than 1, this extractor will first try to find the bottom left-most
/// alignment pattern and adjust for any perspective skewing.
///
/// Data is extracted by sampling the center pixel of the estimated module locations.
/// These are determined by dividing each row and column into equal parts.
pub struct QRExtractor {}

impl QRExtractor {
    /// Construct a new QRExtractor
    pub fn new() -> QRExtractor {
        QRExtractor {}
    }
}

impl Extract<GrayImage, QRLocation, QRData, QRError> for QRExtractor {
    fn extract(&self, prepared: &GrayImage, loc: QRLocation) -> Result<QRData, QRError> {
        let size = 17 + loc.version * 4;
        let p = determine_perspective(prepared, loc.version, size, &loc)?;

        debug!("PERSPECTIVE {:?}", p);

        let mut start = loc.top_left - 3.0 * p.dy - 3.0 * p.ddy;

        debug!("START {:?}", start);

        let mut data = vec![];

        #[cfg(feature = "debug-images")]
        let mut img = DynamicImage::ImageLuma8(prepared.clone()).to_rgb();

        let mut dy = p.dy - 3.0 * p.ddy;
        let mut dx = p.dx - 3.0 * p.ddx;
        for _ in 0..size {
            let mut line = start - 3.0 * dx;

            for _ in 0..size {
                let x = line.x.round() as u32;
                let y = line.y.round() as u32;
                let pixel = prepared.get_pixel(x, y)[0];

                #[cfg(feature = "debug-images")]
                {
                    if pixel == 0 {
                        for i in max(0, x.saturating_sub(2))..min(img.dimensions().0, x + 2) {
                            for j in max(0, y.saturating_sub(2))..min(img.dimensions().0, y + 2) {
                                img.put_pixel(i, j, Rgb { data: [255, 0, 0] });
                            }
                        }
                    }
                }

                data.push(pixel);
                line = line + dx;
            }
            dx = dx + p.ddx;

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

        Ok(QRData::new(data, loc.version))
    }
}

fn determine_perspective(
    prepared: &GrayImage,
    version: u32,
    size: u32,
    loc: &QRLocation,
) -> Result<Perspective, QRError> {
    let mut dx = loc.top_right - loc.top_left;
    dx = dx / f64::from(size - 7);

    let mut dy = loc.bottom_left - loc.top_left;
    dy = dy / f64::from(size - 7);

    if version == 1 {
        return Ok(Perspective::new(
            dx,
            Delta { dx: 0.0, dy: 0.0 },
            dy,
            Delta { dx: 0.0, dy: 0.0 },
        ));
    }

    let mut est_alignment = Point {
        x: (loc.top_right - 3.0 * dx + f64::from(size - 10) * dy).x,
        y: (loc.bottom_left + f64::from(size - 10) * dx - 3.0 * dy).y,
    };

    let mut found = false;

    'distance: for i in (0..4).rev() {
        'scale: for j in -2..3 {
            let scale = 1.0 + (f64::from(j) / 10.0);

            if i == 0 {
                if is_alignment(prepared, est_alignment, dx, dy, scale) {
                    found = true;
                    break 'distance;
                }

                continue 'scale;
            }

            for x in -i..=i {
                let alignment = est_alignment + f64::from(x) / 2.0 * dx - f64::from(i) / 2.0 * dy;
                if is_alignment(prepared, alignment, dx, dy, scale) {
                    est_alignment = alignment;
                    found = true;
                    break 'distance;
                }

                let alignment = est_alignment + f64::from(x) / 2.0 * dx + f64::from(i) / 2.0 * dy;
                if is_alignment(prepared, alignment, dx, dy, scale) {
                    est_alignment = alignment;
                    found = true;
                    break 'distance;
                }
            }

            for y in -i + 1..i {
                let alignment = est_alignment - f64::from(i) / 2.0 * dx + f64::from(y) / 2.0 * dy;
                if is_alignment(prepared, alignment, dx, dy, scale) {
                    est_alignment = alignment;
                    found = true;
                    break 'distance;
                }

                let alignment = est_alignment + f64::from(i) / 2.0 * dx + f64::from(y) / 2.0 * dy;
                if is_alignment(prepared, alignment, dx, dy, scale) {
                    est_alignment = alignment;
                    found = true;
                    break 'distance;
                }
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
        let mut img = DynamicImage::ImageLuma8(prepared.clone()).to_rgb();

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

    let orig_estimate = Point {
        x: (loc.top_right - 3.0 * dx + f64::from(size - 10) * dy).x,
        y: (loc.bottom_left + f64::from(size - 10) * dx - 3.0 * dy).y,
    };

    debug!("ORIG EST {:?}, NEW EST {:?}", orig_estimate, est_alignment);

    let mut delta = est_alignment - orig_estimate;

    debug!("DELTA {:?}", delta);

    delta = delta / f64::from((size - 10) * (size - 10));

    Ok(Perspective::new(dx, delta, dy, Delta { dx: 0.0, dy: 0.0 }))
}

fn is_alignment(prepared: &GrayImage, p: Point, dx: Delta, dy: Delta, scale: f64) -> bool {
    let dx = scale * dx;
    let dy = scale * dy;

    #[cfg(feature = "debug-images")]
    {
        let mut img = DynamicImage::ImageLuma8(prepared.clone()).to_rgb();

        for i in -2..3 {
            for j in -2..3 {
                let pp = p + f64::from(i) * dx + f64::from(j) * dy;
                img.put_pixel(
                    pp.x.round() as u32,
                    pp.y.round() as u32,
                    Rgb { data: [255, 0, 0] },
                );
            }
        }

        let mut tmp = temp_dir();
        tmp.push("bardecoder-debug-images");
        tmp.push("alignment");

        if let Ok(_) = create_dir_all(tmp.clone()) {
            tmp.push(format!(
                "alignment_p_{}_{}_dx_{}_{}_dy_{}_{}.png",
                p.x, p.y, dx.dx, dx.dy, dy.dx, dy.dy
            ));

            if let Ok(_) = DynamicImage::ImageRgb8(img).save(tmp.clone()) {
                debug!("Debug image with data pixels saved to {:?}", tmp);
            }
        }
    }

    let top_left = p - 2.0 * dx - 2.0 * dy;
    if top_left.x < 0.0 || top_left.y < 0.0 {
        return false;
    }

    let bottom_right = p + 2.0 * dx + 2.0 * dy;
    let dims = prepared.dimensions();
    if bottom_right.x > f64::from(dims.0) || bottom_right.y > f64::from(dims.1) {
        return false;
    }

    for x in -2..2 {
        let twice_up = p - f64::from(x) * dx - 2.0 * dy;
        if prepared.get_pixel(twice_up.x.round() as u32, twice_up.y.round() as u32)[0] == 255 {
            return false;
        }

        let twice_down = p - f64::from(x) * dx + 2.0 * dy;
        if prepared.get_pixel(twice_down.x.round() as u32, twice_down.y.round() as u32)[0] == 255 {
            return false;
        }
    }

    for y in -1..1 {
        let twice_left = p - 2.0 * dx - f64::from(y) * dy;
        if prepared.get_pixel(twice_left.x.round() as u32, twice_left.y.round() as u32)[0] == 255 {
            return false;
        }

        let twice_right = p + 2.0 * dx - f64::from(y) * dy;
        if prepared.get_pixel(twice_right.x.round() as u32, twice_right.y.round() as u32)[0] == 255
        {
            return false;
        }

        let left = p - dx - f64::from(y) * dy;
        if prepared.get_pixel(left.x.round() as u32, left.y.round() as u32)[0] == 0 {
            return false;
        }

        let right = p - dx - f64::from(y) * dy;
        if prepared.get_pixel(right.x.round() as u32, right.y.round() as u32)[0] == 0 {
            return false;
        }
    }

    let up = p - dy;
    if prepared.get_pixel(up.x.round() as u32, up.y.round() as u32)[0] == 0 {
        return false;
    }

    let down = p + dy;
    if prepared.get_pixel(down.x.round() as u32, down.y.round() as u32)[0] == 0 {
        return false;
    }

    prepared.get_pixel(p.x.round() as u32, p.y.round() as u32)[0] == 0
}

#[derive(Debug)]
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
