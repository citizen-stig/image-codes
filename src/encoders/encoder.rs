use image::{DynamicImage, ImageOutputFormat};

pub struct OutputParams {
    pub format: ImageOutputFormat,
    pub append_text: bool,
}

pub trait Encode {
    fn encode(&self) -> Result<DynamicImage, String>;

    fn payload(&self) -> &str;

    fn get_params(&self) -> OutputParams;

    fn output(&self) -> Result<Vec<u8>, String> {
        let params = self.get_params();
        let img = self.encode()?;

        let mut output_image_bytes: Vec<u8> = Vec::new();
        match img.write_to(&mut output_image_bytes, params.format) {
            Ok(_) => Ok(output_image_bytes),
            Err(err) => Err(err.to_string()),
        }
    }
}
