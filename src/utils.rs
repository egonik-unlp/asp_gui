use eframe::IconData;

pub fn load_icon(path: &str) -> eframe::IconData {
    let img = image::open(path).expect("Cannot open image");
    let rgba_im = img.as_rgba8().unwrap();
    let (width, height) = rgba_im.dimensions();
    IconData {
        rgba: rgba_im.as_raw().to_vec(),
        width: width,
        height: height,
    }
}
