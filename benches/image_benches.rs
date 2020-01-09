#![cfg_attr(feature = "benchmark", feature(test))]

#[cfg(all(feature = "benchmark", test))]
mod bench {
    extern crate bardecoder;
    extern crate image;
    extern crate test;

    use image::DynamicImage;

    use self::test::Bencher;

    #[bench]
    pub fn version1_example(b: &mut Bencher) {
        let img = image::open("tests/images/version1_example.jpg").unwrap();
        bench_image(&img, b);
    }

    #[bench]
    pub fn version3_example2(b: &mut Bencher) {
        let img = image::open("tests/images/version3_example2.jpg").unwrap();
        bench_image(&img, b);
    }

    #[bench]
    pub fn needs_alignment(b: &mut Bencher) {
        let img = image::open("tests/images/needs_alignment.jpg").unwrap();
        bench_image(&img, b);
    }

    pub fn bench_image(image: &DynamicImage, b: &mut Bencher) {
        let decoder = bardecoder::default_decoder();

        b.iter(|| decoder.decode(image))
    }
}
