use iced::{
    Background, Border, Color, Shadow,
    border::Radius,
    widget::{button, container},
};

const BORDER_WIDTH: f32 = 3.5;

pub fn common_button_style() -> button::Style {
    button::Style {
        background: Some(Background::Color(Color::from_rgba8(0, 0, 0, 0.5))),
        text_color: Color::WHITE,
        border: Border {
            color: Color::from_rgba8(255, 255, 0, 1.0),
            width: BORDER_WIDTH,
            radius: Radius::new(0),
        },
        shadow: Shadow::default(),
        snap: true,
    }
}

pub fn hovered_button_style() -> button::Style {
    button::Style {
        background: Some(Background::Color(Color::from_rgba8(100, 100, 0, 0.5))),
        ..common_button_style()
    }
}

pub fn disabled_button_style() -> button::Style {
    button::Style {
        background: Some(Background::Color(Color::from_rgba8(128, 128, 128, 0.5))),
        text_color: Color::WHITE,
        border: Border {
            color: Color::from_rgba8(128, 128, 128, 1.0),
            width: BORDER_WIDTH,
            radius: Radius::new(0),
        },
        shadow: Shadow::default(),
        snap: true,
    }
}

pub fn tooltip_style() -> impl Fn(&iced::Theme) -> container::Style {
    |_| container::Style {
        text_color: Some(Color::WHITE),
        background: Some(Background::Color(Color::from_rgba8(0, 0, 0, 0.5))),
        border: Border {
            color: Color::from_rgba8(128, 128, 128, 1.0),
            width: BORDER_WIDTH / 1.5,
            radius: Radius::new(0),
        },
        shadow: Shadow::default(),
        snap: true,
    }
}

pub fn info_box_style() -> impl Fn(&iced::Theme) -> container::Style {
    |_| container::Style {
        text_color: Some(Color::WHITE),
        background: Some(Background::Color(Color::from_rgba8(0, 0, 0, 0.5))),
        border: Border {
            color: Color::from_rgba8(255, 255, 0, 1.0),
            width: BORDER_WIDTH,
            radius: Radius::new(0),
        },
        shadow: Shadow::default(),
        snap: true,
    }
}
