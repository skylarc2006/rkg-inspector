use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Message {
    LoadFile,
    FilePicked(Option<PathBuf>),
    ToggleEditMenu,
    SaveAsFile,
    FileSaved(Option<PathBuf>),
}
