use image::{DynamicImage, ImageOutputFormat, Luma};
use qrcode::QrCode;

use crate::encoders::encoder::{Encode, OutputParams};

pub struct QRCode {
    height: u32,
    payload: String,
}

impl QRCode {
    pub fn new(payload: String, height: u32) -> Self {
        QRCode { payload, height }
    }
}

impl Encode for QRCode {
    fn encode(&self) -> Result<DynamicImage, String> {
        let code = QrCode::new(self.payload.as_bytes());
        match code {
            Ok(code) => {
                let mut renderer = code.render::<Luma<u8>>();
                let image = renderer
                    .min_dimensions(self.height, self.height)
                    .quiet_zone(true)
                    .build();
                Ok(image::DynamicImage::ImageLuma8(image))
            }
            Err(error) => Err(error.to_string()),
        }
    }

    fn payload(&self) -> &str {
        &self.payload
    }

    fn get_params(&self) -> OutputParams {
        OutputParams {
            format: ImageOutputFormat::PNG,
            append_text: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let qrcode = QRCode::new("a".to_owned(), 10);
        let image = qrcode.encode().unwrap();
        let expected_bytes = vec![
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 0, 0, 0, 0, 0, 0, 0, 255, 255, 0, 255, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 255, 255,
            255, 255, 255, 255, 255, 255, 0, 255, 255, 255, 255, 255, 0, 255, 0, 255, 0, 0, 255,
            255, 0, 255, 255, 255, 255, 255, 0, 255, 255, 255, 255, 255, 255, 255, 255, 0, 255, 0,
            0, 0, 255, 0, 255, 0, 0, 255, 0, 255, 255, 0, 255, 0, 0, 0, 255, 0, 255, 255, 255, 255,
            255, 255, 255, 255, 0, 255, 0, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 0, 255, 0, 0,
            0, 255, 0, 255, 255, 255, 255, 255, 255, 255, 255, 0, 255, 0, 0, 0, 255, 0, 255, 255,
            0, 255, 255, 0, 255, 0, 255, 0, 0, 0, 255, 0, 255, 255, 255, 255, 255, 255, 255, 255,
            0, 255, 255, 255, 255, 255, 0, 255, 255, 255, 0, 0, 255, 255, 0, 255, 255, 255, 255,
            255, 0, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 255, 0, 255, 0,
            255, 0, 255, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 255, 255, 255, 255, 255, 0, 255, 0,
            255, 0, 0, 255, 0, 0, 255, 255, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            0, 255, 255, 0, 0, 255, 255, 255, 255, 255, 255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 0,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 255, 0, 0, 0, 255, 255, 0, 0, 255,
            0, 255, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0,
            255, 0, 255, 0, 255, 0, 0, 255, 255, 0, 0, 0, 0, 0, 255, 0, 255, 0, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 0, 0, 255, 0, 255, 255, 0, 0, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 0, 0, 255, 255, 0, 255, 255, 255, 255, 255, 0, 255, 0, 255, 255, 255, 255, 255,
            255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 255, 255, 0, 0, 0, 255, 0, 255, 255, 0, 0, 0, 0,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 255, 255, 255, 255, 255, 0, 255, 255,
            255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255,
            255, 0, 255, 0, 0, 0, 255, 0, 255, 255, 0, 0, 0, 255, 0, 255, 255, 0, 0, 0, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 0, 255, 0, 0, 0, 255, 0, 255, 255, 0, 255, 0,
            0, 0, 0, 0, 255, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 255, 0,
            0, 0, 255, 0, 255, 255, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 255, 255, 255, 255,
            255, 255, 255, 255, 0, 255, 255, 255, 255, 255, 0, 255, 255, 255, 0, 0, 0, 0, 0, 0,
            255, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0,
            255, 0, 255, 0, 255, 0, 255, 255, 0, 255, 255, 0, 0, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        ];
        assert_eq!(expected_bytes, image.raw_pixels());
    }
}
