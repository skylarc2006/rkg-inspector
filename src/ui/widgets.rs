use iced::{
    Alignment, Color, Element, Length, Rectangle,
    widget::{
        Button, Image, Space, button, canvas, checkbox, column, container, image, pick_list, row,
        scrollable, slider, stack, svg, text, text_input, tooltip,
    },
};
use rkg_utils::{
    CTGPFooter, ControllerInput, FooterType, Ghost, Mii, SPFooter, Shroomstrat, footer::ctgp_footer::Category, header::{Controller, Date, InGameTime, TransmissionMod, combo::GetWeightClass},
};

use std::{cmp::max, time::Duration};

use crate::{
    helpers::array_to_hex_string,
    message::{CtgpLink, Message},
    ui::{
        assets::MUSHROOM,
        constants::{CTMKF, RODIN_NTLG_PRO_EB, VERSION},
        controller_canvas::{DPadCanvas, StickCanvas},
        edit_data::{
            self, CHARACTERS, CONTROLLERS, EditBuffers, GHOST_TYPES, SLOT_IDS, TRANSMISSION_MODS,
            VEHICLES,
        },
        fit_text::FitText,
        footer_tab::FooterTab,
        format::{disc_region_string, favorite_color_string},
        input_playback::{self, InputPlayback, PLAYBACK_SPEEDS, PlaybackSpeed},
        layout::{CLOSE_BUTTON_POS, FOOTER_INFO_ORIGIN},
        positioned, styles,
    },
};

const COMMON_BUTTON_WIDTH: u32 = 140;
const COMMON_BUTTON_HEIGHT: u32 = 40;

fn ghost_action_button(label: &str, enabled: bool, msg: Message) -> Button<'_, Message> {
    let btn = button(text(label).font(RODIN_NTLG_PRO_EB).size(16).center())
        .width(COMMON_BUTTON_WIDTH)
        .height(COMMON_BUTTON_HEIGHT);
    if enabled {
        btn.on_press(msg).style(styles::common_button_theme())
    } else {
        btn.style(|_, _| styles::disabled_button_style())
    }
}

fn close_style_button(label: &str, msg: Message) -> Button<'_, Message> {
    button(text(label).font(RODIN_NTLG_PRO_EB).size(28).center())
        .width(COMMON_BUTTON_WIDTH as f32 * 1.5)
        .height(COMMON_BUTTON_HEIGHT as f32 * 1.5)
        .on_press(msg)
        .style(styles::common_button_theme())
}

fn footer_tab_button(label: &str, size: f32, is_active: bool, msg: Message) -> Button<'_, Message> {
    button(text(label).font(RODIN_NTLG_PRO_EB).size(size).center())
        .width(COMMON_BUTTON_WIDTH as f32 * 1.25)
        .height(COMMON_BUTTON_HEIGHT as f32 * 1.25)
        .on_press(msg)
        .style(move |_, status| match status {
            button::Status::Hovered => styles::hovered_red_green_button_style(is_active),
            _ => styles::red_green_button_style(is_active),
        })
}

fn visit_button(label: &str, msg: Message) -> Button<'_, Message> {
    button(text(label).font(RODIN_NTLG_PRO_EB).size(12).center())
        .width(COMMON_BUTTON_WIDTH as f32 / 1.2)
        .height(COMMON_BUTTON_HEIGHT)
        .on_press(msg)
        .style(styles::common_button_theme())
}

/// Shared tail of every CTGP footer-info text block: font, sizing, and positioning.
fn info_paragraph<'a>(
    content: String,
    color: Color,
    size: f32,
    x: u32,
    y: u32,
) -> Element<'a, Message> {
    let text = text(content)
        .font(RODIN_NTLG_PRO_EB)
        .size(size)
        .width(930)
        .height(400)
        .color(color);

    positioned(text, x, y)
}

fn mii_label<'a>(label: &'a str, size: f32) -> Element<'a, Message> {
    text(label)
        .font(CTMKF)
        .color(Color::WHITE)
        .size(size)
        .into()
}

fn mii_value<'a>(value: impl std::fmt::Display, size: f32) -> Element<'a, Message> {
    text(value.to_string())
        .font(CTMKF)
        .color(Color::WHITE)
        .size(size)
        .align_x(Alignment::End)
        .into()
}

pub fn background(
    background_handle: image::Handle,
    ghost_box_handle: image::Handle,
) -> Element<'static, Message> {
    let background_image = image(background_handle).scale(1.0f32);
    let ghost_box: Image = image(ghost_box_handle).scale(0.5f32);
    stack!(background_image, ghost_box).into()
}

pub fn info_background(info_background_handle: image::Handle) -> Element<'static, Message> {
    image(info_background_handle).scale(0.85f32).into()
}

pub fn rkg_inspector_text() -> Element<'static, Message> {
    let t = text(format!("RKG Inspector {}", VERSION))
        .color(Color::from_rgba8(128, 128, 128, 1.0))
        .align_x(Alignment::Start)
        .align_y(Alignment::Start)
        .width(600)
        .font(RODIN_NTLG_PRO_EB)
        .size(28);
    positioned(t, 20, 43)
}

pub fn select_ghost_button() -> Element<'static, Message> {
    let btn = button(
        text("Select Ghost")
            .font(RODIN_NTLG_PRO_EB)
            .size(16)
            .center(),
    )
    .width(COMMON_BUTTON_WIDTH)
    .height(COMMON_BUTTON_HEIGHT)
    .on_press(Message::LoadGhost)
    .style(styles::common_button_theme());
    positioned(btn, 507, 80)
}

pub fn previous_ghost_button(enabled: bool) -> Element<'static, Message> {
    positioned(
        ghost_action_button("< Previous", enabled, Message::PreviousGhost),
        910,
        606,
    )
}

pub fn next_ghost_button(enabled: bool) -> Element<'static, Message> {
    positioned(
        ghost_action_button("Next >", enabled, Message::NextGhost),
        1130,
        606,
    )
}

pub fn clear_ghosts_button(enabled: bool) -> Element<'static, Message> {
    positioned(
        ghost_action_button("Clear", enabled, Message::ClearGhosts),
        1130,
        560,
    )
}

pub fn ghost_counter_text(active_index: usize, ghost_count: usize) -> Element<'static, Message> {
    let t = text(format!("{} / {}", active_index + 1, ghost_count))
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .width(80)
        .height(COMMON_BUTTON_HEIGHT)
        .font(RODIN_NTLG_PRO_EB)
        .size(16)
        .color(Color::WHITE);
    positioned(t, 1049, 606)
}

pub fn toggle_edit_button(ghost_is_loaded: bool) -> Element<'static, Message> {
    positioned(
        ghost_action_button("Edit Ghost", ghost_is_loaded, Message::ToggleEditMenu),
        657,
        80,
    )
}

pub fn save_as_button(ghost_is_loaded: bool) -> Element<'static, Message> {
    positioned(
        ghost_action_button("Save As...", ghost_is_loaded, Message::SaveGhostAsFile),
        807,
        80,
    )
}

pub fn close_edit_button() -> Element<'static, Message> {
    let (x, y) = CLOSE_BUTTON_POS;
    positioned(close_style_button("Close", Message::ToggleEditMenu), x, y)
}

pub fn close_input_data_button() -> Element<'static, Message> {
    let (x, y) = CLOSE_BUTTON_POS;
    positioned(close_style_button("Close", Message::ToggleInputDataMenu), x, y)
}

pub fn input_data_button<'a>() -> Option<Element<'a, Message>> {
    let btn = button(text("View input data").font(RODIN_NTLG_PRO_EB).size(16).center())
        .width(263)
        .height(COMMON_BUTTON_HEIGHT)
        .on_press(Message::ToggleInputDataMenu)
        .style(styles::common_button_theme());

    Some(positioned(btn, 508, 446))
}

pub fn ctgp_footer_identity_button(is_active: bool) -> Element<'static, Message> {
    let btn = footer_tab_button(
        "Identity",
        28.0,
        is_active,
        Message::SetActiveFooterTab(FooterTab::CtgpIdentity),
    );
    positioned(btn, 170, 115)
}

pub fn ctgp_footer_time_info_button(is_active: bool) -> Element<'static, Message> {
    let btn = footer_tab_button(
        "Time",
        28.0,
        is_active,
        Message::SetActiveFooterTab(FooterTab::CtgpTimeInfo),
    );
    positioned(btn, 375, 115)
}

pub fn ctgp_footer_race_events_button(is_active: bool) -> Element<'static, Message> {
    let btn = footer_tab_button(
        "Race Events",
        22.0,
        is_active,
        Message::SetActiveFooterTab(FooterTab::CtgpRaceEvents),
    );
    positioned(btn, 580, 115)
}

pub fn sp_footer_identity_button(is_active: bool) -> Element<'static, Message> {
    let btn = footer_tab_button(
        "Identity",
        28.0,
        is_active,
        Message::SetActiveFooterTab(FooterTab::SpIdentity),
    );
    positioned(btn, 170, 115)
}

pub fn sp_footer_time_info_button(is_active: bool) -> Element<'static, Message> {
    let btn = footer_tab_button(
        "Time",
        28.0,
        is_active,
        Message::SetActiveFooterTab(FooterTab::SpTimeInfo),
    );
    positioned(btn, 375, 115)
}

pub fn sp_footer_race_events_button(is_active: bool) -> Element<'static, Message> {
    let btn = footer_tab_button(
        "Race Events",
        22.0,
        is_active,
        Message::SetActiveFooterTab(FooterTab::SpRaceEvents),
    );
    positioned(btn, 580, 115)
}

pub fn close_footer_info_button() -> Element<'static, Message> {
    let (x, y) = CLOSE_BUTTON_POS;
    positioned(
        close_style_button("Close", Message::ToggleFooterInfoMenu),
        x,
        y,
    )
}

pub fn footer_info_text<'a>(
    active_footer_tab: FooterTab,
    ghost: &'a Ghost,
) -> Element<'a, Message> {
    let mut footer_info_view = stack!();

    if let Some(footer) = ghost.footer() {
        match footer {
            FooterType::CTGPFooter(ctgp_footer) => match active_footer_tab {
                FooterTab::CtgpIdentity => {
                    footer_info_view =
                        footer_info_view.push(ctgp_identity_info_element(ctgp_footer));
                    footer_info_view = footer_info_view.push(visit_ctgp_leaderboard_button());
                    footer_info_view = footer_info_view.push(visit_ctgp_ghost_page_button());
                    footer_info_view = footer_info_view.push(visit_ctgp_player_page_button());
                }
                FooterTab::CtgpTimeInfo => {
                    footer_info_view =
                        footer_info_view.push(ctgp_exact_time_info_element(ctgp_footer));
                    footer_info_view = footer_info_view.push(ctgp_rtc_info_element(ctgp_footer));
                    footer_info_view = footer_info_view.push(ctgp_pause_info_element(ctgp_footer));

                    if !ctgp_footer.pause_times().is_empty() {
                        footer_info_view =
                            footer_info_view.push(ctgp_pause_time_list_element(ctgp_footer));
                    }
                }
                FooterTab::CtgpRaceEvents => {
                    footer_info_view = footer_info_view.push(ctgp_race_flags_element(ctgp_footer));

                    footer_info_view = footer_info_view.push(ctgp_potentially_cheated_element(
                        ctgp_footer.potentially_cheated_ghost(),
                    ));

                    footer_info_view = footer_info_view.push(ctgp_potential_rapidfire_element(
                        ctgp_footer.potential_rapidfire(),
                    ));

                    footer_info_view = footer_info_view.push(ctgp_potential_slowdown_element(
                        ctgp_footer.potential_slowdown(),
                    ));

                    if let Some(intersection) = ctgp_footer.final_lap_dubious_intersection() {
                        footer_info_view =
                            footer_info_view.push(ctgp_final_lap_flag_element(intersection));
                    }

                    if let Some(enabled) = ctgp_footer.usb_gamecube_enabled() {
                        footer_info_view =
                            footer_info_view.push(ctgp_usb_gamecube_element(enabled));
                    }

                    if let Some(enabled) = ctgp_footer.my_stuff_enabled()
                        && let Some(used) = ctgp_footer.my_stuff_used()
                    {
                        footer_info_view =
                            footer_info_view.push(ctgp_my_stuff_element(enabled, used));
                    }

                    if let Some(disabled) = ctgp_footer.anti_tas_deliberately_disabled() {
                        footer_info_view = footer_info_view
                            .push(ctgp_anti_tas_deliberately_disabled_element(disabled));
                    }

                    if let Some(intersections) = ctgp_footer.lap_split_dubious_intersections() {
                        footer_info_view = footer_info_view
                            .push(ctgp_lap_dubious_intersection_element(intersections));
                    }
                }

                _ => (),
            },

            FooterType::SPFooter(sp_footer) => match active_footer_tab {
                FooterTab::SpIdentity => {
                    footer_info_view = footer_info_view.push(sp_identity_info_element(sp_footer));
                }
                FooterTab::SpTimeInfo => {
                    footer_info_view = footer_info_view.push(sp_exact_time_info_element(sp_footer));
                }
                FooterTab::SpRaceEvents => {
                    footer_info_view = footer_info_view.push(sp_race_settings_element(sp_footer));

                    footer_info_view = footer_info_view
                        .push(sp_ultra_shortcut_element(sp_footer.has_ultra_shortcut()));

                    footer_info_view = footer_info_view.push(sp_horizontal_wall_glitch_element(
                        sp_footer.has_horizontal_wall_glitch(),
                    ));

                    footer_info_view =
                        footer_info_view.push(sp_wallride_element(sp_footer.has_wallride()));
                }

                _ => (),
            },
            FooterType::Unknown(_) => (),
        }
    }

    footer_info_view.into()
}

pub fn ctgp_identity_info_element<'a>(ctgp_footer: &CTGPFooter) -> Element<'a, Message> {
    use std::fmt::Write;
    let mut s = String::new();
    write!(s, "Footer version: {}", ctgp_footer.footer_version()).unwrap();
    write!(
        s,
        "\n\nTrack SHA1: {}",
        array_to_hex_string(ctgp_footer.track_sha1())
    )
    .unwrap();
    write!(s, "\nCategory: {}", ctgp_footer.category()).unwrap();
    write!(
        s,
        "\n\nGhost SHA1: {}",
        array_to_hex_string(ctgp_footer.ghost_sha1())
    )
    .unwrap();
    write!(
        s,
        "\n\nPlayer ID: {}",
        array_to_hex_string(&ctgp_footer.player_id().to_be_bytes())
    )
    .unwrap();

    if let Some(disc_region) = ctgp_footer.disc_region() {
        write!(s, "\nDisc region: {}", disc_region_string(disc_region)).unwrap();
    }

    write!(s, "\n\nCTGP CORE version: {}", ctgp_footer.core_version()).unwrap();

    let ctgp_versions_opt = ctgp_footer.possible_release_versions();
    let release_versions = if let Some(ctgp_versions) = &ctgp_versions_opt {
        if ctgp_versions.len() == 1 {
            format!("{}", ctgp_versions[0])
        } else {
            format!(
                "{} - {}",
                ctgp_versions[0],
                ctgp_versions[ctgp_versions.len() - 1]
            )
        }
    } else {
        "Unknown".to_string()
    };

    write!(s, "\nPossible CTGP release versions: {}", release_versions).unwrap();

    let (x, y) = FOOTER_INFO_ORIGIN;
    info_paragraph(s, styles::grey_text(), 22.0, x, y)
}

pub fn ctgp_exact_time_info_element<'a>(ctgp_footer: &CTGPFooter) -> Element<'a, Message> {
    use std::fmt::Write;
    let mut s = String::new();

    write!(s, "Exact finish time: {}", ctgp_footer.exact_finish_time()).unwrap();

    write!(s, "\n\nExact lap splits:").unwrap();
    for (idx, exact_lap_time) in ctgp_footer.exact_lap_times().iter().enumerate() {
        write!(s, "\n\tLap {}:\t{}", idx + 1, exact_lap_time).unwrap();
    }

    let (x, y) = FOOTER_INFO_ORIGIN;
    info_paragraph(s, styles::grey_text(), 20.0, x, y)
}

pub fn ctgp_rtc_info_element<'a>(ctgp_footer: &CTGPFooter) -> Element<'a, Message> {
    use std::fmt::Write;
    let mut s = String::new();

    write!(
        s,
        "Time at run start: {}",
        ctgp_footer
            .rtc_race_begins()
            .format("%Y-%m-%d %H:%M:%S%.3f")
    )
    .unwrap();
    write!(
        s,
        "\nTime at run end: {}",
        ctgp_footer.rtc_race_end().format("%Y-%m-%d %H:%M:%S%.3f")
    )
    .unwrap();

    let run_duration = ctgp_footer.rtc_race_end() - ctgp_footer.rtc_race_begins();

    let total_ms = run_duration.num_milliseconds();
    let minutes = total_ms / 60_000;
    let seconds = (total_ms % 60_000) / 1_000;
    let millis = total_ms % 1_000;

    write!(s, "\nRun duration: {}m {}s {}ms", minutes, seconds, millis).unwrap();

    info_paragraph(s, styles::grey_text(), 20.0, 170, 515)
}

pub fn ctgp_pause_info_element<'a>(ctgp_footer: &CTGPFooter) -> Element<'a, Message> {
    use std::fmt::Write;
    let mut s = String::new();

    write!(
        s,
        "Total pause duration: {:.3}s",
        ctgp_footer.rtc_time_paused().num_milliseconds() as f32 / 1000.0
    )
    .unwrap();

    write!(
        s,
        "\nPause input count: {}",
        ctgp_footer.pause_times().len()
    )
    .unwrap();

    if !ctgp_footer.pause_times().is_empty() {
        write!(s, "\n\nPause times:").unwrap();
    }

    info_paragraph(s, styles::grey_text(), 20.0, 700, 185)
}

pub fn ctgp_pause_time_list_element<'a>(ctgp_footer: &CTGPFooter) -> Element<'a, Message> {
    let entries = column(ctgp_footer.pause_times().iter().map(|pause_time| {
        text(pause_time.to_string())
            .font(RODIN_NTLG_PRO_EB)
            .size(20)
            .color(styles::grey_text())
            .into()
    }))
    .spacing(2)
    .width(400);

    let list = scrollable(entries).height(250).width(400);

    positioned(list, 700, 295)
}

pub fn ctgp_final_lap_flag_element<'a>(intersection: bool) -> Element<'a, Message> {
    let s = format!("Final lap dubious intersection? \t{}", intersection);
    let color = if intersection {
        styles::alarm_color()
    } else {
        styles::grey_text()
    };

    info_paragraph(s, color, 20.0, 170, 419)
}

pub fn ctgp_usb_gamecube_element<'a>(enabled: bool) -> Element<'a, Message> {
    let s = format!("USB Gamecube enabled?\t\t\t{}", enabled);
    let color = if enabled {
        styles::notice_color()
    } else {
        styles::grey_text()
    };

    info_paragraph(s, color, 20.0, 170, 445)
}

pub fn ctgp_my_stuff_element<'a>(enabled: bool, used: bool) -> Element<'a, Message> {
    let s = format!(
        "My Stuff enabled?\t\t\t\t{}\nMy Stuff used?\t\t\t\t\t{}",
        enabled, used
    );

    info_paragraph(s, styles::grey_text(), 20.0, 170, 471)
}

pub fn ctgp_anti_tas_deliberately_disabled_element<'a>(disabled: bool) -> Element<'a, Message> {
    let s = format!("Anti-TAS deliberately disabled?\t{}", disabled);
    let color = if disabled {
        styles::alarm_color()
    } else {
        styles::grey_text()
    };

    info_paragraph(s, color, 20.0, 170, 523)
}

pub fn ctgp_race_flags_element<'a>(ctgp_footer: &CTGPFooter) -> Element<'a, Message> {
    use std::fmt::Write;
    let mut s = String::new();

    write!(s, "Respawns?\t\t\t\t\t{}", ctgp_footer.respawns()).unwrap();
    write!(
        s,
        "\nMii name replaced?\t\t\t\t{}",
        ctgp_footer.has_name_replaced()
    )
    .unwrap();
    write!(
        s,
        "\nMii data replaced?\t\t\t\t{}",
        ctgp_footer.has_mii_data_replaced()
    )
    .unwrap();
    write!(s, "\nOut of bounds?\t\t\t\t\t{}", ctgp_footer.went_oob()).unwrap();
    write!(s, "\nCannoned?\t\t\t\t\t{}", ctgp_footer.cannoned()).unwrap();

    let (x, y) = FOOTER_INFO_ORIGIN;
    info_paragraph(s, styles::grey_text(), 20.0, x, y)
}

pub fn ctgp_potentially_cheated_element<'a>(cheated: bool) -> Element<'a, Message> {
    let s = format!("Potentially cheated? \t\t\t{}", cheated);
    let color = if cheated {
        styles::alarm_color()
    } else {
        styles::grey_text()
    };

    info_paragraph(s, color, 20.0, 170, 315)
}

pub fn ctgp_potential_rapidfire_element<'a>(rapidfire: bool) -> Element<'a, Message> {
    let s = format!("Potential rapidfire?\t\t\t\t{}", rapidfire);
    let color = if rapidfire {
        styles::alarm_color()
    } else {
        styles::grey_text()
    };

    info_paragraph(s, color, 20.0, 170, 341)
}

pub fn ctgp_potential_slowdown_element<'a>(slowdown: bool) -> Element<'a, Message> {
    let s = format!("Potential slowdown?\t\t\t{}", slowdown);
    let color = if slowdown {
        styles::alarm_color()
    } else {
        styles::grey_text()
    };

    info_paragraph(s, color, 20.0, 170, 367)
}

pub fn ctgp_lap_dubious_intersection_element<'a>(intersections: &[bool]) -> Element<'a, Message> {
    let x = 720;
    use std::fmt::Write;
    let mut stack = stack![];
    let mut s = String::new();
    write!(s, "Lap split dubious intersections:").unwrap();

    let text = info_paragraph(s, styles::grey_text(), 20.0, x, FOOTER_INFO_ORIGIN.1);
    stack = stack.push(text);

    for (idx, intersection) in intersections.iter().enumerate() {
        let t = format!("\tLap {}:\t {}", idx + 1, intersection);
        let color = if *intersection {
            styles::alarm_color()
        } else {
            styles::grey_text()
        };

        let base = FOOTER_INFO_ORIGIN.1 + 26;

        let text = info_paragraph(t, color, 20.0, x, base + (idx as u32 * 26));
        stack = stack.push(text);
    }
    stack.into()
}

pub fn visit_ctgp_leaderboard_button() -> Element<'static, Message> {
    let btn = visit_button(
        "Visit CTGP Leaderboard",
        Message::OpenCtgpLink(CtgpLink::Leaderboard),
    );
    positioned(btn, 970, 249)
}

pub fn visit_ctgp_ghost_page_button() -> Element<'static, Message> {
    let btn = visit_button("Visit Ghost Page", Message::OpenCtgpLink(CtgpLink::Ghost));
    positioned(btn, 970, 322)
}

pub fn visit_ctgp_player_page_button() -> Element<'static, Message> {
    let btn = visit_button("Visit Player Page", Message::OpenCtgpLink(CtgpLink::Player));
    positioned(btn, 970, 394)
}

pub fn sp_identity_info_element<'a>(sp_footer: &SPFooter) -> Element<'a, Message> {
    use std::fmt::Write;
    let mut s = String::new();
    write!(s, "Footer version: {}", sp_footer.footer_version()).unwrap();
    write!(
        s,
        "\n\nTrack SHA1: {}",
        array_to_hex_string(sp_footer.track_sha1())
    )
    .unwrap();

    let sp_versions_opt = sp_footer.possible_sp_versions();
    let sp_versions = if let Some(sp_versions) = &sp_versions_opt {
        if sp_versions.len() == 1 {
            format!("{}", sp_versions[0])
        } else {
            format!(
                "{} - {}",
                sp_versions[0],
                sp_versions[sp_versions.len() - 1]
            )
        }
    } else {
        "Unknown".to_string()
    };

    write!(s, "\n\nPossible MKW-SP versions: {}", sp_versions).unwrap();

    let (x, y) = FOOTER_INFO_ORIGIN;
    info_paragraph(s, styles::grey_text(), 22.0, x, y)
}

pub fn sp_exact_time_info_element<'a>(sp_footer: &SPFooter) -> Element<'a, Message> {
    use std::fmt::Write;
    let mut s = String::new();

    write!(s, "Exact finish time: {}", sp_footer.exact_finish_time()).unwrap();

    write!(s, "\n\nExact lap splits:").unwrap();
    for (idx, exact_lap_time) in sp_footer.exact_lap_times().iter().enumerate() {
        write!(s, "\n\tLap {}:\t{}", idx + 1, exact_lap_time).unwrap();
    }

    let (x, y) = FOOTER_INFO_ORIGIN;
    info_paragraph(s, styles::grey_text(), 20.0, x, y)
}

pub fn sp_race_settings_element<'a>(sp_footer: &SPFooter) -> Element<'a, Message> {
    use std::fmt::Write;
    let mut s = String::new();

    write!(s, "200cc?\t\t\t\t\t\t{}", sp_footer.is_200cc()).unwrap();

    let vanilla_mode = sp_footer
        .is_vanilla_mode_enabled()
        .map_or("Unknown".to_string(), |b| b.to_string());
    write!(s, "\nVanilla mode enabled?\t\t\t{}", vanilla_mode).unwrap();

    let simplified_controls = sp_footer
        .has_simplified_controls()
        .map_or("Unknown".to_string(), |b| b.to_string());
    write!(
        s,
        "\nSimplified controls enabled?\t\t{}",
        simplified_controls
    )
    .unwrap();

    let mirror = sp_footer
        .set_in_mirror()
        .map_or("Unknown".to_string(), |b| b.to_string());
    write!(s, "\nMirror mode?\t\t\t\t\t{}", mirror).unwrap();

    let (x, y) = FOOTER_INFO_ORIGIN;
    info_paragraph(s, styles::grey_text(), 20.0, x, y)
}

pub fn sp_ultra_shortcut_element<'a>(has_ultra_shortcut: bool) -> Element<'a, Message> {
    let s = format!("Ultra shortcut performed?\t\t{}", has_ultra_shortcut);
    let color = if has_ultra_shortcut {
        styles::alarm_color()
    } else {
        styles::grey_text()
    };

    info_paragraph(s, color, 20.0, 170, 315)
}

pub fn sp_horizontal_wall_glitch_element<'a>(
    has_horizontal_wall_glitch: bool,
) -> Element<'a, Message> {
    let s = format!(
        "Horizontal wall glitch performed?\t{}",
        has_horizontal_wall_glitch
    );
    let color = if has_horizontal_wall_glitch {
        styles::alarm_color()
    } else {
        styles::grey_text()
    };

    info_paragraph(s, color, 20.0, 170, 341)
}

pub fn sp_wallride_element<'a>(has_wallride: bool) -> Element<'a, Message> {
    let s = format!("Wallride performed?\t\t\t\t{}", has_wallride);
    let color = if has_wallride {
        styles::alarm_color()
    } else {
        styles::grey_text()
    };

    info_paragraph(s, color, 20.0, 170, 367)
}

pub fn track_name_text<'a>(
    ghost: &'a Ghost,
    custom_track_name: Option<String>,
) -> Element<'a, Message> {
    use std::fmt::Write;

    let mut track_name = if let Some(c) = custom_track_name {
        c
    } else {
        ghost.header().slot_id().to_string()
    };

    match ghost.footer() {
        Some(FooterType::CTGPFooter(ctgp_footer)) => {
            let category_string = match ctgp_footer.category() {
                Category::Glitch => String::from("(Glitch)"),
                Category::NoShortcut => String::from("(No Shortcut)"),
                Category::Normal => String::from("(Normal)"),
                Category::Shortcut => String::from("(Shortcut)"),
                Category::NoShortcutTAS => String::from("(No Shortcut) (TAS)"),
                Category::NormalTAS => String::from("(Normal) (TAS)"),
                Category::ShortcutTAS => String::from("(Shortcut) (TAS)"),
                Category::GlitchTAS => String::from("(Glitch) (TAS)"),
                Category::NoShortcut200cc => String::from("(200cc) (No Shortcut)"),
                Category::Normal200cc => String::from("(200cc) (Normal)"),
                Category::Shortcut200cc => String::from("(200cc) (Shortcut)"),
                Category::Glitch200cc => String::from("(200cc) (Glitch)"),
                Category::NoShortcut200ccTAS => String::from("(200cc) (No Shortcut) (TAS)"),
                Category::Normal200ccTAS => String::from("(200cc) (Normal) (TAS)"),
                Category::Shortcut200ccTAS => String::from("(200cc) (Shortcut) (TAS)"),
                Category::Glitch200ccTAS => String::from("(200cc) (Glitch) (TAS)"),
            };

            write!(track_name, " {}", category_string).unwrap();
        }
        Some(FooterType::SPFooter(sp_footer)) => {
            let mut category_string = String::new();
            if sp_footer.is_200cc() {
                category_string.push_str(" (200cc)");
            }
            if let Some(mirror) = sp_footer.set_in_mirror()
                && mirror
            {
                category_string.push_str(" (Mirror)")
            }
            if sp_footer.has_ultra_shortcut() {
                category_string.push_str(" (Glitch)");
            }

            write!(track_name, "{}", category_string).unwrap();
        }
        _ => (),
    }

    let t = FitText {
        content: track_name,
        font: RODIN_NTLG_PRO_EB,
        max_size: 32.0,
        min_size: 1.0,
        width: 548.0,
        height: 39.0,
    };

    positioned(iced::Element::new(t), 365, 154)
}

pub fn finish_time_text(finish_time: InGameTime) -> Element<'static, Message> {
    let t = text(finish_time.to_string())
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .width(548)
        .font(RODIN_NTLG_PRO_EB)
        .size(32);
    positioned(t, 365, 205)
}

pub fn mii_name_text(mii_name: &str) -> Element<'static, Message> {
    let t = FitText {
        content: mii_name.to_owned(),
        font: CTMKF,
        max_size: 26.0,
        min_size: 8.0,
        width: 216.0,
        height: 33.0,
    };

    positioned(iced::Element::new(t), 448, 257)
}

pub fn country_element<'a>(ghost: &'a Ghost, handle: &'a svg::Handle) -> Element<'a, Message> {
    let country_image = svg(handle.clone()).height(32).width(Length::Shrink);

    let tooltip_text = text(format!(
        "{} ({})",
        ghost.header().location().country(),
        ghost.header().location().subregion(),
    ))
    .font(RODIN_NTLG_PRO_EB);

    let img_with_tooltip = tooltip(
        country_image,
        container(tooltip_text)
            .padding(4)
            .style(styles::tooltip_style()),
        tooltip::Position::FollowCursor,
    )
    .delay(Duration::from_millis(500));

    positioned(img_with_tooltip, 534, 300)
}

pub fn character_element<'a>(ghost: &'a Ghost, handle: &'a image::Handle) -> Element<'a, Message> {
    let tooltip_text = text(ghost.header().combo().character().to_string()).font(RODIN_NTLG_PRO_EB);

    let img_with_tooltip = tooltip(
        image(handle.clone()).height(128.0 * 0.6),
        container(tooltip_text)
            .padding(4)
            .style(styles::tooltip_style()),
        tooltip::Position::FollowCursor,
    )
    .delay(Duration::from_millis(500));

    positioned(img_with_tooltip, 678, 255)
}

pub fn vehicle_element<'a>(ghost: &'a Ghost, handle: &'a image::Handle) -> Element<'a, Message> {
    use std::fmt::Write;
    let mut t = format!(
        "{} ({})",
        ghost.header().combo().vehicle(),
        if ghost.header().is_automatic_drift() {
            "Automatic"
        } else {
            "Manual"
        },
    );

    if ghost.header().transmission_mod() != TransmissionMod::Vanilla {
        write!(t, " ({})", ghost.header().transmission_mod()).unwrap();
    }

    let tooltip_text = text(t)
    .font(RODIN_NTLG_PRO_EB);

    let img_with_tooltip = tooltip(
        image(handle.clone()).height(100.0 * 0.76),
        container(tooltip_text)
            .padding(4)
            .style(styles::tooltip_style()),
        tooltip::Position::FollowCursor,
    )
    .delay(Duration::from_millis(500));

    positioned(img_with_tooltip, 765, 256)
}

pub fn lap_splits_box<'a>(lap_splits: &[InGameTime]) -> Element<'a, Message> {
    use std::fmt::Write;

    let Some((first, rest)) = lap_splits.split_first() else {
        return positioned(Space::new(), 30, 135);
    };

    let mut lap_splits_text = format!("Lap 1:   {}", first);

    for (idx, lap) in rest.iter().enumerate() {
        write!(
            lap_splits_text,
            "\nLap {}:{}{}",
            idx + 2,
            if idx + 2 < 10 { "   " } else { " " },
            lap
        )
        .unwrap();
    }

    // adjust lap split box size based on number of laps
    let (size, x_offset): (f32, u32) = match lap_splits.len() {
        1..=6 => (25.5, 30),
        7 => (21.9, 66),
        8 => (19.1, 94),
        9 => (17.05, 114),
        10 => (15.3, 132),
        _ => (13.95, 145),
    };

    let lap_splits_element = container(
        text(lap_splits_text)
            .font(RODIN_NTLG_PRO_EB)
            .align_x(Alignment::End)
            .align_y(Alignment::Center)
            .color(Color::WHITE)
            .size(size),
    )
    .padding(10)
    .style(styles::info_box_style());

    positioned(lap_splits_element, x_offset, 135)
}

pub fn mii_info_box<'a>(mii: &'a Mii) -> Element<'a, Message> {
    let font_size = if cfg!(target_os = "macos") {
        10.5
    } else {
        14.0
    };
    let element_y_pos = if cfg!(target_os = "macos") { 430 } else { 391 };

    let label_col = column![
        mii_label("Mii Info", font_size * 1.5),
        mii_label("Creator:", font_size),
        mii_label("Creation date:", font_size),
        mii_label("Type:", font_size),
        mii_label("Gender:", font_size),
        mii_label("Birthday:", font_size),
        mii_label("Favorite color:", font_size),
        mii_label("Favorite Mii?", font_size),
    ];

    let birthday = if let Some(month) = mii.birthday().month()
        && let Some(day) = mii.birthday().day()
    {
        format!("{:0>2}/{:0>2}", month, day)
    } else {
        String::from("Not set")
    };

    let value_col = column![
        mii_value("", font_size * 1.5),
        mii_value(
            if !mii.creator_name().is_empty() {
                mii.creator_name()
            } else {
                "—"
            },
            font_size
        ),
        mii_value(mii.creation_date(), font_size),
        mii_value(mii.mii_type(), font_size),
        mii_value(if mii.is_girl() { "Female" } else { "Male" }, font_size),
        mii_value(birthday, font_size),
        mii_value(favorite_color_string(mii.favorite_color()), font_size),
        mii_value(mii.is_favorite(), font_size),
    ]
    .align_x(Alignment::End)
    .width(149);

    let content = row![label_col, value_col].spacing(10);

    let mii_info_element = container(content)
        .padding(10)
        .style(styles::info_box_style());

    positioned(mii_info_element, 30, element_y_pos)
}

pub fn shroomstrat_box<'a>(shroomstrat: Shroomstrat) -> Element<'a, Message> {
    let len = max(shroomstrat.to_string().len(), 5);
    let text_width = len as f32 * 13.5 + 22.5;

    let shroomstrat_text = text(shroomstrat.to_string())
        .font(RODIN_NTLG_PRO_EB)
        .color(Color::WHITE)
        .size(24)
        .width(text_width)
        .height(80)
        .align_x(Alignment::Center)
        .align_y(Alignment::End);

    let shroomstrat_element = container(shroomstrat_text)
        .padding(10)
        .style(styles::info_box_style());

    positioned(shroomstrat_element, 960, 135)
}

pub fn shroom_element<'a>(shroomstrat: Shroomstrat) -> Element<'a, Message> {
    let handle = image::Handle::from_bytes(MUSHROOM);
    let shroom_image = image(handle.clone()).height(55);

    let len = max(shroomstrat.to_string().len(), 5);
    let x_position = (len as f32 * 6.75 + 955.0) as u32;

    positioned(shroom_image, x_position, 141)
}

pub fn mii_import_button() -> Element<'static, Message> {
    let x_pos = if cfg!(target_os = "linux") { 315 } else { 310 };

    positioned(
        ghost_action_button("Import Mii", true, Message::MiiImport),
        x_pos,
        480,
    )
}

pub fn mii_export_button() -> Element<'static, Message> {
    let x_pos = if cfg!(target_os = "linux") { 315 } else { 310 };

    positioned(
        ghost_action_button("Export Mii", true, Message::MiiExport),
        x_pos,
        526,
    )
}

pub fn date_set_box<'a>(date: &'a Date) -> Element<'a, Message> {
    let date_set_text = text(format!("Date set:\n{}", date.to_string()))
        .font(RODIN_NTLG_PRO_EB)
        .color(Color::WHITE)
        .size(20)
        .width(150)
        .height(50)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center);

    let date_set_element = container(date_set_text)
        .padding(10)
        .style(styles::info_box_style());

    positioned(date_set_element, 331, 370)
}

pub fn ghost_type_box<'a>(ghost: &'a Ghost) -> Element<'a, Message> {
    let ghost_text = format!(
        "Ghost type: 0x{:0>2X}\n{}",
        u8::from(ghost.header().ghost_type()),
        ghost.header().ghost_type().to_string()
    );

    let ghost_type_text = text(ghost_text)
        .font(RODIN_NTLG_PRO_EB)
        .color(Color::WHITE)
        .size(20)
        .width(243)
        .height(50)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center);

    let ghost_type_element = container(ghost_type_text)
        .padding(10)
        .style(styles::info_box_style());

    positioned(ghost_type_element, 508, 370)
}

pub fn controller_box(controller: Controller) -> Element<'static, Message> {
    let date_set_text = text(format!("Controller:\n{}", controller.to_string()))
        .font(RODIN_NTLG_PRO_EB)
        .color(Color::WHITE)
        .size(20)
        .width(150)
        .height(50)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center);

    let date_set_element = container(date_set_text)
        .padding(10)
        .style(styles::info_box_style());

    positioned(date_set_element, 778, 370)
}

pub fn external_footer_button<'a>(ghost: &'a Ghost) -> Option<Element<'a, Message>> {
    let label = match ghost.footer()? {
        FooterType::CTGPFooter(_) => "CTGP ghost",
        FooterType::SPFooter(_) => "MKW-SP ghost",
        FooterType::Unknown(_) => "Unknown",
    };

    let btn = button(text(label).font(RODIN_NTLG_PRO_EB).size(16).center())
        .width(263)
        .height(COMMON_BUTTON_HEIGHT)
        .on_press(Message::ToggleFooterInfoMenu)
        .style(styles::common_button_theme());

    Some(positioned(btn, 508, 491))
}

pub fn mii_image_element<'a>(handle: &'a image::Handle) -> Element<'a, Message> {
    let crop = Rectangle {
        width: 270,
        height: 229,
        x: 0,
        y: 0,
    };

    let img = image(handle).crop(crop).width(115);

    positioned(img, 345, 234)
}

fn edit_label<'a>(label: impl Into<String>) -> Element<'a, Message> {
    text(label.into())
        .font(RODIN_NTLG_PRO_EB)
        .size(18)
        .color(styles::grey_text())
        .width(200)
        .into()
}

fn edit_row<'a>(
    label: impl Into<String>,
    control: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    row![edit_label(label), control.into()]
        .spacing(12)
        .align_y(Alignment::Center)
        .into()
}


pub fn edit_form<'a>(ghost: &'a Ghost, buffers: &'a EditBuffers) -> Element<'a, Message> {
    let header = ghost.header();

    let track_picker = pick_list(SLOT_IDS, Some(header.slot_id()), Message::EditSlotIdSelected)
        .width(260)
        .text_size(16);
    let character_picker = pick_list(
        CHARACTERS,
        Some(header.combo().character()),
        Message::EditCharacterSelected,
    )
    .width(260)
    .text_size(16);
    let compatible_vehicles: Vec<_> = VEHICLES
        .into_iter()
        .filter(|v| v.get_weight_class() == header.combo().character().get_weight_class())
        .collect();
    let vehicle_picker = pick_list(
        compatible_vehicles,
        Some(header.combo().vehicle()),
        Message::EditVehicleSelected,
    )
    .width(260)
    .text_size(16);
    let controller_picker = pick_list(
        CONTROLLERS,
        Some(header.controller()),
        Message::EditControllerSelected,
    )
    .width(150)
    .text_size(16);
    let transmission_picker = pick_list(
        TRANSMISSION_MODS,
        Some(header.transmission_mod()),
        Message::EditTransmissionModSelected,
    )
    .width(180)
    .text_size(16);
    let ghost_type_picker = pick_list(
        GHOST_TYPES,
        Some(header.ghost_type()),
        Message::EditGhostTypeSelected,
    )
    .width(260)
    .text_size(16);

    let drift_checkbox = checkbox(header.is_automatic_drift())
        .label("Automatic drift")
        .on_toggle(Message::EditAutomaticDriftToggled)
        .style(|theme, status| checkbox::Style {
            text_color: Some(Color::BLACK),
            ..checkbox::primary(theme, status)
        });

    let finish_time_input = text_input("MM:SS.mmm", &buffers.finish_time)
        .on_input(Message::EditFinishTimeChanged)
        .width(140);

    let date_input = text_input("YYYY-MM-DD", &buffers.date)
        .on_input(Message::EditDateChanged)
        .width(140);

    let current_country = header.location().country();
    let current_subregion = header.location().subregion();

    let countries: Vec<_> = edit_data::location_table()
        .iter()
        .map(|entry| entry.country)
        .collect();
    let country_picker = pick_list(countries, Some(current_country), Message::EditCountrySelected)
        .width(260)
        .text_size(16);

    let subregions: Vec<_> = edit_data::location_table()
        .iter()
        .find(|entry| entry.country == current_country)
        .map(|entry| entry.options.iter().map(|o| o.subregion).collect())
        .unwrap_or_default();
    let subregion_picker = pick_list(
        subregions,
        Some(current_subregion),
        Message::EditSubregionSelected,
    )
    .width(220)
    .text_size(16);

    let location_row = row![edit_label("Location"), country_picker, subregion_picker,]
        .spacing(12)
        .align_y(Alignment::Center);

    let mut fields = column![
        edit_row("Track", track_picker),
        edit_row("Character", character_picker),
        edit_row("Vehicle", vehicle_picker),
        edit_row("Controller", controller_picker),
        edit_row("Transmission mod", transmission_picker),
        edit_row("Ghost type", ghost_type_picker),
        edit_row("", drift_checkbox),
        edit_row("Finish time", finish_time_input),
        edit_row("Date set", date_input),
        location_row,
    ]
    .spacing(14);

    if !buffers.lap_splits.is_empty() {
        fields = fields.push(
            text("Lap splits")
                .font(RODIN_NTLG_PRO_EB)
                .size(18)
                .color(styles::grey_text()),
        );

        for (idx, buffer) in buffers.lap_splits.iter().enumerate() {
            let input = text_input("MM:SS.mmm", buffer)
                .on_input(move |s| Message::EditLapSplitChanged(idx, s))
                .width(140);
            fields = fields.push(edit_row(format!("Lap {}", idx + 1), input));
        }
    }

    let scrollable_form = scrollable(fields).height(400).width(1000);

    positioned(scrollable_form, 170, 130)
}

/// Positions a widget so that its *center* (not top-left corner) lands at
/// `(cx, cy)`, given its own `(width, height)`.
fn positioned_centered<'a, M: 'a>(
    widget: impl Into<Element<'a, M>>,
    cx: f32,
    cy: f32,
    width: f32,
    height: f32,
) -> Element<'a, M> {
    positioned(
        widget,
        (cx - width / 2.0).round() as u32,
        (cy - height / 2.0).round() as u32,
    )
}

/// A pill- or circle-shaped, non-interactive state indicator for one of the
/// controller's face buttons, with no label.
fn face_shape<'a>(active: bool, width: f32, radius: f32) -> Element<'a, Message> {
    button(Space::new())
        .width(width)
        .height(radius * 2.0)
        .style(move |_, _| styles::capsule_button_style(active, radius))
        .into()
}

/// Pixels a black backing shape extends beyond the white shape it sits
/// behind, so a thin black edge is visible all the way around.
const FACE_OUTLINE_PAD: f32 = 3.0;

/// Where and how big a controller face element (button-based shape or
/// canvas) is: centered at `(cx, cy)`, `width`×`height` in size, with
/// corner `radius` (`height / 2.0` for a capsule, `width / 2.0` for a
/// circle).
#[derive(Clone, Copy)]
struct FaceGeometry {
    cx: f32,
    cy: f32,
    width: f32,
    height: f32,
    radius: f32,
}

/// Wraps a face shape sized per `geometry` with a solid black backing a few
/// pixels larger, plus a thin black ring just inside its own white border
/// (drawn on top, so it stays visible even when the shape fills solid white
/// for its active state), then centers the whole thing per `geometry`.
fn outlined_at<'a>(inner: Element<'a, Message>, geometry: FaceGeometry) -> Element<'a, Message> {
    let FaceGeometry {
        cx,
        cy,
        width,
        height,
        radius,
    } = geometry;

    let outer_w = width + FACE_OUTLINE_PAD * 2.0;
    let outer_h = height + FACE_OUTLINE_PAD * 2.0;

    let backing = button(Space::new())
        .width(outer_w)
        .height(outer_h)
        .style(move |_, _| styles::black_backing_style(radius + FACE_OUTLINE_PAD, FACE_OUTLINE_PAD));

    let centered_face = container(inner).center(Length::Fill);

    let ring_w = (width - styles::FACE_BORDER_WIDTH * 2.0).max(0.0);
    let ring_h = (height - styles::FACE_BORDER_WIDTH * 2.0).max(0.0);
    let ring_radius = (radius - styles::FACE_BORDER_WIDTH).max(0.0);
    let inner_ring = button(Space::new())
        .width(ring_w)
        .height(ring_h)
        .style(move |_, _| styles::inner_ring_style(ring_radius));
    let centered_ring = container(inner_ring).center(Length::Fill);

    positioned_centered(
        stack!(backing, centered_face, centered_ring),
        cx,
        cy,
        outer_w,
        outer_h,
    )
}

/// Same as [`outlined_at`], but for a face element with a text label: the
/// label is rendered as its own topmost layer, sized to fit strictly inside
/// the inner ring's own hole (`ring_w`/`ring_h` from `outlined_at`, minus
/// the ring's own thickness) and centered on the same point as everything
/// else — so it can't visually collide with that ring no matter how the two
/// are laid out.
fn outlined_button_at<'a>(
    label: &'static str,
    active: bool,
    text_size: f32,
    geometry: FaceGeometry,
) -> Element<'a, Message> {
    let FaceGeometry {
        cx,
        cy,
        width,
        height,
        radius,
    } = geometry;

    let shape = outlined_at(face_shape(active, width, radius), geometry);

    let ring_w = (width - styles::FACE_BORDER_WIDTH * 2.0).max(0.0);
    let ring_h = (height - styles::FACE_BORDER_WIDTH * 2.0).max(0.0);
    let text_w = (ring_w - styles::INNER_RING_WIDTH * 2.0).max(0.0);
    let text_h = (ring_h - styles::INNER_RING_WIDTH * 2.0).max(0.0);

    let text_color = if active { Color::BLACK } else { Color::WHITE };
    let label_el = text(label)
        .font(RODIN_NTLG_PRO_EB)
        .size(text_size)
        .color(text_color)
        .width(text_w)
        .height(text_h)
        .center();

    let label_positioned = positioned_centered(label_el, cx, cy, text_w, text_h);

    stack!(shape, label_positioned)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn input_box_background<'a>(handle: image::Handle) -> Element<'a, Message> {
    let img = image(handle).width(BOX_W).height(BOX_H);
    positioned(img, BOX_X as u32, BOX_Y as u32)
}

const BOX_X: f32 = 145.0;
const BOX_Y: f32 = 200.0;
const BOX_W: f32 = 545.0;
const BOX_H: f32 = 350.0;


fn controller_face<'a>(current: ControllerInput, is_drifting: bool) -> Element<'a, Message> {
    const DPAD_SIZE: f32 = 100.0;
    const STICK_SIZE: f32 = 170.0;
    const ACCEL_SIZE: f32 = 90.0;
    const SHOULDER_W: f32 = 130.0;
    const SHOULDER_H: f32 = 34.0;
    const SECONDARY_W: f32 = 90.0;
    const SECONDARY_H: f32 = 34.0;
    const GAP: f32 = 18.0;

    const COL1: f32 = 267.0;
    const COL2: f32 = COL1 + DPAD_SIZE / 2.0 + GAP + STICK_SIZE / 2.0;
    const COL3: f32 = COL2 + STICK_SIZE / 2.0 + GAP + ACCEL_SIZE / 2.0;
    const SHOULDER_ROW: f32 = 255.0;
    const MAIN_ROW: f32 = SHOULDER_ROW + SHOULDER_H / 2.0 + GAP + STICK_SIZE / 2.0;
    const SECONDARY_ROW: f32 = MAIN_ROW + STICK_SIZE / 2.0 + GAP + SECONDARY_H / 2.0;

    let item_shoulder = outlined_at(
        face_shape(current.item(), SHOULDER_W, SHOULDER_H / 2.0),
        FaceGeometry {
            cx: COL1,
            cy: SHOULDER_ROW,
            width: SHOULDER_W,
            height: SHOULDER_H,
            radius: SHOULDER_H / 2.0,
        },
    );
    let brake_shoulder = outlined_at(
        face_shape(current.brake(), SHOULDER_W, SHOULDER_H / 2.0),
        FaceGeometry {
            cx: COL3,
            cy: SHOULDER_ROW,
            width: SHOULDER_W,
            height: SHOULDER_H,
            radius: SHOULDER_H / 2.0,
        },
    );

    let dpad = positioned_centered(
        canvas(DPadCanvas {
            dpad: current.dpad(),
        })
        .width(DPAD_SIZE)
        .height(DPAD_SIZE),
        COL1,
        MAIN_ROW,
        DPAD_SIZE,
        DPAD_SIZE,
    );
    let stick = positioned_centered(
        canvas(StickCanvas {
            stick: current.stick(),
        })
        .width(STICK_SIZE)
        .height(STICK_SIZE),
        COL2,
        MAIN_ROW,
        STICK_SIZE,
        STICK_SIZE,
    );
    let accelerator = outlined_at(
        face_shape(current.accelerator(), ACCEL_SIZE, ACCEL_SIZE / 2.0),
        FaceGeometry {
            cx: COL3,
            cy: MAIN_ROW,
            width: ACCEL_SIZE,
            height: ACCEL_SIZE,
            radius: ACCEL_SIZE / 2.0,
        },
    );

    let drift = outlined_button_at(
        "Drift",
        is_drifting,
        11.0,
        FaceGeometry {
            cx: COL1,
            cy: SECONDARY_ROW,
            width: SECONDARY_W,
            height: SECONDARY_H,
            radius: SECONDARY_H / 2.0,
        },
    );
    let brake_drift = outlined_button_at(
        "B.Drift",
        current.brake_drift(),
        11.0,
        FaceGeometry {
            cx: COL2,
            cy: SECONDARY_ROW,
            width: SECONDARY_W,
            height: SECONDARY_H,
            radius: SECONDARY_H / 2.0,
        },
    );
    let pause = outlined_button_at(
        "Pause",
        current.pause(),
        11.0,
        FaceGeometry {
            cx: COL3,
            cy: SECONDARY_ROW,
            width: SECONDARY_W,
            height: SECONDARY_H,
            radius: SECONDARY_H / 2.0,
        },
    );

    stack!(
        brake_shoulder,
        item_shoulder,
        dpad,
        stick,
        accelerator,
        drift,
        brake_drift,
        pause,
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn format_race_time(total_seconds: f64) -> String {
    let sign = if total_seconds < 0.0 { "-" } else { "" };
    let total_millis = (total_seconds.abs() * 1000.0).round() as u64;
    let minutes = total_millis / 60_000;
    let seconds = (total_millis % 60_000) / 1_000;
    let millis = total_millis % 1_000;
    format!("{sign}{:02}:{:02}.{:03}", minutes, seconds, millis)
}

fn frame_info_box<'a>(
    current_input: ControllerInput,
    current_frame: u32,
    total_frames: u32,
) -> Element<'a, Message> {
    use std::fmt::Write;

    let mut s = String::new();
    write!(s, "Frame: {} / {}", current_frame, total_frames).unwrap();

    let time_secs = (current_frame as f64 - input_playback::RACE_START_FRAME as f64)
        / input_playback::FRAME_RATE;
    write!(s, "\nTime: {}", format_race_time(time_secs)).unwrap();

    write!(
        s,
        "\nStick: ({}, {})",
        current_input.stick().x() as i8 - 7,
        current_input.stick().y() as i8 - 7
    )
    .unwrap();

    write!(s, "\nHeld for: {} frames", current_input.frame_duration()).unwrap();

    let text_el = text(s)
        .font(RODIN_NTLG_PRO_EB)
        .color(Color::WHITE)
        .size(18)
        .width(300)
        .height(94);

    let box_el = container(text_el).padding(10).style(styles::info_box_style());

    positioned(box_el, 710, 200)
}

fn seek_slider<'a>(current_frame: u32, total_frames: u32) -> Element<'a, Message> {
    // Stop well short of the close button (`CLOSE_BUTTON_POS.0`) so the
    // slider's full-width rail and thumb never render underneath it.
    let width = CLOSE_BUTTON_POS.0 as f32 - 170.0 - 30.0;
    let s = slider(1..=total_frames.max(1), current_frame, Message::InputSeek).width(width);
    positioned(s, 170, 562)
}

fn transport_button<'a>(label: &'static str, msg: Message, enabled: bool) -> Element<'a, Message> {
    let btn = button(text(label).font(RODIN_NTLG_PRO_EB).size(16).center())
        .width(50)
        .height(36);

    if enabled {
        btn.on_press(msg).style(styles::common_button_theme()).into()
    } else {
        btn.style(|_, _| styles::disabled_button_style()).into()
    }
}

fn transport_controls<'a>(
    is_playing: bool,
    current_frame: u32,
    total_frames: u32,
) -> Element<'a, Message> {
    let jump_start = transport_button("|<", Message::InputJumpToStart, current_frame > 1);
    let step_back = transport_button("<", Message::InputStepFrame(-1), current_frame > 1);

    let play_pause = button(
        text(if is_playing { "Pause" } else { "Play" })
            .font(RODIN_NTLG_PRO_EB)
            .size(16)
            .center(),
    )
    .width(90)
    .height(36)
    .on_press(Message::ToggleInputPlayback)
    .style(styles::common_button_theme());

    let step_fwd = transport_button(">", Message::InputStepFrame(1), current_frame < total_frames);
    let jump_end = transport_button(">|", Message::InputJumpToEnd, current_frame < total_frames);

    let controls = row![jump_start, step_back, play_pause, step_fwd, jump_end].spacing(8);
    positioned(controls, 170, 599)
}

fn speed_picker<'a>(speed: PlaybackSpeed) -> Element<'a, Message> {
    let label = text("Speed")
        .font(RODIN_NTLG_PRO_EB)
        .size(16)
        .color(styles::grey_text());
    let picker = pick_list(PLAYBACK_SPEEDS, Some(speed), Message::InputSpeedSelected)
        .width(90)
        .text_size(16);

    let row_el = row![label, picker].spacing(8).align_y(Alignment::Center);
    positioned(row_el, 540, 605)
}

pub fn input_viewer<'a>(
    ghost: &'a Ghost,
    playback: &'a InputPlayback,
    effective_drift: &'a [bool],
    input_box_handle: image::Handle,
) -> Element<'a, Message> {
    let input_data = ghost.input_data();
    let total_frames = input_data.total_frame_duration();
    let current_frame = playback.current_frame;

    let idx = input_data.input_index_at_frame(current_frame);
    let current_input = idx
        .and_then(|i| input_data.controller_inputs().get(i))
        .copied()
        .unwrap_or_default();
    let is_drifting = idx
        .and_then(|i| effective_drift.get(i))
        .copied()
        .unwrap_or(false);

    let title = positioned(
        text("Input Data")
            .font(RODIN_NTLG_PRO_EB)
            .size(26)
            .color(styles::grey_text()),
        170,
        140,
    );

    stack!(
        title,
        input_box_background(input_box_handle),
        controller_face(current_input, is_drifting),
        frame_info_box(current_input, current_frame, total_frames),
        seek_slider(current_frame, total_frames),
        transport_controls(playback.is_playing, current_frame, total_frames),
        speed_picker(playback.speed),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
