use std::path::PathBuf;

pub async fn pick_file() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .add_filter("Mario Kart Wii ghosts", &["rkg"])
        .pick_file()
}

pub async fn save_as_file(default_file_name: String) -> Option<PathBuf> {
    rfd::FileDialog::new()
        .set_file_name(default_file_name)
        .add_filter("Mario Kart Wii ghosts", &["rkg"])
        .save_file()
}
