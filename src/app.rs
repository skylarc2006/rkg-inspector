use iced::widget::{container, image, svg, text, tooltip};
use iced::{Element, Length, Task, Theme, widget::stack};
use rkg_utils::Ghost;
use std::path::PathBuf;
use std::time::Duration;

use crate::files::{pick_file, save_as_file};
use crate::ui::constants::RODIN_NTLG_PRO_EB;
use crate::ui::{assets, image_handles, positioned, widgets};

#[derive(Debug, Clone)]
pub enum Message {
    LoadFile,
    FilePicked(Option<PathBuf>),
    ToggleEditMenu,
    SaveAsFile,
    FileSaved(Option<PathBuf>),
}

pub struct RkgInspector {
    pub active_ghost: Option<Ghost>,
    pub background_handle: image::Handle,
    pub ghost_box_handle: image::Handle,
    pub edit_menu_active: bool,
    pub character_handle: Option<image::Handle>,
    pub vehicle_handle: Option<image::Handle>,
    pub country_handle: Option<svg::Handle>,
}

impl RkgInspector {
    pub fn new() -> Self {
        Self {
            active_ghost: None,
            background_handle: image::Handle::from_bytes(assets::BACKGROUND),
            ghost_box_handle: image::Handle::from_bytes(assets::GHOST_BOX),
            edit_menu_active: false,
            character_handle: None,
            vehicle_handle: None,
            country_handle: None,
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
                if let Some(ghost) = &self.active_ghost {
                    self.character_handle = Some(image_handles::get_character_image_handle(ghost.header().combo().character()));
                    self.vehicle_handle = Some(image_handles::get_vehicle_image_handle(ghost.header().combo().vehicle()));
                    self.country_handle = Some(image_handles::get_country_image_handle(ghost.header().location().country()));
                } else {
                    self.character_handle = None;
                    self.vehicle_handle = None;
                    self.country_handle = None;
                }
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
            .active_ghost
            .as_ref()
            .map(|g| widgets::track_name_text(g.header().slot_id()));

        let finish_time_text = self
            .active_ghost
            .as_ref()
            .map(|g| widgets::finish_time_text(g.header().finish_time()));

        let mii_name_text = self
            .active_ghost
            .as_ref()
            .map(|g| widgets::mii_name_text(g.header().mii().name()));

        let country_element = self.active_ghost.as_ref().zip(self.country_handle.as_ref()).map(|(g, handle)| {
            let country_image = svg(handle.clone()).height(32).width(Length::Shrink);

            let tooltip_text = text(format!(
                "{} ({})",
                g.header().location().country(),
                g.header().location().subregion(),
            ))
            .font(RODIN_NTLG_PRO_EB);

            let img_with_tooltip = tooltip(
                country_image,
                container(tooltip_text)
                    .padding(3)
                    .style(container::rounded_box),
                tooltip::Position::FollowCursor,
            )
            .delay(Duration::from_millis(500));

            positioned(img_with_tooltip, 534, 300)
        });

        let character_element = self.active_ghost.as_ref().zip(self.character_handle.as_ref()).map(|(g, handle)| {
            let tooltip_text = text(g.header().combo().character().to_string()).font(RODIN_NTLG_PRO_EB);

            let img_with_tooltip = tooltip(
                image(handle.clone()).height(128.0 * 0.6),
                container(tooltip_text)
                    .padding(3)
                    .style(container::rounded_box),
                tooltip::Position::FollowCursor,
            )
            .delay(Duration::from_millis(500));
            positioned(img_with_tooltip, 678, 255)
        });

        let vehicle_element = self.active_ghost.as_ref().zip(self.vehicle_handle.as_ref()).map(|(g, handle)| {
            let tooltip_text = text(format!(
                "{} ({})",
                g.header().combo().vehicle(),
                if g.header().is_automatic_drift() { "Automatic" } else { "Manual" },
            ))
            .font(RODIN_NTLG_PRO_EB);

            let img_with_tooltip = tooltip(
                image(handle.clone()).height(100.0 * 0.76),
                container(tooltip_text)
                    .padding(3)
                    .style(container::rounded_box),
                tooltip::Position::FollowCursor,
            )
            .delay(Duration::from_millis(500));
            positioned(img_with_tooltip, 765, 256)
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
            country_element,
            character_element,
            vehicle_element,
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
