use eframe::IconData;
use image::ImageFormat::Png;

pub fn load_icon() -> eframe::IconData {
    let bytes = include_bytes!("../assets/icon.png");
    let from_bytes = image::load_from_memory_with_format(bytes, Png).expect("Cannot load image");
    let rgba_im = from_bytes.as_rgba8().unwrap();
    let (width, height) = rgba_im.dimensions();
    IconData {
        rgba: rgba_im.as_raw().to_vec(),
        width: width,
        height: height,
    }
}
