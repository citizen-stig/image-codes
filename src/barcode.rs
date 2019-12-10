use crate::encoder::{Encoder, OutputParams};
use image::{DynamicImage, ImageOutputFormat};
use barcoders::generators::image::{Image, Rotation, Color};
use barcoders::sym::code128::Code128;
use barcoders::sym::code39::Code39;
use barcoders::sym::code93::Code93;
use barcoders::sym::codabar::Codabar;

enum Symbology {
    Code128,
    Code39,
    Code93,
    Codabar,
}

pub struct BarCode {
    height: u32,
    payload: String,
    symbology: Symbology
}

impl BarCode {
    pub fn new(payload: String) -> Self {
        BarCode { payload, height: 200, symbology: Symbology::Code93 }
    }
    

    fn xdim(&self) -> u32 {
        // TODO: Define xdim based on payload size as well, so barcode looks pretty
        match self.height {
            0...100 => 1,
            101...200 => 2,
            201...300 => 3,
            301...400 => 4,
            _ => 5
        }
    }

    fn get_bytes(&self) -> Vec<u8> {
        let mut data =  &self.payload;
        match self.symbology {
            Symbology::Code128 => {
                let data = "À".to_owned() + data;
                Code128::new(data).unwrap().encode()
            },
            Symbology::Code39 => Code39::new(data).unwrap().encode(),
            Symbology::Code93 => Code93::new(data).unwrap().encode(),
            Symbology::Codabar => Codabar::new(data).unwrap().encode(),
        }
    }
}

impl Encoder for BarCode {
    fn encode(&self) -> DynamicImage {
        let encoded = self.get_bytes();
        let buffer = Image::ImageBuffer {
            height: self.height,
            xdim: self.xdim(),
            rotation: Rotation::Zero,
            foreground: Color::black(),
            background: Color::white(),
        };
        image::DynamicImage::ImageRgba8(buffer.generate_buffer(&encoded[..]).unwrap())
        // TODO: add quite zone
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
