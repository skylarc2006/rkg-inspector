use std::path::PathBuf;

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => ' ',
            _ => c,
        })
        .collect()
}

pub async fn pick_file(filter_name: &str, extensions: &[&str]) -> Option<PathBuf> {
    rfd::AsyncFileDialog::new()
        .add_filter(filter_name, extensions)
        .pick_file()
        .await
        .map(|fh| fh.path().to_path_buf())
}

pub async fn pick_files(filter_name: &str, extensions: &[&str]) -> Vec<PathBuf> {
    rfd::AsyncFileDialog::new()
        .add_filter(filter_name, extensions)
        .pick_files()
        .await
        .into_iter()
        .flatten()
        .map(|fh| fh.path().to_path_buf())
        .collect()
}

pub async fn save_as_file(
    default_file_name: String,
    filter_name: &str,
    extensions: &[&str],
) -> Option<PathBuf> {
    let default_file_name = sanitize_filename(&default_file_name);
    rfd::AsyncFileDialog::new()
        .set_file_name(default_file_name)
        .add_filter(filter_name, extensions)
        .save_file()
        .await
        .map(|fh| fh.path().to_path_buf())
}
