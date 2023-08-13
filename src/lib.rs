use base64::{decode, encode};
use image::load_from_memory;
use image::ImageOutputFormat::Png;
use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log; //log_1 print one value

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    log(&"Grayscale called".into());

    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"image decoded".into());

    let mut image: image::DynamicImage = load_from_memory(&base64_to_vector).unwrap();
    log(&"image loaded".into());

    image = image.grayscale();
    log(&"Grayscale effect applaied".into());

    let mut buffer = vec![];
    image.write_to(&mut buffer, Png).unwrap();
    log(&"New image wretten".into());

    let encoded_image = encode(&buffer);
    let data_url = format!(
        "data:image/png;base64,{}",
        encoded_image
    );

    data_url
}
