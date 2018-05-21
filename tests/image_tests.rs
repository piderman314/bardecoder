extern crate bardecoder;
extern crate image;

#[test]
pub fn test_version1_example() {
    let img = image::open("tests/images/version1_example.jpg").unwrap();

    let decoder = bardecoder::default_decoder();
    let result = decoder.decode(&img);

    assert_eq!("01234567", result);
}
