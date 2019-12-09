use image::{DynamicImage, ImageOutputFormat};
use imageproc::drawing::draw_text_mut;
use rusttype::{FontCollection, Scale};

pub struct OutputParams {
    pub format: ImageOutputFormat,
    pub append_text: bool,
}

pub trait Encoder {
    fn encode(&self) -> DynamicImage;

    fn payload(&self) -> &str;

    fn params(&self) -> OutputParams;

    fn append_text_to_bottom(&self, img: &mut DynamicImage) {
        let font = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
        let font = FontCollection::from_bytes(font)
            .unwrap()
            .into_font()
            .unwrap();
        let scale = Scale {
            x: 20.0,
            y: 20.0,
        };
        draw_text_mut(
            img,
            image::Rgba([0u8, 0u8, 0u8, 0u8]),
            0,
            0,
            scale,
            &font,
            self.payload(),
        );
    }

    fn output(&self) -> Vec<u8> {
        let params = self.params();
        let mut img = self.encode();

        if params.append_text {
            self.append_text_to_bottom(&mut img);
        }

        let mut output_image_bytes: Vec<u8> = Vec::new();
        img.write_to(&mut output_image_bytes, params.format)
            .expect("Unable to write image");
        output_image_bytes
    }
}