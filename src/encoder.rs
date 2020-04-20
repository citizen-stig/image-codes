use image::{DynamicImage, ImageOutputFormat};

pub struct OutputParams {
    pub format: ImageOutputFormat,
    pub append_text: bool,
}

pub trait Encode {
    fn encode(&self) -> DynamicImage;

    fn payload(&self) -> &str;

    fn get_params(&self) -> OutputParams;

    fn output(&self) -> Vec<u8> {
        let params = self.get_params();
        let img = self.encode();

        let mut output_image_bytes: Vec<u8> = Vec::new();
        img.write_to(&mut output_image_bytes, params.format)
            .expect("Unable to write image");
        output_image_bytes
    }
}
