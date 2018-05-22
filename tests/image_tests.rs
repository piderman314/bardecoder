extern crate bardecoder;
extern crate image;

use bardecoder::qr::QRError;

#[test]
pub fn test_version1_example() {
    test_image(
        "tests/images/version1_example.jpg",
        vec![Ok(String::from("01234567"))],
    );
}

#[test]
pub fn test_version1_example_upside_down() {
    test_image(
        "tests/images/version1_example_upside_down.jpg",
        vec![Ok(String::from("01234567"))],
    );
}

pub fn test_image(file: &str, expected: Vec<Result<String, QRError>>) {
    let img = image::open(file).unwrap();

    let decoder = bardecoder::default_decoder();
    let result = decoder.decode(&img);

    assert_eq!(expected, result);
}
