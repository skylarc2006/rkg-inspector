use std::time::Duration;

use iced::widget::{image, svg};
use iced::{Element, Length, Subscription, Task, Theme, widget::stack};
use rkg_utils::Ghost;
use rkg_utils::footer::FooterType;
use rkg_utils::header::Combo;
use rkg_utils::header::combo::GetWeightClass;
use rkg_utils::header::mii::Mii;

use crate::chadsoft::{
    chadsoft_ghost_link, chadsoft_leaderboard_link, chadsoft_player_link, fetch_ctgp_track_name,
};
use crate::files::{pick_file, pick_files, save_as_file};
use crate::helpers::track_abbreviation;
use crate::link_type::LinkType;
use crate::message::{CtgpLink, Message};
use crate::mii_rendering;
use crate::ui::edit_data::{self, EditBuffers, VEHICLES, parse_date, parse_in_game_time};
use crate::ui::footer_tab::FooterTab;
use crate::ui::input_playback::{self, InputPlayback};
use crate::ui::{assets, image_handles, widgets};

/// A ghost and everything derived from it that the UI needs to render.
struct LoadedGhost {
    ghost: Ghost,
    character_handle: image::Handle,
    vehicle_handle: image::Handle,
    country_handle: svg::Handle,
    /// `None` while the Mii Studio render is still loading.
    mii_handle: Option<image::Handle>,
    custom_track_name: Option<String>,
    edit_buffers: EditBuffers,
    input_playback: InputPlayback,
    /// Whether drift is actually active for each entry in
    /// `ghost.input_data().controller_inputs()`, index-aligned to it.
    effective_drift: Vec<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Screen {
    Main,
    EditMenu,
    FooterInfo,
    InputDataInfo,
}

pub struct RkgInspector {
    ghosts: Vec<LoadedGhost>,
    active_index: usize,
    background_handle: image::Handle,
    ghost_box_handle: image::Handle,
    info_background_handle: image::Handle,
    input_box_handle: image::Handle,
    screen: Screen,
    active_footer_tab: FooterTab,
}

impl RkgInspector {
    pub fn new() -> Self {
        Self {
            ghosts: Vec::new(),
            active_index: 0,
            background_handle: image::Handle::from_bytes(assets::BACKGROUND),
            ghost_box_handle: image::Handle::from_bytes(assets::GHOST_BOX),
            info_background_handle: image::Handle::from_bytes(assets::INFO_BACKGROUND),
            input_box_handle: image::Handle::from_bytes(assets::INPUT_BOX),
            screen: Screen::Main,
            active_footer_tab: FooterTab::CtgpIdentity,
        }
    }

    pub fn title(&self) -> String {
        String::from("RKG Inspector")
    }

    pub fn theme(&self) -> Theme {
        Theme::Dark
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        let file_drop = iced::event::listen_with(|event, _status, _id| {
            if let iced::Event::Window(iced::window::Event::FileDropped(path)) = event {
                Some(Message::GhostDropped(path))
            } else {
                None
            }
        });

        let is_playing = self.screen == Screen::InputDataInfo
            && self
                .active()
                .is_some_and(|loaded| loaded.input_playback.is_playing);

        if is_playing {
            let playback_tick =
                iced::time::every(Duration::from_millis(16)).map(|_| Message::InputPlaybackTick);
            Subscription::batch([file_drop, playback_tick])
        } else {
            file_drop
        }
    }

    fn active(&self) -> Option<&LoadedGhost> {
        self.ghosts.get(self.active_index)
    }

    fn with_loaded(&self, f: impl FnOnce(&LoadedGhost) -> Task<Message>) -> Task<Message> {
        self.active().map_or_else(Task::none, f)
    }

    fn with_loaded_mut(
        &mut self,
        f: impl FnOnce(&mut LoadedGhost) -> Task<Message>,
    ) -> Task<Message> {
        let index = self.active_index;
        self.ghosts.get_mut(index).map_or_else(Task::none, f)
    }

    fn sync_active_footer_tab(&mut self) {
        if let Some(loaded) = self.active() {
            match loaded.ghost.footer() {
                Some(FooterType::CTGPFooter(_)) => {
                    self.active_footer_tab = FooterTab::CtgpIdentity;
                }
                Some(FooterType::SPFooter(_)) => {
                    self.active_footer_tab = FooterTab::SpIdentity;
                }
                Some(FooterType::Unknown(_)) | None => (),
            }
        }
    }

    fn append_ghost(&mut self, ghost: Ghost) -> Task<Message> {
        let index = self.ghosts.len();

        let character_handle =
            image_handles::get_character_image_handle(ghost.header().combo().character());
        let vehicle_handle =
            image_handles::get_vehicle_image_handle(ghost.header().combo().vehicle());
        let country_handle =
            image_handles::get_country_image_handle(ghost.header().location().country());

        let mii_task = Task::perform(
            mii_rendering::get_mii_image_handle(ghost.header().mii().raw_data().to_vec()),
            move |handle| Message::MiiHandleLoaded(index, handle),
        );

        let track_name_task = if let Some(FooterType::CTGPFooter(ctgp_footer)) = ghost.footer() {
            let slot_id = ghost.header().slot_id();
            let track_sha1 = ctgp_footer.track_sha1().to_vec();
            let category = ctgp_footer.category();
            Task::perform(
                fetch_ctgp_track_name(slot_id, track_sha1, category),
                move |name| Message::CtgpTrackNameLoaded(index, name),
            )
        } else {
            Task::none()
        };

        let edit_buffers = EditBuffers::from_header(ghost.header());
        let effective_drift =
            input_playback::effective_drift_flags(ghost.input_data().controller_inputs());

        self.pause_active_input_playback();

        self.ghosts.push(LoadedGhost {
            ghost,
            character_handle,
            vehicle_handle,
            country_handle,
            mii_handle: None,
            custom_track_name: None,
            edit_buffers,
            input_playback: InputPlayback::new(),
            effective_drift,
        });

        self.screen = Screen::Main;
        self.active_index = index;
        self.sync_active_footer_tab();

        Task::batch([mii_task, track_name_task])
    }

    
    fn pause_active_input_playback(&mut self) {
        if let Some(loaded) = self.ghosts.get_mut(self.active_index) {
            loaded.input_playback.pause();
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::LoadGhost => Task::perform(
                pick_files("Mario Kart Wii ghosts", &["rkg"]),
                Message::GhostsPicked,
            ),

            Message::GhostDropped(path) => self.update(Message::GhostPicked(Some(path))),

            Message::GhostPicked(path) => {
                // A cancelled file dialog (`None`) or a file that failed to parse
                // leaves whatever ghosts were already loaded untouched, rather
                // than wiping the screen.
                let Some(path) = path else {
                    return Task::none();
                };
                let Ok(ghost) = Ghost::new_from_file(&path) else {
                    return Task::none();
                };
                self.append_ghost(ghost)
            }

            Message::GhostsPicked(paths) => Task::batch(
                paths
                    .into_iter()
                    .filter_map(|path| Ghost::new_from_file(&path).ok())
                    .map(|ghost| self.append_ghost(ghost))
                    .collect::<Vec<_>>(),
            ),

            Message::NextGhost => {
                if !self.ghosts.is_empty() {
                    self.active_index = (self.active_index + 1) % self.ghosts.len();
                    self.sync_active_footer_tab();
                }
                Task::none()
            }

            Message::PreviousGhost => {
                if !self.ghosts.is_empty() {
                    self.active_index =
                        (self.active_index + self.ghosts.len() - 1) % self.ghosts.len();
                    self.sync_active_footer_tab();
                }
                Task::none()
            }

            Message::ClearGhosts => {
                self.ghosts.clear();
                self.active_index = 0;
                self.screen = Screen::Main;
                Task::none()
            }

            Message::MiiExport => {
                let index = self.active_index;
                self.with_loaded(|loaded| {
                    Task::perform(
                        save_as_file(
                            loaded.ghost.header().mii().name().to_string(),
                            "Mii data",
                            &["miigx", "mae", "mii"],
                        ),
                        move |path| Message::MiiSaved(index, path),
                    )
                })
            }

            Message::MiiImport => {
                if self.active().is_some() {
                    let index = self.active_index;
                    Task::perform(
                        pick_file("Mii data", &["miigx", "mae", "mii", "rkg"]),
                        move |path| Message::MiiSelected(index, path),
                    )
                } else {
                    Task::none()
                }
            }

            Message::MiiSaved(index, path) => {
                if let Some(loaded) = self.ghosts.get(index) {
                    path.and_then(|p| loaded.ghost.header().mii().save_to_file(&p).ok());
                }
                Task::none()
            }

            Message::MiiSelected(index, path) => {
                let Some(loaded) = self.ghosts.get_mut(index) else {
                    return Task::none();
                };
                let Some(mii) = path.and_then(|p| Mii::new_from_file(&p).ok()) else {
                    return Task::none();
                };
                loaded.ghost.header_mut().set_mii(mii);
                loaded.mii_handle = None;
                Task::perform(
                    mii_rendering::get_mii_image_handle(
                        loaded.ghost.header().mii().raw_data().to_vec(),
                    ),
                    move |handle| Message::MiiHandleLoaded(index, handle),
                )
            }

            Message::MiiHandleLoaded(index, mii_handle) => {
                if let Some(loaded) = self.ghosts.get_mut(index) {
                    loaded.mii_handle = mii_handle;
                }
                Task::none()
            }

            Message::ToggleEditMenu => {
                self.screen = if self.screen == Screen::EditMenu {
                    Screen::Main
                } else {
                    Screen::EditMenu
                };
                Task::none()
            }

            Message::ToggleFooterInfoMenu => {
                self.screen = if self.screen == Screen::FooterInfo {
                    Screen::Main
                } else {
                    Screen::FooterInfo
                };
                Task::none()
            }

            Message::ToggleInputDataMenu => {
                self.screen = if self.screen == Screen::InputDataInfo {
                    self.pause_active_input_playback();
                    Screen::Main
                } else {
                    Screen::InputDataInfo
                };
                Task::none()
            }

            Message::SetActiveFooterTab(footer_tab) => {
                self.active_footer_tab = footer_tab;
                Task::none()
            }

            Message::SaveGhostAsFile => {
                let index = self.active_index;
                self.with_loaded(|loaded| {
                    let finish_time = loaded.ghost.header().finish_time();
                    let time = format!(
                        "{:02}m{:02}s{:03}",
                        finish_time.minutes(),
                        finish_time.seconds(),
                        finish_time.milliseconds()
                    );
                    let mii_name = loaded.ghost.header().mii().name();
                    let track_abbreviation = track_abbreviation(loaded.ghost.header().slot_id());

                    let default_file_name =
                        format!("{}_{}_{}.rkg", time, track_abbreviation, mii_name);
                    Task::perform(
                        save_as_file(default_file_name, "Mario Kart Wii ghosts", &["rkg"]),
                        move |path| Message::GhostSaved(index, path),
                    )
                })
            }

            Message::GhostSaved(index, path) => {
                if let Some(loaded) = self.ghosts.get_mut(index) {
                    path.and_then(|p| loaded.ghost.save_to_file(&p).ok());
                }
                Task::none()
            }

            Message::OpenCtgpLink(link) => {
                if let Some(loaded) = self.active()
                    && let Some(FooterType::CTGPFooter(footer)) = loaded.ghost.footer()
                {
                    let url = match link {
                        CtgpLink::Leaderboard => chadsoft_leaderboard_link(
                            loaded.ghost.header().slot_id(),
                            footer.track_sha1(),
                            footer.category(),
                            LinkType::Html,
                        ),
                        CtgpLink::Ghost => chadsoft_ghost_link(footer.ghost_sha1(), LinkType::Html),
                        CtgpLink::Player => {
                            chadsoft_player_link(footer.player_id(), LinkType::Html)
                        }
                    };

                    if webbrowser::open(&url).is_ok() {
                        // TODO: error handle
                    }
                }
                Task::none()
            }

            Message::CtgpTrackNameLoaded(index, track_name) => {
                if let Some(loaded) = self.ghosts.get_mut(index)
                    && loaded.custom_track_name.is_none()
                {
                    loaded.custom_track_name = track_name;
                }
                Task::none()
            }

            Message::EditFinishTimeChanged(s) => self.with_loaded_mut(|loaded| {
                if let Some(time) = parse_in_game_time(&s) {
                    loaded.ghost.header_mut().set_finish_time(time);
                }
                loaded.edit_buffers.finish_time = s;
                Task::none()
            }),

            Message::EditLapSplitChanged(idx, s) => self.with_loaded_mut(|loaded| {
                if let Some(time) = parse_in_game_time(&s) {
                    loaded.ghost.header_mut().set_lap_split_time(idx, time);
                }
                if let Some(buffer) = loaded.edit_buffers.lap_splits.get_mut(idx) {
                    *buffer = s;
                }
                Task::none()
            }),

            Message::EditDateChanged(s) => self.with_loaded_mut(|loaded| {
                if let Some(date) = parse_date(&s) {
                    loaded.ghost.header_mut().set_date_set(date);
                }
                loaded.edit_buffers.date = s;
                Task::none()
            }),

            Message::EditSlotIdSelected(slot_id) => self.with_loaded_mut(|loaded| {
                loaded.ghost.header_mut().set_slot_id(slot_id);
                Task::none()
            }),

            Message::EditCharacterSelected(character) => self.with_loaded_mut(|loaded| {
                let vehicle = loaded.ghost.header().combo().vehicle();
                let vehicle = if vehicle.get_weight_class() == character.get_weight_class() {
                    vehicle
                } else {
                    VEHICLES
                        .iter()
                        .copied()
                        .find(|v| v.get_weight_class() == character.get_weight_class())
                        .unwrap_or(vehicle)
                };

                if let Ok(combo) = Combo::new(vehicle, character) {
                    loaded.ghost.header_mut().set_combo(combo);
                    loaded.character_handle = image_handles::get_character_image_handle(character);
                    loaded.vehicle_handle = image_handles::get_vehicle_image_handle(vehicle);
                }
                Task::none()
            }),

            Message::EditVehicleSelected(vehicle) => self.with_loaded_mut(|loaded| {
                // The vehicle picker only offers options matching the current
                // character's weight class, so this combo is always valid.
                let character = loaded.ghost.header().combo().character();
                if let Ok(combo) = Combo::new(vehicle, character) {
                    loaded.ghost.header_mut().set_combo(combo);
                    loaded.vehicle_handle = image_handles::get_vehicle_image_handle(vehicle);
                }
                Task::none()
            }),

            Message::EditControllerSelected(controller) => self.with_loaded_mut(|loaded| {
                loaded.ghost.header_mut().set_controller(controller);
                Task::none()
            }),

            Message::EditTransmissionModSelected(transmission_mod) => {
                self.with_loaded_mut(|loaded| {
                    loaded
                        .ghost
                        .header_mut()
                        .set_transmission_mod(transmission_mod);
                    Task::none()
                })
            }

            Message::EditGhostTypeSelected(ghost_type) => self.with_loaded_mut(|loaded| {
                loaded.ghost.header_mut().set_ghost_type(ghost_type);
                Task::none()
            }),

            Message::EditAutomaticDriftToggled(is_automatic_drift) => {
                self.with_loaded_mut(|loaded| {
                    loaded
                        .ghost
                        .header_mut()
                        .set_automatic_drift(is_automatic_drift);
                    Task::none()
                })
            }

            Message::EditCountrySelected(country) => self.with_loaded_mut(|loaded| {
                if let Some(location) = edit_data::location_table()
                    .iter()
                    .find(|c| c.country == country)
                    .and_then(|c| c.options.first())
                    .map(|o| o.location)
                {
                    loaded.ghost.header_mut().set_location(location);
                    loaded.country_handle = image_handles::get_country_image_handle(country);
                }
                Task::none()
            }),

            Message::EditSubregionSelected(subregion) => self.with_loaded_mut(|loaded| {
                let country = loaded.ghost.header().location().country();
                if let Some(location) = edit_data::location_table()
                    .iter()
                    .find(|c| c.country == country)
                    .and_then(|c| c.options.iter().find(|o| o.subregion == subregion))
                    .map(|o| o.location)
                {
                    loaded.ghost.header_mut().set_location(location);
                }
                Task::none()
            }),

            Message::ToggleInputPlayback => self.with_loaded_mut(|loaded| {
                let total_frames = loaded.ghost.input_data().total_frame_duration();
                loaded.input_playback.toggle_play(total_frames);
                Task::none()
            }),

            Message::InputPlaybackTick => self.with_loaded_mut(|loaded| {
                let total_frames = loaded.ghost.input_data().total_frame_duration();
                loaded.input_playback.tick(total_frames);
                Task::none()
            }),

            Message::InputSeek(frame) => self.with_loaded_mut(|loaded| {
                let total_frames = loaded.ghost.input_data().total_frame_duration();
                loaded.input_playback.seek(frame, total_frames);
                Task::none()
            }),

            Message::InputStepFrame(delta) => self.with_loaded_mut(|loaded| {
                let total_frames = loaded.ghost.input_data().total_frame_duration();
                loaded.input_playback.step(delta, total_frames);
                Task::none()
            }),

            Message::InputJumpToStart => self.with_loaded_mut(|loaded| {
                loaded.input_playback.jump_to_start();
                Task::none()
            }),

            Message::InputJumpToEnd => self.with_loaded_mut(|loaded| {
                let total_frames = loaded.ghost.input_data().total_frame_duration();
                loaded.input_playback.jump_to_end(total_frames);
                Task::none()
            }),

            Message::InputSpeedSelected(speed) => self.with_loaded_mut(|loaded| {
                loaded.input_playback.set_speed(speed);
                Task::none()
            }),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        match self.screen {
            Screen::Main => self.main_view(),
            Screen::EditMenu => self.edit_view(),
            Screen::FooterInfo => self.footer_info_view(),
            Screen::InputDataInfo => self.input_data_info_view(),
        }
    }

    fn main_view(&self) -> Element<'_, Message> {
        let background = widgets::background(
            self.background_handle.clone(),
            self.ghost_box_handle.clone(),
        );
        let rkg_inspector_text = widgets::rkg_inspector_text();
        let select_ghost_button = widgets::select_ghost_button();
        let has_multiple_ghosts = self.ghosts.len() > 1;
        let previous_ghost_button = widgets::previous_ghost_button(has_multiple_ghosts);
        let next_ghost_button = widgets::next_ghost_button(has_multiple_ghosts);
        let clear_ghosts_button = widgets::clear_ghosts_button(!self.ghosts.is_empty());
        let toggle_edit_button = widgets::toggle_edit_button(self.active().is_some());
        let save_as_button = widgets::save_as_button(self.active().is_some());

        let mut s = stack!(
            background,
            rkg_inspector_text,
            select_ghost_button,
            previous_ghost_button,
            next_ghost_button,
            clear_ghosts_button,
            toggle_edit_button,
            save_as_button,
        )
        .width(Length::Fill)
        .height(Length::Fill);

        let Some(loaded) = self.active() else {
            return s.into();
        };
        let ghost = &loaded.ghost;

        let elements = [
            Some(widgets::track_name_text(
                ghost,
                loaded.custom_track_name.clone(),
            )),
            (self.ghosts.len() > 1)
                .then(|| widgets::ghost_counter_text(self.active_index, self.ghosts.len())),
            Some(widgets::finish_time_text(ghost.header().finish_time())),
            Some(widgets::mii_name_text(ghost.header().mii().name())),
            Some(widgets::country_element(ghost, &loaded.country_handle)),
            Some(widgets::character_element(ghost, &loaded.character_handle)),
            Some(widgets::vehicle_element(ghost, &loaded.vehicle_handle)),
            Some(widgets::lap_splits_box(ghost.header().lap_split_times())),
            Some(widgets::mii_info_box(ghost.header().mii())),
            Some(widgets::shroomstrat_box(ghost.shroomstrat())),
            Some(widgets::shroom_element(ghost.shroomstrat())),
            Some(widgets::date_set_box(ghost.header().date_set())),
            Some(widgets::ghost_type_box(ghost)),
            Some(widgets::controller_box(ghost.header().controller())),
            widgets::external_footer_button(ghost),
            widgets::input_data_button(),
            loaded.mii_handle.as_ref().map(widgets::mii_image_element),
            Some(widgets::mii_import_button()),
            Some(widgets::mii_export_button()),
        ];

        for elem in elements.into_iter().flatten() {
            s = s.push(elem);
        }

        s.into()
    }

    fn edit_view(&self) -> Element<'_, Message> {
        let background = widgets::background(
            self.background_handle.clone(),
            self.ghost_box_handle.clone(),
        );
        let info_background = widgets::info_background(self.info_background_handle.clone());

        let mut s = stack!(
            background,
            widgets::rkg_inspector_text(),
            info_background,
        )
        .width(Length::Fill)
        .height(Length::Fill);

        if let Some(loaded) = self.active() {
            s = s.push(widgets::close_edit_button());
            s = s.push(widgets::edit_form(&loaded.ghost, &loaded.edit_buffers));
        }

        s.into()
    }

    fn footer_info_view(&self) -> Element<'_, Message> {
        let background = widgets::background(
            self.background_handle.clone(),
            self.ghost_box_handle.clone(),
        );
        let info_background = widgets::info_background(self.info_background_handle.clone());

        let mut s = stack!(
            background,
            widgets::rkg_inspector_text(),
            info_background,
        )
        .width(Length::Fill)
        .height(Length::Fill);

        let Some(loaded) = self.active() else {
            return s.into();
        };

        if let Some(FooterType::CTGPFooter(_)) = loaded.ghost.footer() {
            s = s.push(widgets::ctgp_footer_identity_button(
                self.active_footer_tab == FooterTab::CtgpIdentity,
            ));
            s = s.push(widgets::ctgp_footer_time_info_button(
                self.active_footer_tab == FooterTab::CtgpTimeInfo,
            ));
            s = s.push(widgets::ctgp_footer_race_events_button(
                self.active_footer_tab == FooterTab::CtgpRaceEvents,
            ));
        } else if let Some(FooterType::SPFooter(_)) = loaded.ghost.footer() {
            s = s.push(widgets::sp_footer_identity_button(
                self.active_footer_tab == FooterTab::SpIdentity,
            ));
            s = s.push(widgets::sp_footer_time_info_button(
                self.active_footer_tab == FooterTab::SpTimeInfo,
            ));
            s = s.push(widgets::sp_footer_race_events_button(
                self.active_footer_tab == FooterTab::SpRaceEvents,
            ));
        }

        s = s.push(widgets::footer_info_text(
            self.active_footer_tab,
            &loaded.ghost,
        ));
        s = s.push(widgets::close_footer_info_button());

        s.into()
    }

    fn input_data_info_view(&self) -> Element<'_, Message> {
        let background = widgets::background(
            self.background_handle.clone(),
            self.ghost_box_handle.clone(),
        );
        let info_background = widgets::info_background(self.info_background_handle.clone());

        let mut s = stack!(
            background,
            widgets::rkg_inspector_text(),
            info_background,
        )
        .width(Length::Fill)
        .height(Length::Fill);

        s = s.push(widgets::close_input_data_button());

        if let Some(loaded) = self.active() {
            s = s.push(widgets::input_viewer(
                &loaded.ghost,
                &loaded.input_playback,
                &loaded.effective_drift,
                self.input_box_handle.clone(),
            ));
        }

        s.into()
    }
}

impl Default for RkgInspector {
    fn default() -> Self {
        Self::new()
    }
}
