extern crate barcoders;
extern crate base64;
extern crate image;
extern crate qrcode;

use barcoders::sym::code128::Code128;
use base64::encode;
use image::png;
use image::ColorType;
use image::DynamicImage;
use image::ImageBuffer;
use image::Luma;
use image::Rgba;
use qrcode::QrCode;
use std::io::Write;

pub fn encode_barcode128(payload: &String) -> String {
    let data = "À".to_owned() + payload;
    println!("Going to encode: {:?}", data);
    let barcode = Code128::new(data).unwrap();
    //    let img = BarCodeImage::png(80);
    //    // TODO: Add payload as text at the bottom.
    //
    //    let encoded: Vec<u8> = barcode.encode();
    //    let bytes = image.generate(&encoded[..]).unwrap();
    //    encode(&bytes)
    "HEELO".to_owned()
}

pub fn encode_qrcode(payload: &String) -> String {
    let code = QrCode::new(payload.as_bytes()).unwrap();
    let renderer = code.render::<Luma<u8>>();
    let img = renderer.build();
    let img2 = image::DynamicImage::ImageLuma8(img);
    let mut buf = Vec::new();
    img2.write_to(&mut buf, image::ImageOutputFormat::PNG)
        .expect("Unable to write");

    encode(&buf[..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qr_code_simple() {
        let payload = "TEST".to_owned();
        let encoded = encode_qrcode(&payload);
    }
}
