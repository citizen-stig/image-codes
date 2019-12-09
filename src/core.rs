extern crate barcoders;
extern crate base64;
extern crate image;
extern crate qrcode;

use barcoders::sym::code128::Code128;
use barcoders::generators::image::Image as BarCodeImage;
use imageproc::drawing::draw_text_mut;
use rusttype::{FontCollection, Scale};
use base64::encode;
use image::Luma;
use qrcode::QrCode;
use image::{DynamicImage, ImageOutputFormat, FilterType, GenericImageView};

fn encode_as_base64(image_object: DynamicImage, format: ImageOutputFormat) -> String {
    let mut output_image_bytes: Vec<u8> = Vec::new();
    image_object.write_to(&mut output_image_bytes, format).expect("Unable to write image");
    encode(&output_image_bytes[..])
}

pub fn encode_barcode128(payload: &str, height: u32) -> String {
    let data = "À".to_owned() + payload;
    println!("Going to encode: {:?}", data);

    let barcode = Code128::new(data).unwrap();
    let encoded = barcode.encode();
    let buffer = BarCodeImage::image_buffer(height);
    let mut img = image::DynamicImage::ImageRgba8(
        buffer.generate_buffer(&encoded[..]).unwrap()
    );
    if img.width() < height {
        img = img.resize_exact(height * 3, height, FilterType::Nearest)
    }
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
        &mut img,
        image::Rgba([255u8, 255u8, 255u8, 0u8]),
        0,
        0,
        scale,
        &font,
        payload,
    );
    encode_as_base64(img, ImageOutputFormat::PNG)
}

pub fn encode_qrcode(payload: &str, height: u32) -> String {
    let code = QrCode::new(payload.as_bytes()).unwrap();
    let mut renderer = code.render::<Luma<u8>>();
    renderer.min_dimensions(height, height);
    let img = image::DynamicImage::ImageLuma8(renderer.build());
    encode_as_base64(img, ImageOutputFormat::PNG)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qr_code_simple() {
        let payload = "TEST".to_owned();
        let actual = encode_qrcode(&payload);
        let expected = "iVBORw0KGgoAAAANSUhEUgAAAOgAAADoCAAAAADAwvekAAAFOElEQVR4nO3PMY4kRwx\
        EUe39Dy3Jec4HiMya7nFq+YEyghFkVvz595+/gy36Nrbo29iib2OLvo0t+ja26NvYom9ji76NLfo2tujb2KJvY4u+\
        jS36Nrbo29iib2OL4s//309w1351Ofkn7E9sUXz6sP3qcvJP2J/Yovj0YfvV5eSfsD+xRfHpw/ary8k/YX9ii8LDpx\
        ymvDnqQ27yy21+i+L2EKa8OepDbvLLbX6L4vYQprw56kNu8sttfovi9hCmvDnqQ27yy21+i6KH6FKffkr36VKfntii\
        6CG61Kef0n261Kcntih6iC716ad0ny716Yktih6iS336Kd2nS316Youih+hSf9InmqdLfXpii6KH6FJ/0ieap0t9em\
        KLoofoUn/SJ5qnS316Youih+hSf9InmqdLfXpii6KH6FJ/0qX+pEt9emKLoofoUn/Spf6kS316Youih+hSf9Kl/qRL\
        fXpii6KH6FJ/0qX+pEt9emKL4vYQmq/G0/nEbX6L4vYQmq/G0/nEbX6L4vYQmq/G0/nEbX6L4vYQmq/G0/nEbX6Lwq\
        GnuGv/qX6K/Yktik8ftv9UP8X+xBbFpw/bf6qfYn9ii+LTh+0/1U+xP7FFv4Uf9w5d+L/FFv0WinmHLvzfYot+C8W8\
        Qxf+b7FFv4Vi3qEL/7fYovBjU64+XfiQM6dPyMNe52WL4nSoPl34kDOnT8jDXudli+J0qD5d+JAzp0/Iw17nZYvidK\
        g+XfiQM6dPyMNe52WLwiFMeTl+deGjOX7nP2WLwoOY8nL86sJHc/zOf8oWhQcx5eX41YWP5vid/5QtCg9iysvxqwsf\
        zfE7/ylbFNOD5k9xxz4N81u6P7FF4eHmzJ/ijn0a5rd0f2KLwsPNmT/FHfs0zG/p/sQWhYebM3+KO/ZpmN/S/Yktek\
        t/zL3Oixya509z8DsvW/QWD8G9zoscmudPc/A7L1v0Fg/Bvc6LHJrnT3PwOy9b9BYPwb3Oixya509z8DsvWxQOQb7z\
        Igf5aV5uc2i+bFH0AfnOixzkp3m5zaH5skXRB+Q7L3KQn+blNofmyxZFH5DvvMhBfpqX2xyaL1v0W0w/6N2TX+T51R\
        Nb9Fv4keLdk1/k+dUTW/Rb+JHi3ZNf5PnVE1v0W/iR4t2TX+T51RNbFA49xd3um+PkozlM+bJFMT1wwt3um+PkozlM\
        +bJFMT1wwt3um+PkozlM+bJFMT1wwt3um+PkozlM+bJF4YFTDqc8vzQvZ16NaV62KG4P4ZTnl+blzKsxzcsWxe0hnP\
        L80ryceTWmedmiuD2EU55fmpczr8Y0L1sUPUSX+tXonIZ5kTv5E1sUDsvRpX41OqdhXuRO/sQWhcNydKlfjc5pmBe5\
        kz+xReGwHF3qV6NzGuZF7uRPbFE4LEeX+vSEHJrnm08a5hNbFA7K0aU+PSGH5vnmk4b5xBaFg3J0qU9PyKF5vvmkYT\
        6xReGgHF3q0xNyaJ5vPmmYT2xROChHl/p04aO5+hPdO7FF4WE5utSnCx/N1Z/o3oktCg/L0aU+Xfhorv5E905sUXhY\
        ji716cJHc/Unundii8LDpxym/DQHH3Lmt3pii+L2EKb8NAcfcua3emKL4vYQpvw0Bx9y5rd6Yovi9hCm/DQHH3Lmt3\
        pii8Khp7hrny788q08tiimwyfctU8XfvlWHlsU0+ET7tqnC798K48tiunwCXft04VfvpXHFn0bW/RtbNG3sUXfxhZ9\
        G1v0bWzRt7FF38YWfRtb9G1s0bexRd/GFn0bW/RtbNG3sUXfxl9T9D8UUWg9gqJo9wAAAABJRU5ErkJggg==";
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_bar_code() {
        let payload = "TEST".to_owned();
        let actual = encode_barcode128(&payload);
        eprintln!("{}", actual);
    }
}
