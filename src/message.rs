use std::path::PathBuf;

use iced::widget::image;
use rkg_utils::header::{
    Controller, GhostType, SlotId, TransmissionMod,
    combo::{Character, Vehicle},
    location::constants::{Country, Subregion},
};

use crate::ui::footer_tab::FooterTab;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CtgpLink {
    Leaderboard,
    Ghost,
    Player,
}

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
    OpenCtgpLink(CtgpLink),
    GetCtgpTrackName,
    CtgpTrackNameLoaded(Option<String>),
    EditFinishTimeChanged(String),
    EditLapSplitChanged(usize, String),
    EditDateChanged(String),
    EditSlotIdSelected(SlotId),
    EditCharacterSelected(Character),
    EditVehicleSelected(Vehicle),
    EditControllerSelected(Controller),
    EditTransmissionModSelected(TransmissionMod),
    EditGhostTypeSelected(GhostType),
    EditAutomaticDriftToggled(bool),
    EditCountrySelected(Country),
    EditSubregionSelected(Subregion),
}
