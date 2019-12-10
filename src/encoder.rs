use image::{DynamicImage, ImageOutputFormat, GenericImageView, ImageBuffer, Pixel};
use image::imageops::overlay;
use imageproc::drawing::draw_text;
use rusttype::{FontCollection, Scale};

pub struct OutputParams {
    pub format: ImageOutputFormat,
    pub append_text: bool,
}

pub trait Encoder {
    fn encode(&self) -> DynamicImage;

    fn payload(&self) -> &str;

    fn params(&self) -> OutputParams;

    fn append_text_to_bottom(&self, img: &DynamicImage) -> DynamicImage {
        let font = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
        let font = FontCollection::from_bytes(font)
            .unwrap()
            .into_font()
            .unwrap();
        let (img_width, img_height ) = img.dimensions();

        let font_size: f32 = img_height as f32 * 0.2;
        let mut img2 = ImageBuffer::new(img_width, img_height + (font_size as u32));
        overlay(&mut img2, img, 0, 0);

        let scale = Scale { x: font_size, y: font_size };


        let x = 0;
        let y = img_height;

        DynamicImage::ImageRgba8(draw_text(
            &mut img2,
            image::Rgba([0u8, 0u8, 0u8, 0u8]),
            0,
            y,
            scale,
            &font,
            self.payload(),
        ))
    }

    fn output(&self) -> Vec<u8> {
        let params = self.params();
        let mut img = self.encode();

        if params.append_text {
            img = self.append_text_to_bottom(&mut img);
        }

        let mut output_image_bytes: Vec<u8> = Vec::new();
        img.write_to(&mut output_image_bytes, params.format)
            .expect("Unable to write image");
        output_image_bytes
    }
}