use iced::{
    Alignment, Color, Element, Length,
    widget::{Button, Image, button, image, stack, svg, text},
};
use rkg_utils::header::{in_game_time::InGameTime, slot_id::SlotId};

use crate::{
    app::Message,
    ui::{empty_width, empty_height, positioned, styles, constants::{CTMKF, RODIN_NTLG_PRO_EB, VERSION}},
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
    let btn = button(text("Select Ghost").font(RODIN_NTLG_PRO_EB).size(16).center())
        .width(COMMON_BUTTON_WIDTH)
        .height(COMMON_BUTTON_HEIGHT)
        .on_press(Message::LoadFile)
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
        ghost_action_button("Save As...", ghost_is_loaded, Message::SaveAsFile),
        807,
        80,
    )
}

pub fn track_name_text(slot_id: &SlotId) -> Element<'_, Message> {
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
    let t = text(mii_name)
        .align_x(Alignment::Center)
        .align_y(Alignment::Start)
        .width(548)
        .font(CTMKF)
        .size(26);
    positioned(t, 282, 255)
}

pub fn country_image(handle: svg::Handle) -> Element<'static, Message> {
    let country_image = svg(handle).height(32);

    let country_image = iced::widget::row![country_image, empty_width(167)]
        .width(Length::Fill)
        .align_y(Alignment::Center)
        .spacing(0);

    let country_image = iced::widget::column![empty_height(300), country_image]
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .spacing(0);

    country_image.into()
}

pub fn character_image(handle: image::Handle) -> Element<'static, Message> {
    positioned(image(handle).scale(0.6), 652, 230)
}

pub fn vehicle_image(handle: image::Handle) -> Element<'static, Message> {
    positioned(image(handle).scale(0.76), 744, 245)
}
