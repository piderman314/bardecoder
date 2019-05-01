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

## Background

## How to use

### Quick
The quickest way to integrate is to use the built-in default decoder. This will work for the vast majority of cases, though please keep in mind the [Tips](#tips) below.

``` rust
use bardecoder;
use image;

fn main() {
    let img = image::open("<<image location>>").unwrap();

    // Use default decoder
    let decoder = bardecoder::default_decoder();

    let results = decoder.decode(img);
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

    let results = decoder.decode(img);
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

    let results = decoder.decode(img);
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

## Contributing