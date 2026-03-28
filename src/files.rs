use std::path::PathBuf;

pub async fn pick_file(filter_name: &str, extensions: &[&str]) -> Option<PathBuf> {
    rfd::AsyncFileDialog::new()
        .add_filter(filter_name, extensions)
        .pick_file()
        .await
        .map(|fh| fh.path().to_path_buf())
}

pub async fn save_as_file(default_file_name: String, filter_name: &str, extensions: &[&str]) -> Option<PathBuf> {
    rfd::AsyncFileDialog::new()
        .set_file_name(default_file_name)
        .add_filter(filter_name, extensions)
        .save_file()
        .await
        .map(|fh| fh.path().to_path_buf())
}
