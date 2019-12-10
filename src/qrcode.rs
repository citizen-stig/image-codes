use crate::encoder::{Encoder, OutputParams};
use image::{DynamicImage, ImageOutputFormat, Luma};
use qrcode::QrCode;



pub struct QRCode {
    height: u32,
    payload: String,
}

impl QRCode {
    pub fn new(payload: String) -> Self {
        QRCode { payload, height: 300 }
    }
}

impl Encoder for QRCode {
    fn encode(&self) -> DynamicImage {
        let code = QrCode::new(self.payload.as_bytes()).unwrap();
        let mut renderer = code.render::<Luma<u8>>();
        renderer.min_dimensions(self.height, self.height);
        image::DynamicImage::ImageLuma8(renderer.build())
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