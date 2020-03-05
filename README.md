# Bardecoder

Detect and decode QR Codes, written in 100% Rust.

* [Background](#background)
* [How to use](#how-to-use)
    * [Quick](#quick)
    * [Modified](#modified)
    * [Advanced](#advanced)
* [Tips](#tips)
* [Features](#features)
* [Support](#support)
* [Contributing](#contributing)

[![Travis Build](https://travis-ci.com/piderman314/bardecoder.svg?branch=master)](https://travis-ci.com/piderman314/bardecoder)
[![License](https://img.shields.io/github/license/piderman314/bardecoder.svg?color=success)](https://github.com/piderman314/bardecoder/blob/master/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/bardecoder.svg)](https://crates.io/crates/bardecoder)
[![docs.rs](https://docs.rs/bardecoder/badge.svg)](https://docs.rs/bardecoder)
[![Rustc version](https://img.shields.io/badge/rustc-1.40%2B-informational.svg)](https://www.rust-lang.org/)

## Background

This library came about after perusing the [Not Yet Awesome Rust](https://github.com/not-yet-awesome-rust/not-yet-awesome-rust) list. It strives to be modular so algorithms with different strengths, speeds and robustness can be used interchangeably.

## How to use

Add the following to your `Cargo.toml`:

``` toml
[dependencies]
bardecoder = "0.2.1"
image = "0.22"
```

### Quick
The quickest way to integrate is to use the built-in default decoder. This will work for the vast majority of cases, though please keep in mind the [Tips](#tips) below.

``` rust
fn main() {
    let img = image::open("<<image location>>").unwrap();

    // Use default decoder
    let decoder = bardecoder::default_decoder();

    let results = decoder.decode(&img);
    for result in results {
        println!("{}", result.unwrap());
    }
}
```

### Modified
If you want a little customizability, you can start with the default builder instead. It will be pre-populated with the default components but you are free to replace any of them with modified parameters. 

``` rust
use bardecoder;
use bardecoder::prepare::BlockedMean;

use image;

fn main() {
    let img = image::open("<<image location>>").unwrap();

    // Use default decoder builder
    let mut db = bardecoder::default_builder();

    // Use some different arguments in one of the default components
    db.prepare(Box::new(BlockedMean::new(7, 9)));

    // Build the actual decoder
    let decoder = db.build();

    let results = decoder.decode(&img);
    for result in results {
        println!("{}", result.unwrap());
    }
}
```

You can also start with a completely empty builder but be aware that the `build()` function will `Panic!` if any of the components are missing.

``` rust
use bardecoder::DecoderBuilder;

let mut decoder_builder = DecoderBuilder::new();
```

### Advanced
If you want to go absolutely nuts, you can also provide your own implementations for the various components. Use at your own risk!

``` rust
use bardecoder;
use bardecoder::prepare::BlockedMean;
use bardecoder::detect::{Detect, Location};

use image;
use image::GrayImage;

struct MyDetector {}

impl MyDetector {
    pub fn new() -> MyDetector {
        MyDetector {}
    }
}

impl Detect<GrayImage> for MyDetector {
    fn detect(&self, prepared: &GrayImage) -> Vec<Location> {
        vec![]
    }
}

fn main() {
    let img = image::open("<<image location>>").unwrap();

    // Use default decoder builder
    let mut db = bardecoder::default_builder();

    // Use some different arguments in one of the default components
    db.prepare(Box::new(BlockedMean::new(7, 9)));

    // Use your homemade Detector!
    db.detect(Box::new(MyDetector::new()));

    // Build the actual decoder
    let decoder = db.build();

    let results = decoder.decode(&img);
    for result in results {
        println!("{}", result.unwrap());
    }
}
```

## Tips
Though this library can handle all sorts of QR images, here are some tips for optimal results:

* Keep the resolution of the source image low-ish, say between 400x300 and 800x600 pixels. Any higher and it takes quite long to detect any codes.
* Keep the QR code centered and zoomed in.
* Keep the QR code free of errors, deliberate or otherwise. While QR codes are self-correcting, the actual correction is not cheap. However before starting that process it is easy to detect that a QR code is error free so in that case an early exit is taken.

## Features

`Bardecoder` exposes the following features for use in your project:

* `debug-images` : Some of the default components will output debug images in the  `<tmp>/bardecoder-debug-images` folder, where `<tmp>` is the default OS temp folder. This can help show visually what the algorithms are doing. Be aware that some of the components (for example `QRExtractor`) output a *lot* of images so definitely do not use this feature other than to have a look what is happening when things are going wrong.

* `fail-on-warnings` : if you fancy that sort of thing, though its purpose is mostly for `travis-ci`.

## Support

If you find an image with a QR code that this library is unable to decode, please raise an [Issue](https://github.com/piderman314/bardecoder/issues). Please include the image and the code you are trying to decode it with (especially when using the [Modified](#modified) method). I will try my best improve the algorithm though I cannot 100% guarantee that I will succeed, especially with more esoteric QR codes.

## Contributing

If you find a small bug and manage to fix it yourself, please feel free to submit a pull request. For larger refactorings and more fundamental issues please submit a [ticket](https://github.com/piderman314/bardecoder/issues) outlining the problem and potential solution.