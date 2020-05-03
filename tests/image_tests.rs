use failure::Error;

use bardecoder::{ECLevel, QRInfo};

#[test]
pub fn test_version1_example() {
    test_image(
        "tests/images/version1_example.jpg",
        vec![Ok(String::from("01234567"))],
    );
}

#[test]
pub fn test_version1_example_with_info() {
    test_image_with_info(
        "tests/images/version1_example.jpg",
        vec![Ok((
            String::from("01234567"),
            QRInfo {
                version: 1,
                ec_level: ECLevel::MEDIUM,
                total_data: 128,
                errors: 0,
            },
        ))],
    );
}

#[test]
pub fn test_version1_example_no_border() {
    test_image(
        "tests/images/version1_example_no_border.png",
        vec![Ok(String::from("Ver1"))],
    );
}

#[test]
pub fn test_version1_example_large_border() {
    test_image(
        "tests/images/version1_example_large_border.png",
        vec![Ok(String::from("Ver1"))],
    );
}

#[test]
pub fn test_version1_example_upside_down() {
    test_image(
        "tests/images/version1_example_upside_down.jpg",
        vec![Ok(String::from("01234567"))],
    );
}

#[test]
pub fn test_version1_example2() {
    test_image(
        "tests/images/version1_example2.jpg",
        vec![Ok(String::from("0P1UF3L3016456"))],
    );
}

#[test]
pub fn test_version3_example() {
    test_image(
        "tests/images/version3_example.jpg",
        vec![Ok(String::from(
            "https://payapp.weixin.qq.com/olspree?code_type=2",
        ))],
    );
}

#[test]
pub fn test_version3_example2() {
    test_image(
        "tests/images/version3_example2.jpg",
        vec![Ok(String::from("http://www.prolinepetfood.com/1/"))],
    );
}

#[test]
pub fn test_version4_example() {
    test_image(
        "tests/images/version4_example.jpg",
        vec![Ok(String::from(
            "http://m.langnese-honing.nl/index.php?id=1870",
        ))],
    );
}

#[test]
pub fn test_needs_alignment() {
    test_image(
        "tests/images/needs_alignment.jpg",
        vec![Ok(String::from("http://cblink.je/app-install-display-nl"))],
    );
}

#[test]
pub fn test_needs_alignment_with_info() {
    test_image_with_info(
        "tests/images/needs_alignment.jpg",
        vec![Ok((
            String::from("http://cblink.je/app-install-display-nl"),
            QRInfo {
                version: 3,
                ec_level: ECLevel::LOW,
                total_data: 440,
                errors: 3,
            },
        ))],
    );
}

#[test]
pub fn test_multiple_codes() {
    test_image(
        "tests/images/multiple_codes.png",
        vec![
            Ok(String::from("http://www.prolinepetfood.com/1/")),
            Ok(String::from("Ver1")),
        ],
    );
}

#[test]
pub fn test_multiple_codes_with_info() {
    test_image_with_info(
        "tests/images/multiple_codes.png",
        vec![
            Ok((
                String::from("http://www.prolinepetfood.com/1/"),
                QRInfo {
                    version: 3,
                    ec_level: ECLevel::MEDIUM,
                    total_data: 352,
                    errors: 0,
                },
            )),
            Ok((
                String::from("Ver1"),
                QRInfo {
                    version: 1,
                    ec_level: ECLevel::HIGH,
                    total_data: 72,
                    errors: 0,
                },
            )),
        ],
    );
}

#[test]
pub fn test_wikipedia_examples() {
    // Downloaded from https://en.wikipedia.org/wiki/QR_code
    test_image(
        "tests/images/wikipedia/version1_example.png",
        vec![Ok(String::from("Ver1"))],
    );

    test_image(
        "tests/images/wikipedia/version2_example.png",
        vec![Ok(String::from("Version 2"))],
    );

    test_image(
        "tests/images/wikipedia/version3_example.png",
        vec![Ok(String::from("Version 3 QR Code"))],
    );

    test_image(
        "tests/images/wikipedia/version4_example.png",
        vec![Ok(String::from("Version 4 QR Code, up to 50 char"))],
    );

    test_image(
        "tests/images/wikipedia/version10_example.png",
        vec![Ok(String::from(
            "VERSION 10 QR CODE, UP TO 174 CHAR AT H LEVEL, WITH 57X57 MODULES AND PLENTY OF ERROR CORRECTION TO GO AROUND.  NOTE THAT THERE ARE ADDITIONAL TRACKING BOXES",
        ))],
    );

    let version_25_40_text = String::from("Version 40 QR Code can contain up to 1852 chars.\nA QR code (abbreviated from Quick Response code) is a type of matrix barcode (or two-dimensional code) that is designed to be read by smartphones. The code consists of black modules arranged in a square pattern on a white background. The information encoded may be text, a URL, or other data.\nCreated by Toyota subsidiary Denso Wave in 1994, the QR code is one of the most popular types of two-dimensional barcodes. The QR code was designed to allow its contents to be decoded at high speed.\nThe technology has seen frequent use in Japan and South Korea; the United Kingdom is the seventh-largest national consumer of QR codes.\nAlthough initially used for tracking parts in vehicle manufacturing, QR codes now are used in a much broader context, including both commercial tracking applications and convenience-oriented applications aimed at mobile phone users (termed mobile tagging). QR codes may be used to display text to the user, to add a vCard contact to the user\'s device, to open a Uniform Resource Identifier (URI), or to compose an e-mail or text message. Users can generate and print their own QR codes for others to scan and use by visiting one of several paid and free QR code generating sites or apps.\n");
    test_image(
        "tests/images/wikipedia/version25_example.png",
        vec![Ok(version_25_40_text.clone())],
    );

    test_image(
        "tests/images/wikipedia/version40_example.png",
        vec![Ok(version_25_40_text)],
    );
}

pub fn test_image(file: &str, expected: Vec<Result<String, Error>>) {
    let img = image::open(file).unwrap();

    let decoder = bardecoder::default_decoder();
    let result = decoder.decode(&img);

    assert_eq!(expected.len(), result.len());

    for (expected, result) in expected.into_iter().zip(result) {
        assert!(expected.is_ok());
        assert!(result.is_ok());
        assert_eq!(expected.unwrap(), result.unwrap());
    }
}

pub fn test_image_with_info(file: &str, expected: Vec<Result<(String, QRInfo), Error>>) {
    let img = image::open(file).unwrap();

    let decoder = bardecoder::default_decoder_with_info();
    let result = decoder.decode(&img);

    assert_eq!(expected.len(), result.len());

    for (expected, result) in expected.into_iter().zip(result) {
        assert!(expected.is_ok());
        assert!(result.is_ok());
        assert_eq!(expected.unwrap(), result.unwrap());
    }
}
