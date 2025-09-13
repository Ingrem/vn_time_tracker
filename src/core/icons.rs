use eframe::egui::IconData;

/// Load window icon from `icon.ico`, included to binary file.
pub fn load_icon_from_bytes() -> IconData {
    let icon_bytes = include_bytes!("../../resources/icon.ico");
    let image = image::load_from_memory(icon_bytes).unwrap().into_rgba8();
    let (width, height) = image.dimensions();

    IconData { rgba: image.into_vec(), width, height }
}
