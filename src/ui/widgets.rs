use iced::{
    Alignment, Color, Element, Length,
    widget::{Button, Image, button, column, image, row, stack, text},
};
use rkg_utils::header::{in_game_time::InGameTime, slot_id::SlotId};

use crate::{CTMKF, RODIN_NTLG_PRO_EB, VERSION, app::Message, ui::*};

const COMMON_BUTTON_WIDTH: u32 = 140;
const COMMON_BUTTON_HEIGHT: u32 = 40;

pub fn background(
    background_handle: image::Handle,
    ghost_box_handle: image::Handle,
) -> Element<'static, Message> {
    let background_image = image(background_handle).scale(1.0);
    let ghost_box: Image = image(ghost_box_handle).scale(0.5);

    stack!(background_image, ghost_box).into()
}

pub fn rkg_inspector_text() -> Element<'static, Message> {
    let rkg_inspector_text = text(format!("RKG Inspector {}", VERSION))
        .color(Color::from_rgba8(128, 128, 128, 1.0))
        .align_x(Alignment::Start)
        .align_y(Alignment::Start)
        .width(600)
        .font(RODIN_NTLG_PRO_EB)
        .size(28);

    // Set positioning
    let rkg_inspector_text = row![empty_width(20), rkg_inspector_text,]
        .width(Length::Fill)
        .align_y(Alignment::Center)
        .spacing(0);

    let rkg_inspector_text = column![empty_height(43), rkg_inspector_text,]
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .spacing(0);

    rkg_inspector_text.into()
}

pub fn select_ghost_button() -> Element<'static, Message> {
    let select_ghost_text = text("Select Ghost")
        .font(RODIN_NTLG_PRO_EB)
        .size(16)
        .center();

    let select_ghost_button: Button<Message> = button(select_ghost_text)
        .width(COMMON_BUTTON_WIDTH)
        .height(COMMON_BUTTON_HEIGHT)
        .on_press(Message::LoadFile)
        .style(move |_theme, status| match status {
            button::Status::Hovered => styles::hovered_button_style(),
            _ => styles::common_button_style(),
        });

    // Set positioning
    let select_ghost_button = row![empty_width(507), select_ghost_button,]
        .width(Length::Fill)
        .align_y(Alignment::Start)
        .spacing(0);

    let select_ghost_button = column![empty_height(80), select_ghost_button,]
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .spacing(0);

    select_ghost_button.into()
}

pub fn toggle_edit_button(ghost_is_loaded: bool) -> Element<'static, Message> {
    let edit_text = "Edit Ghost";

    let toggle_edit_text = text(edit_text).font(RODIN_NTLG_PRO_EB).size(16).center();

    let toggle_edit_button: Button<Message> = if ghost_is_loaded {
        button(toggle_edit_text)
            .width(COMMON_BUTTON_WIDTH)
            .height(COMMON_BUTTON_HEIGHT)
            .on_press(Message::ToggleEditMenu)
            .style(move |_theme, status| match status {
                button::Status::Hovered => styles::hovered_button_style(),
                _ => styles::common_button_style(),
            })
    } else {
        button(toggle_edit_text)
            .width(COMMON_BUTTON_WIDTH)
            .height(COMMON_BUTTON_HEIGHT)
            .style(move |_theme, status| match status {
                _ => styles::disabled_button_style(),
            })
    };

    // Set positioning
    let toggle_edit_button = row![empty_width(657), toggle_edit_button,]
        .width(Length::Fill)
        .align_y(Alignment::Start)
        .spacing(0);

    let toggle_edit_button = column![empty_height(80), toggle_edit_button,]
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .spacing(0);

    toggle_edit_button.into()
}

pub fn save_as_button(ghost_is_loaded: bool) -> Element<'static, Message> {
    let save_as_text = text("Save As...").font(RODIN_NTLG_PRO_EB).size(16).center();

    let save_as_button: Button<Message> = if ghost_is_loaded {
        button(save_as_text)
            .width(COMMON_BUTTON_WIDTH)
            .height(COMMON_BUTTON_HEIGHT)
            .on_press(Message::SaveAsFile)
            .style(move |_theme, status| match status {
                button::Status::Hovered => styles::hovered_button_style(),
                _ => styles::common_button_style(),
            })
    } else {
        button(save_as_text)
            .width(COMMON_BUTTON_WIDTH)
            .height(COMMON_BUTTON_HEIGHT)
            .style(move |_theme, status| match status {
                _ => styles::disabled_button_style(),
            })
    };

    // Set positioning
    let save_as_button = row![empty_width(807), save_as_button,]
        .width(Length::Fill)
        .align_y(Alignment::Start)
        .spacing(0);

    let save_as_button = column![empty_height(80), save_as_button,]
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .spacing(0);

    save_as_button.into()
}

pub fn track_name_text(slot_id: &SlotId) -> Element<'_, Message> {
    let track_name_text = text(slot_id.to_string())
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .width(548)
        .font(RODIN_NTLG_PRO_EB)
        .size(32);

    // Set positioning
    let track_name_text = row![empty_width(365), track_name_text,]
        .width(Length::Fill)
        .align_y(Alignment::Center)
        .spacing(0);

    let track_name_text = column![empty_height(154), track_name_text,]
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .spacing(0);

    track_name_text.into()
}

pub fn finish_time_text(finish_time: &InGameTime) -> Element<'_, Message> {
    let finish_time_text = text(finish_time.to_string())
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .width(548)
        .font(RODIN_NTLG_PRO_EB)
        .size(32);

    // Set positioning
    let finish_time_text = row![empty_width(365), finish_time_text,]
        .width(Length::Fill)
        .align_y(Alignment::Center)
        .spacing(0);

    let finish_time_text = column![empty_height(205), finish_time_text,]
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .spacing(0);

    finish_time_text.into()
}

pub fn mii_name_text(mii_name: &str) -> Element<'_, Message> {
    let mii_name_text = text(mii_name)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .width(548)
        .font(CTMKF)
        .size(26);

    // Set positioning
    let mii_name_text = row![empty_width(282), mii_name_text,]
        .width(Length::Fill)
        .align_y(Alignment::Start)
        .spacing(0);

    let mii_name_text = column![empty_height(255), mii_name_text,]
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .spacing(0);

    mii_name_text.into()
}

pub fn country_image(country_image_handle: &image::Handle) -> Element<'_, Message> {
    let country_image = image(country_image_handle)
    .scale(0.77);

    // Set positioning
    let country_image = row![empty_width(524), country_image,]
        .width(Length::Fill)
        .align_y(Alignment::Center)
        .spacing(0);

    let country_image = column![empty_height(284), country_image,]
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .spacing(0);

    country_image.into()
}
