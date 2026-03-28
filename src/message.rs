use std::path::PathBuf;

use iced::widget::image;

#[derive(Debug, Clone)]
pub enum Message {
    LoadGhost,
    GhostDropped(PathBuf),
    GhostPicked(Option<PathBuf>),
    ToggleEditMenu,
    SaveGhostAsFile,
    GhostSaved(Option<PathBuf>),
    MiiExport,
    MiiImport,
    MiiSelected(Option<PathBuf>),
    MiiSaved(Option<PathBuf>),
    ToggleFooterView,
    MiiHandleLoaded(Option<image::Handle>),
}
