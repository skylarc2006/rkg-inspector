use iced::{
    Alignment, Color, Element, Length, Rectangle,
    widget::{Button, Image, button, column, container, image, row, stack, svg, text, tooltip},
};
use rkg_utils::{
    Ghost,
    footer::{FooterType, ctgp_footer},
    header::{
        controller::Controller, date::Date, in_game_time::InGameTime, mii::Mii, slot_id::SlotId,
    },
};
use std::time::Duration;

use crate::{
    helpers::{array_to_hex_string, disc_region_string, favorite_color_string},
    message::Message,
    ui::{
        constants::{CTMKF, RODIN_NTLG_PRO_EB, VERSION},
        fit_text::FitText,
        footer_tab::FooterTab,
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
        btn.on_press(msg).style(|_, status| match status {
            button::Status::Hovered => styles::hovered_button_style(),
            _ => styles::common_button_style(),
        })
    } else {
        btn.style(|_, _| styles::disabled_button_style())
    }
}

pub fn background(
    background_handle: image::Handle,
    ghost_box_handle: image::Handle,
) -> Element<'static, Message> {
    let background_image = image(background_handle).scale(1.0);
    let ghost_box: Image = image(ghost_box_handle).scale(0.5);
    stack!(background_image, ghost_box).into()
}

pub fn info_background(info_background_handle: image::Handle) -> Element<'static, Message> {
    image(info_background_handle).scale(0.85).into()
}

pub fn prerelease_warning_text() -> Element<'static, Message> {
    let t = text("WARNING: this is an unfinished pre-release version for testing! ")
        .color(Color::from_rgba8(128, 128, 128, 1.0))
        .align_x(Alignment::End)
        .align_y(Alignment::Start)
        .width(Length::Fill)
        .font(RODIN_NTLG_PRO_EB)
        .size(14);
    positioned(t, 0, 5)
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
    .style(|_, status| match status {
        button::Status::Hovered => styles::hovered_button_style(),
        _ => styles::common_button_style(),
    });
    positioned(btn, 507, 80)
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
    let btn = button(text("Close").font(RODIN_NTLG_PRO_EB).size(28).center())
        .width(COMMON_BUTTON_WIDTH as f32 * 1.5)
        .height(COMMON_BUTTON_HEIGHT as f32 * 1.5)
        .on_press(Message::ToggleEditMenu)
        .style(|_, status| match status {
            button::Status::Hovered => styles::hovered_button_style(),
            _ => styles::common_button_style(),
        });

    positioned(btn, 892, 551)
}

pub fn ctgp_footer_identity_button(is_active: bool) -> Element<'static, Message> {
    let btn = button(text("Identity").font(RODIN_NTLG_PRO_EB).size(28).center())
        .width(COMMON_BUTTON_WIDTH as f32 * 1.25)
        .height(COMMON_BUTTON_HEIGHT as f32 * 1.25)
        .on_press(Message::SetActiveFooterTab(FooterTab::CtgpIdentity))
        .style(move |_, status| match status {
            button::Status::Hovered => styles::hovered_red_green_button_style(is_active),
            _ => styles::red_green_button_style(is_active),
        });

    positioned(btn, 170, 115)
}

pub fn ctgp_footer_time_info_button(is_active: bool) -> Element<'static, Message> {
    let btn = button(text("Time").font(RODIN_NTLG_PRO_EB).size(28).center())
        .width(COMMON_BUTTON_WIDTH as f32 * 1.25)
        .height(COMMON_BUTTON_HEIGHT as f32 * 1.25)
        .on_press(Message::SetActiveFooterTab(FooterTab::CtgpTimeInfo))
        .style(move |_, status| match status {
            button::Status::Hovered => styles::hovered_red_green_button_style(is_active),
            _ => styles::red_green_button_style(is_active),
        });

    positioned(btn, 375, 115)
}

pub fn ctgp_footer_race_events_button(is_active: bool) -> Element<'static, Message> {
    let btn = button(
        text("Race Events")
            .font(RODIN_NTLG_PRO_EB)
            .size(22)
            .center(),
    )
    .width(COMMON_BUTTON_WIDTH as f32 * 1.25)
    .height(COMMON_BUTTON_HEIGHT as f32 * 1.25)
    .on_press(Message::SetActiveFooterTab(FooterTab::CtgpRaceEvents))
    .style(move |_, status| match status {
        button::Status::Hovered => styles::hovered_red_green_button_style(is_active),
        _ => styles::red_green_button_style(is_active),
    });

    positioned(btn, 580, 115)
}

pub fn close_footer_info_button() -> Element<'static, Message> {
    let btn = button(text("Close").font(RODIN_NTLG_PRO_EB).size(28).center())
        .width(COMMON_BUTTON_WIDTH as f32 * 1.5)
        .height(COMMON_BUTTON_HEIGHT as f32 * 1.5)
        .on_press(Message::ToggleFooterInfoMenu)
        .style(|_, status| match status {
            button::Status::Hovered => styles::hovered_button_style(),
            _ => styles::common_button_style(),
        });

    positioned(btn, 892, 551)
}

pub fn footer_info_text<'a>(
    active_footer_tab: FooterTab,
    ghost: &'a Ghost,
) -> Element<'a, Message> {
    use std::fmt::Write;
    let mut footer_info_view = stack!();

    if let Some(footer) = &ghost.footer() {
        match footer {
            &FooterType::CTGPFooter(ctgp_footer) => match active_footer_tab {
                FooterTab::CtgpIdentity => {
                    let mut footer_info_text = String::new();
                    write!(footer_info_text, "Footer version: {}", ctgp_footer.footer_version()).unwrap();
                    write!(footer_info_text, "\nTrack SHA1: {}", array_to_hex_string(ctgp_footer.track_sha1())).unwrap();
                    write!(footer_info_text, "\nPlayer ID: {}", array_to_hex_string(&ctgp_footer.player_id().to_be_bytes())).unwrap();

                    if let Some(disc_region) = &ctgp_footer.disc_region() {
                        write!(footer_info_text, "\nDisc region: {}", disc_region_string(disc_region)).unwrap();
                    }

                    write!(footer_info_text, "\n\nCategory: {}", ctgp_footer.category()).unwrap();
                    write!(footer_info_text, "\n\nCTGP CORE version: {}", ctgp_footer.core_version()).unwrap();

                    let ctgp_versions_opt = ctgp_footer.possible_ctgp_versions();
                    let release_versions = if let Some(ctgp_versions) = &ctgp_versions_opt {
                        if ctgp_versions.len() == 1 {
                            format!("{}", ctgp_versions[0])
                        } else {
                            format!("{} - {}", ctgp_versions[0], ctgp_versions[ctgp_versions.len() - 1])
                        }
                    } else {
                        "Unknown".to_string()
                    };

                    write!(footer_info_text, "\nPossible CTGP release versions: {}", release_versions).unwrap();

                    let footer_info_text = text(footer_info_text)
                        .color(Color::from_rgba8(128, 128, 128, 1.0))
                        .align_x(Alignment::Start)
                        .align_y(Alignment::Start)
                        .width(930)
                        .font(RODIN_NTLG_PRO_EB)
                        .size(26);

                    footer_info_view = footer_info_view.push(positioned(footer_info_text, 170, 185));
                }

                _ => (),
            },

            &FooterType::SPFooter(sp_footer) => match active_footer_tab {
                _ => (),
            },
        }
    }

    footer_info_view.into()
}

pub fn track_name_text(slot_id: SlotId) -> Element<'static, Message> {
    let t = text(slot_id.to_string())
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .width(548)
        .font(RODIN_NTLG_PRO_EB)
        .size(32);
    positioned(t, 365, 154)
}

pub fn finish_time_text(finish_time: &InGameTime) -> Element<'_, Message> {
    let t = text(finish_time.to_string())
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .width(548)
        .font(RODIN_NTLG_PRO_EB)
        .size(32);
    positioned(t, 365, 205)
}

pub fn mii_name_text(mii_name: &str) -> Element<'_, Message> {
    let t = FitText {
        content: mii_name,
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
    let tooltip_text = text(format!(
        "{} ({})",
        ghost.header().combo().vehicle(),
        if ghost.header().is_automatic_drift() {
            "Automatic"
        } else {
            "Manual"
        },
    ))
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
    let mut lap_splits_text = format!("Lap 1:   {}", lap_splits[0]);

    for (idx, lap) in lap_splits[1..].iter().enumerate() {
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
        7     => (21.9, 66),
        8     => (19.1, 94),
        9     => (17.05, 114),
        10    => (15.3, 132),
        _     => (13.95, 145),
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
    let font_size = 14.0;

    let label_col = column![
        text("Mii Info")
            .font(CTMKF)
            .color(Color::WHITE)
            .size(font_size * 1.5),
        text("Creator:")
            .font(CTMKF)
            .color(Color::WHITE)
            .size(font_size),
        text("Creation date:")
            .font(CTMKF)
            .color(Color::WHITE)
            .size(font_size),
        text("Type:")
            .font(CTMKF)
            .color(Color::WHITE)
            .size(font_size),
        text("Gender:")
            .font(CTMKF)
            .color(Color::WHITE)
            .size(font_size),
        text("Birthday:")
            .font(CTMKF)
            .color(Color::WHITE)
            .size(font_size),
        /*
        text("Height:")
            .font(CTMKF)
            .color(Color::WHITE)
            .size(font_size),
        text("Weight:")
            .font(CTMKF)
            .color(Color::WHITE)
            .size(font_size),
        */
        text("Favorite color:")
            .font(CTMKF)
            .color(Color::WHITE)
            .size(font_size),
        text("Favorite Mii?")
            .font(CTMKF)
            .color(Color::WHITE)
            .size(font_size),
    ];

    let value_col = column![
        text(" ")
            .font(CTMKF)
            .color(Color::WHITE)
            .size(font_size * 1.5),
        text(if !mii.creator_name().is_empty() {
            mii.creator_name()
        } else {
            "—"
        })
        .font(CTMKF)
        .color(Color::WHITE)
        .size(font_size)
        .align_x(Alignment::End),
        text(mii.creation_date().to_string())
            .font(CTMKF)
            .color(Color::WHITE)
            .size(font_size)
            .align_x(Alignment::End),
        text(mii.mii_type().to_string())
            .font(CTMKF)
            .color(Color::WHITE)
            .size(font_size)
            .align_x(Alignment::End),
        text(if mii.is_girl() { "Female" } else { "Male" })
            .font(CTMKF)
            .color(Color::WHITE)
            .size(font_size)
            .align_x(Alignment::End),
        text(
            if let Some(month) = mii.birthday().month()
                && let Some(day) = mii.birthday().day()
            {
                format!("{:0>2}/{:0>2}", month, day)
            } else {
                String::from("Not set")
            }
        )
        .font(CTMKF)
        .color(Color::WHITE)
        .size(font_size)
        .align_x(Alignment::End),
        /*
        text(format!(
            "{} ({:.1}%)",
            mii.build().height(),
            mii.build().height() as f32 / 1.27 // 127 is max height/weight
        ))
        .font(CTMKF)
        .color(Color::WHITE)
        .size(font_size)
        .align_x(Alignment::End),
        text(format!(
            "{} ({:.1}%)",
            mii.build().weight(),
            mii.build().weight() as f32 / 1.27
        ))
        .font(CTMKF)
        .color(Color::WHITE)
        .size(font_size)
        .align_x(Alignment::End),
        */
        text(favorite_color_string(mii.favorite_color()))
            .font(CTMKF)
            .color(Color::WHITE)
            .size(font_size)
            .align_x(Alignment::End),
        text(mii.is_favorite())
            .font(CTMKF)
            .color(Color::WHITE)
            .size(font_size)
            .align_x(Alignment::End),
    ]
    .align_x(Alignment::End)
    .width(149);

    let content = row![label_col, value_col].spacing(10);

    let mii_info_element = container(content)
        .padding(10)
        .style(styles::info_box_style());

    positioned(mii_info_element, 30, 391) /* 367 with height and weight shown */
}

pub fn mii_import_button() -> Element<'static, Message> {
    positioned(
        ghost_action_button("Import Mii", true, Message::MiiImport),
        310,
        480,
    )
}

pub fn mii_export_button() -> Element<'static, Message> {
    positioned(
        ghost_action_button("Export Mii", true, Message::MiiExport),
        310,
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
    };

    let btn = button(text(label).font(RODIN_NTLG_PRO_EB).size(16).center())
        .width(263)
        .height(COMMON_BUTTON_HEIGHT)
        .on_press(Message::ToggleFooterInfoMenu)
        .style(|_, status| match status {
            button::Status::Hovered => styles::hovered_button_style(),
            _ => styles::common_button_style(),
        });

    Some(positioned(btn, 508, 446))
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
