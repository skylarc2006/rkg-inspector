use std::path::PathBuf;

use iced::widget::{image, text_editor};

use crate::ui::footer_tab::FooterTab;

#[derive(Debug, Clone)]
pub enum Message {
    LoadGhost,
    GhostDropped(PathBuf),
    GhostPicked(Option<PathBuf>),
    ToggleEditMenu,
    ToggleFooterInfoMenu,
    SaveGhostAsFile,
    GhostSaved(Option<PathBuf>),
    MiiExport,
    MiiImport,
    MiiSelected(Option<PathBuf>),
    MiiSaved(Option<PathBuf>),
    MiiHandleLoaded(Option<image::Handle>),
    SetActiveFooterTab(FooterTab),
    CtgpIdentityTextAction(text_editor::Action),
}
