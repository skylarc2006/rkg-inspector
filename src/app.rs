use iced::widget::image;
use iced::{Element, Length, Task, Theme, widget::stack};
use rkg_utils::Ghost;
use rkg_utils::header::slot_id::SlotId;
use std::path::PathBuf;

use crate::files::{pick_file, save_as_file};
use crate::ui::{assets, image_handles, widgets};

#[derive(Debug, Clone)]
pub enum Message {
    LoadFile,
    FilePicked(Option<PathBuf>),
    ToggleEditMenu,
    SaveAsFile,
    FileSaved(Option<PathBuf>),
    SlotIdSelected(SlotId),
}

pub struct RkgInspector {
    pub active_ghost: Option<Ghost>,
    pub background_handle: image::Handle,
    pub ghost_box_handle: image::Handle,
    pub selected_slot_id: Option<SlotId>,
    pub edit_menu_active: bool,
}

impl RkgInspector {
    pub fn new() -> Self {
        Self {
            active_ghost: None,
            background_handle: image::Handle::from_bytes(assets::BACKGROUND),
            ghost_box_handle: image::Handle::from_bytes(assets::GHOST_BOX),
            selected_slot_id: None,
            edit_menu_active: false,
        }
    }

    pub fn title(&self) -> String {
        String::from("RKG Inspector")
    }

    pub fn theme(&self) -> Theme {
        Theme::Dark
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::LoadFile => Task::perform(pick_file(), Message::FilePicked),

            Message::FilePicked(path) => {
                self.active_ghost = path.and_then(|p| Ghost::new_from_file(&p).ok());
                self.selected_slot_id = self.active_ghost.as_ref().map(|g| g.header().slot_id());
                Task::none()
            }

            Message::ToggleEditMenu => Task::none(),

            Message::SaveAsFile => {
                if let Some(ghost) = &self.active_ghost {
                    let finish_time = ghost.header().finish_time();
                    let time = format!(
                        "{:02}m{:02}s{:03}",
                        finish_time.minutes(),
                        finish_time.seconds(),
                        finish_time.milliseconds()
                    );
                    let mii_name = ghost.header().mii().name();
                    let crc32_bytes = ghost.file_crc32().to_be_bytes();
                    let crc32 = format!(
                        "{:02x}{:02x}{:02x}{:02x}",
                        crc32_bytes[0], crc32_bytes[1], crc32_bytes[2], crc32_bytes[3],
                    );

                    let default_file_name = format!("{}_{}_{}.rkg", time, mii_name, crc32);
                    Task::perform(save_as_file(default_file_name), Message::FileSaved)
                } else {
                    Task::none()
                }
            }

            Message::FileSaved(path) => {
                if let Some(ghost) = &mut self.active_ghost {
                    path.and_then(|p| ghost.save_to_file(&p).ok());
                }
                Task::none()
            }

            Message::SlotIdSelected(slot_id) => {
                if let Some(ghost) = &mut self.active_ghost {
                    ghost.header_mut().set_slot_id(slot_id);
                    self.selected_slot_id = Some(slot_id);
                }
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let background = widgets::background(
            self.background_handle.clone(),
            self.ghost_box_handle.clone(),
        );
        let rkg_inspector_text = widgets::rkg_inspector_text();
        let select_ghost_button = widgets::select_ghost_button();
        let toggle_edit_button = widgets::toggle_edit_button(self.active_ghost.is_some());
        let save_as_button = widgets::save_as_button(self.active_ghost.is_some());

        let track_name_text = self
            .selected_slot_id
            .as_ref()
            .map(|slot_id| widgets::track_name_text(slot_id));

        let finish_time_text = self
            .active_ghost
            .as_ref()
            .map(|g| widgets::finish_time_text(g.header().finish_time()));

        let mii_name_text = self
            .active_ghost
            .as_ref()
            .map(|g| widgets::mii_name_text(g.header().mii().name()));

        let country_image = self.active_ghost.as_ref().map(|g| {
            widgets::country_image(image_handles::get_country_image_handle(
                g.header().location().country(),
            ))
        });

        let character_image = self.active_ghost.as_ref().map(|g| {
            widgets::character_image(image_handles::get_character_image_handle(
                g.header().combo().character(),
            ))
        });

        let vehicle_image = self.active_ghost.as_ref().map(|g| {
            widgets::vehicle_image(image_handles::get_vehicle_image_handle(
                g.header().combo().vehicle(),
            ))
        });

        let mut s = stack!(
            background,
            rkg_inspector_text,
            select_ghost_button,
            toggle_edit_button,
            save_as_button,
        )
        .width(Length::Fill)
        .height(Length::Fill);

        for elem in [
            track_name_text,
            finish_time_text,
            mii_name_text,
            country_image,
            character_image,
            vehicle_image,
        ]
        .into_iter()
        .flatten()
        {
            s = s.push(elem);
        }

        s.into()
    }
}

impl Default for RkgInspector {
    fn default() -> Self {
        Self::new()
    }
}
