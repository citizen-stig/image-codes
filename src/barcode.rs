use crate::encoder::{Encoder, OutputParams};
use image::{DynamicImage, ImageOutputFormat};
use barcoders::generators::image::Image as BarCodeImage;
use barcoders::sym::code128::Code128;

pub struct BarCode {
    payload: String,
}

impl BarCode {
    pub fn new(payload: String) -> Self {
        BarCode { payload }
    }
}

impl Encoder for BarCode {
    fn encode(&self) -> DynamicImage {
        let data = "À".to_owned() + &self.payload;
        let barcode = Code128::new(data).unwrap();
        let encoded = barcode.encode();
        let buffer = BarCodeImage::image_buffer(200);
        image::DynamicImage::ImageRgba8(buffer.generate_buffer(&encoded[..]).unwrap())
    }

    fn payload(&self) -> &str {
        &self.payload
    }

    fn params(&self) -> OutputParams {
        OutputParams {
            format: ImageOutputFormat::PNG,
            append_text: true,
        }
    }
}
