use std::path::PathBuf;

use iced::widget::image;

#[derive(Debug, Clone)]
pub enum Message {
    LoadFile,
    FilePicked(Option<PathBuf>),
    ToggleEditMenu,
    SaveAsFile,
    FileSaved(Option<PathBuf>),
    ToggleFooterView,
    MiiHandleLoaded(Option<image::Handle>),
}
