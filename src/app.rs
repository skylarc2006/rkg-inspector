use iced::widget::{image, svg};
use iced::{Element, Length, Task, Theme, widget::stack};
use rkg_utils::Ghost;
use rkg_utils::header::mii::Mii;

use crate::files::{pick_file, save_as_file};
use crate::helpers::*;
use crate::message::Message;
use crate::ui::{assets, image_handles, widgets};

pub struct RkgInspector {
    pub active_ghost: Option<Ghost>,
    pub background_handle: image::Handle,
    pub ghost_box_handle: image::Handle,
    pub edit_menu_active: bool,
    pub character_handle: Option<image::Handle>,
    pub vehicle_handle: Option<image::Handle>,
    pub country_handle: Option<svg::Handle>,
    pub mii_handle: Option<image::Handle>,
    loading: bool,
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
            mii_handle: None,
            loading: false,
        }
    }

    pub fn title(&self) -> String {
        String::from("RKG Inspector")
    }

    pub fn theme(&self) -> Theme {
        Theme::Dark
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::event::listen_with(|event, _status, _id| {
            if let iced::Event::Window(iced::window::Event::FileDropped(path)) = event {
                Some(Message::GhostDropped(path))
            } else {
                None
            }
        })
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::LoadGhost => Task::perform(pick_file("Mario Kart Wii ghosts", &["rkg"]), Message::GhostPicked),

            Message::GhostDropped(path) => {
                if self.loading {
                    return Task::none();
                }
                self.update(Message::GhostPicked(Some(path)))
            },

            Message::GhostPicked(path) => {
                self.loading = true;
                self.mii_handle = None;
                self.active_ghost = path.and_then(|p| Ghost::new_from_file(&p).ok());
                if let Some(ghost) = &self.active_ghost {
                    self.character_handle = Some(image_handles::get_character_image_handle(
                        ghost.header().combo().character(),
                    ));
                    self.vehicle_handle = Some(image_handles::get_vehicle_image_handle(
                        ghost.header().combo().vehicle(),
                    ));
                    self.country_handle = Some(image_handles::get_country_image_handle(
                        ghost.header().location().country(),
                    ));
                    Task::perform(
                        image_handles::get_mii_image_handle(
                            ghost.header().mii().raw_data().to_vec(),
                        ),
                        Message::MiiHandleLoaded,
                    )
                } else {
                    self.character_handle = None;
                    self.vehicle_handle = None;
                    self.country_handle = None;
                    self.mii_handle = None;
                    self.loading = false;
                    Task::none()
                }
            },

            Message::MiiExport => {
                if let Some(ghost) = &self.active_ghost {
                    Task::perform(save_as_file(ghost.header().mii().name().to_string(), "Mii data", &["miigx", "mae", "mii"]), Message::MiiSaved)
                } else {
                    Task::none()
                }
            },

            Message::MiiImport => {
                if self.active_ghost.is_some() {
                    Task::perform(pick_file("Mii data", &["miigx", "mae", "mii", "rkg"]), Message::MiiSelected)
                } else {
                    Task::none()
                }
            },

            Message::MiiSaved(path) => {
                if let Some(ghost) = &self.active_ghost {
                    path.and_then(|p| ghost.header().mii().save_to_file(&p).ok());
                }
                Task::none()
            },

            Message::MiiSelected(path) => {
                if let Some(ghost) = &mut self.active_ghost {
                    if let Some(mii) = path.and_then(|p| Mii::new_from_file(&p).ok()) {
                        ghost.header_mut().set_mii(mii);
                        self.mii_handle = None;
                        Task::perform(
                            image_handles::get_mii_image_handle(
                                ghost.header().mii().raw_data().to_vec(),
                            ),
                            Message::MiiHandleLoaded,
                        )
                    } else {
                        Task::none()
                    }
                } else {
                    Task::none()
                }
            },


            Message::MiiHandleLoaded(mii_handle) => {
                self.mii_handle = mii_handle;
                self.loading = false;
                Task::none()
            },

            Message::ToggleEditMenu => Task::none(),

            Message::SaveGhostAsFile => {
                if let Some(ghost) = &self.active_ghost {
                    let finish_time = ghost.header().finish_time();
                    let time = format!(
                        "{:02}m{:02}s{:03}",
                        finish_time.minutes(),
                        finish_time.seconds(),
                        finish_time.milliseconds()
                    );
                    let mii_name = ghost.header().mii().name();
                    let track_abbreviation = track_abbreviation(ghost.header().slot_id());

                    let default_file_name =
                        format!("{}_{}_{}.rkg", time, track_abbreviation, mii_name);
                    Task::perform(save_as_file(default_file_name, "Mario Kart Wii ghosts", &["rkg"]), Message::GhostSaved)
                } else {
                    Task::none()
                }
            },

            Message::GhostSaved(path) => {
                if let Some(ghost) = &mut self.active_ghost {
                    path.and_then(|p| ghost.save_to_file(&p).ok());
                }
                Task::none()
            },

            Message::ToggleFooterView => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let background = widgets::background(
            self.background_handle.clone(),
            self.ghost_box_handle.clone(),
        );
        let prerelease_warning_text = widgets::prerelease_warning_text();
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

        let country_element = self
            .active_ghost
            .as_ref()
            .zip(self.country_handle.as_ref())
            .map(|(g, h)| widgets::country_element(g, h));

        let character_element = self
            .active_ghost
            .as_ref()
            .zip(self.character_handle.as_ref())
            .map(|(g, h)| widgets::character_element(g, h));

        let vehicle_element = self
            .active_ghost
            .as_ref()
            .zip(self.vehicle_handle.as_ref())
            .map(|(g, h)| widgets::vehicle_element(g, h));

        let lap_splits_box = self
            .active_ghost
            .as_ref()
            .map(|g| widgets::lap_splits_box(g.header().lap_split_times()));

        let mii_box = self
            .active_ghost
            .as_ref()
            .map(|g| widgets::mii_info_box(g.header().mii()));

        let date_set_box = self
            .active_ghost
            .as_ref()
            .map(|g| widgets::date_set_box(g.header().date_set()));

        let ghost_type_box = self
            .active_ghost
            .as_ref()
            .map(|g| widgets::ghost_type_box(g));

        let controller_box = self
            .active_ghost
            .as_ref()
            .map(|g| widgets::controller_box(g.header().controller()));

        let external_footer_button = self
            .active_ghost
            .as_ref()
            .and_then(|g| widgets::external_footer_button(g));

        let mii_image_element = self
            .active_ghost
            .as_ref()
            .zip(self.mii_handle.as_ref())
            .map(|(_, h)| widgets::mii_image_element(h));

        let mii_import_button = self
            .active_ghost
            .as_ref()
            .and_then(|_| Some(widgets::mii_import_button()));

        let mii_export_button = self
            .active_ghost
            .as_ref()
            .and_then(|_| Some(widgets::mii_export_button()));

        let mut s = stack!(
            background,
            prerelease_warning_text,
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
            lap_splits_box,
            mii_box,
            date_set_box,
            ghost_type_box,
            controller_box,
            external_footer_button,
            mii_image_element,
            mii_import_button,
            mii_export_button, 
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
