#![cfg_attr(feature = "benchmark", feature(test))]

#[cfg(all(feature = "benchmark", test))]
mod bench {
    extern crate bardecoder;
    extern crate image;
    extern crate test;

    use self::test::Bencher;

    #[bench]
    pub fn version1_example(b: &mut Bencher) {
        bench_image("tests/images/version1_example.jpg", b);
    }

    #[bench]
    pub fn version3_example2(b: &mut Bencher) {
        bench_image("tests/images/version3_example2.jpg", b);
    }

    #[bench]
    pub fn needs_alignment(b: &mut Bencher) {
        bench_image("tests/images/needs_alignment.jpg", b);
    }

    pub fn bench_image(file: &str, b: &mut Bencher) {
        let img = image::open(file).unwrap();
        let decoder = bardecoder::default_decoder();

        b.iter(|| decoder.decode(img.clone()))
    }
}
