use iced::widget::{image, svg};
use iced::{Element, Length, Task, Theme, widget::stack};
use rkg_utils::Ghost;
use rkg_utils::footer::FooterType;
use rkg_utils::header::mii::Mii;

use crate::chadsoft::{
    chadsoft_ghost_link, chadsoft_leaderboard_link, chadsoft_player_link, fetch_ctgp_track_name,
};
use crate::files::{pick_file, save_as_file};
use crate::helpers::track_abbreviation;
use crate::link_type::LinkType;
use crate::message::{CtgpLink, Message};
use crate::mii_rendering;
use crate::ui::footer_tab::FooterTab;
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
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Screen {
    Main,
    EditMenu,
    FooterInfo,
}

pub struct RkgInspector {
    active: Option<LoadedGhost>,
    background_handle: image::Handle,
    ghost_box_handle: image::Handle,
    info_background_handle: image::Handle,
    screen: Screen,
    loading: bool,
    active_footer_tab: FooterTab,
}

impl RkgInspector {
    pub fn new() -> Self {
        Self {
            active: None,
            background_handle: image::Handle::from_bytes(assets::BACKGROUND),
            ghost_box_handle: image::Handle::from_bytes(assets::GHOST_BOX),
            info_background_handle: image::Handle::from_bytes(assets::INFO_BACKGROUND),
            screen: Screen::Main,
            loading: false,
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
        iced::event::listen_with(|event, _status, _id| {
            if let iced::Event::Window(iced::window::Event::FileDropped(path)) = event {
                Some(Message::GhostDropped(path))
            } else {
                None
            }
        })
    }

    fn with_loaded(&self, f: impl FnOnce(&LoadedGhost) -> Task<Message>) -> Task<Message> {
        self.active.as_ref().map_or_else(Task::none, f)
    }

    fn with_loaded_mut(&mut self, f: impl FnOnce(&mut LoadedGhost) -> Task<Message>) -> Task<Message> {
        self.active.as_mut().map_or_else(Task::none, f)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::LoadGhost => Task::perform(
                pick_file("Mario Kart Wii ghosts", &["rkg"]),
                Message::GhostPicked,
            ),

            Message::GhostDropped(path) => {
                if self.loading {
                    return Task::none();
                }
                self.update(Message::GhostPicked(Some(path)))
            }

            Message::GhostPicked(path) => {
                // A cancelled file dialog (`None`) or a file that failed to parse
                // leaves whatever ghost was already loaded untouched, rather than
                // wiping the screen.
                let Some(path) = path else {
                    return Task::none();
                };
                let Ok(ghost) = Ghost::new_from_file(&path) else {
                    return Task::none();
                };

                self.screen = Screen::Main;
                self.loading = true;

                let character_handle = image_handles::get_character_image_handle(
                    ghost.header().combo().character(),
                );
                let vehicle_handle =
                    image_handles::get_vehicle_image_handle(ghost.header().combo().vehicle());
                let country_handle =
                    image_handles::get_country_image_handle(ghost.header().location().country());

                match ghost.footer() {
                    Some(FooterType::CTGPFooter(_)) => {
                        self.active_footer_tab = FooterTab::CtgpIdentity;
                    }
                    Some(FooterType::SPFooter(_)) => {
                        self.active_footer_tab = FooterTab::SpIdentity;
                    }
                    Some(FooterType::Unknown(_)) | None => (),
                }

                let mii_task = Task::perform(
                    mii_rendering::get_mii_image_handle(ghost.header().mii().raw_data().to_vec()),
                    Message::MiiHandleLoaded,
                );

                self.active = Some(LoadedGhost {
                    ghost,
                    character_handle,
                    vehicle_handle,
                    country_handle,
                    mii_handle: None,
                    custom_track_name: None,
                });

                mii_task
            }

            Message::MiiExport => self.with_loaded(|loaded| {
                Task::perform(
                    save_as_file(
                        loaded.ghost.header().mii().name().to_string(),
                        "Mii data",
                        &["miigx", "mae", "mii"],
                    ),
                    Message::MiiSaved,
                )
            }),

            Message::MiiImport => {
                if self.active.is_some() {
                    Task::perform(
                        pick_file("Mii data", &["miigx", "mae", "mii", "rkg"]),
                        Message::MiiSelected,
                    )
                } else {
                    Task::none()
                }
            }

            Message::MiiSaved(path) => {
                if let Some(loaded) = &self.active {
                    path.and_then(|p| loaded.ghost.header().mii().save_to_file(&p).ok());
                }
                Task::none()
            }

            Message::MiiSelected(path) => self.with_loaded_mut(|loaded| {
                let Some(mii) = path.and_then(|p| Mii::new_from_file(&p).ok()) else {
                    return Task::none();
                };
                loaded.ghost.header_mut().set_mii(mii);
                loaded.mii_handle = None;
                Task::perform(
                    mii_rendering::get_mii_image_handle(
                        loaded.ghost.header().mii().raw_data().to_vec(),
                    ),
                    Message::MiiHandleLoaded,
                )
            }),

            Message::MiiHandleLoaded(mii_handle) => {
                if let Some(loaded) = &mut self.active {
                    loaded.mii_handle = mii_handle;
                }
                self.loading = false;
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

            Message::SetActiveFooterTab(footer_tab) => {
                self.active_footer_tab = footer_tab;
                Task::none()
            }

            Message::SaveGhostAsFile => self.with_loaded(|loaded| {
                let finish_time = loaded.ghost.header().finish_time();
                let time = format!(
                    "{:02}m{:02}s{:03}",
                    finish_time.minutes(),
                    finish_time.seconds(),
                    finish_time.milliseconds()
                );
                let mii_name = loaded.ghost.header().mii().name();
                let track_abbreviation = track_abbreviation(loaded.ghost.header().slot_id());

                let default_file_name = format!("{}_{}_{}.rkg", time, track_abbreviation, mii_name);
                Task::perform(
                    save_as_file(default_file_name, "Mario Kart Wii ghosts", &["rkg"]),
                    Message::GhostSaved,
                )
            }),

            Message::GhostSaved(path) => {
                if let Some(loaded) = &mut self.active {
                    path.and_then(|p| loaded.ghost.save_to_file(&p).ok());
                }
                Task::none()
            }

            Message::OpenCtgpLink(link) => {
                if let Some(loaded) = &self.active
                    && let Some(FooterType::CTGPFooter(footer)) = loaded.ghost.footer()
                {
                    let url = match link {
                        CtgpLink::Leaderboard => chadsoft_leaderboard_link(
                            loaded.ghost.header().slot_id(),
                            footer.track_sha1(),
                            footer.category(),
                            LinkType::Html,
                        ),
                        CtgpLink::Ghost => {
                            chadsoft_ghost_link(footer.ghost_sha1(), LinkType::Html)
                        }
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

            // Chadsoft's JSON API for looking up custom track names is currently
            // extremely unreliable/non-functional, so this is not wired up to any
            // button yet. Left in place to pick back up once the API is usable.
            Message::GetCtgpTrackName => self.with_loaded(|loaded| {
                if let Some(FooterType::CTGPFooter(ctgp_footer)) = loaded.ghost.footer() {
                    let slot_id = loaded.ghost.header().slot_id();
                    let track_sha1 = ctgp_footer.track_sha1().to_vec();
                    let category = ctgp_footer.category();
                    Task::perform(
                        fetch_ctgp_track_name(slot_id, track_sha1, category),
                        Message::CtgpTrackNameLoaded,
                    )
                } else {
                    Task::none()
                }
            }),

            Message::CtgpTrackNameLoaded(track_name) => {
                if let Some(loaded) = &mut self.active
                    && loaded.custom_track_name.is_none()
                {
                    loaded.custom_track_name = track_name;
                }
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        match self.screen {
            Screen::Main => self.main_view(),
            Screen::EditMenu => self.edit_view(),
            Screen::FooterInfo => self.footer_info_view(),
        }
    }

    fn main_view(&self) -> Element<'_, Message> {
        let background = widgets::background(
            self.background_handle.clone(),
            self.ghost_box_handle.clone(),
        );
        let prerelease_warning_text = widgets::prerelease_warning_text();
        let rkg_inspector_text = widgets::rkg_inspector_text();
        let select_ghost_button = widgets::select_ghost_button();
        let toggle_edit_button = widgets::toggle_edit_button(self.active.is_some());
        let save_as_button = widgets::save_as_button(self.active.is_some());

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

        let Some(loaded) = &self.active else {
            return s.into();
        };
        let ghost = &loaded.ghost;

        let elements = [
            Some(widgets::track_name_text(
                ghost,
                loaded.custom_track_name.clone(),
            )),
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
            widgets::prerelease_warning_text(),
            widgets::rkg_inspector_text(),
            info_background,
        )
        .width(Length::Fill)
        .height(Length::Fill);

        if self.active.is_some() {
            s = s.push(widgets::close_edit_button());
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
            widgets::prerelease_warning_text(),
            widgets::rkg_inspector_text(),
            info_background,
        )
        .width(Length::Fill)
        .height(Length::Fill);

        let Some(loaded) = &self.active else {
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
        }

        s = s.push(widgets::footer_info_text(
            self.active_footer_tab,
            &loaded.ghost,
        ));
        s = s.push(widgets::close_footer_info_button());

        s.into()
    }
}

impl Default for RkgInspector {
    fn default() -> Self {
        Self::new()
    }
}
