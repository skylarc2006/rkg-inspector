use iced::{Background, Border, Color, Shadow, border::Radius, widget::button};

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

pub fn red_green_button_style(b: bool) -> button::Style {
    let background_color = if b {
        Color::from_rgba8(0, 100, 0, 0.5)
    } else {
        Color::from_rgba8(100, 0, 0, 0.5)
    };

    let border_color = if b {
        Color::from_rgba8(0, 255, 0, 1.0)
    } else {
        Color::from_rgba8(255, 0, 0, 1.0)
    };

    button::Style {
        background: Some(Background::Color(background_color)),
        text_color: Color::WHITE,
        border: Border {
            color: border_color,
            width: BORDER_WIDTH,
            radius: Radius::new(0),
        },
        shadow: Shadow::default(),
        snap: true,
    }
}

pub fn hovered_red_green_button_style(b: bool) -> button::Style {
    let background_color = if b {
        Color::from_rgba8(0, 175, 0, 0.5)
    } else {
        Color::from_rgba8(175, 0, 0, 0.5)
    };

    let border_color = if b {
        Color::from_rgba8(0, 255, 0, 1.0)
    } else {
        Color::from_rgba8(255, 0, 0, 1.0)
    };

    button::Style {
        background: Some(Background::Color(background_color)),
        text_color: Color::WHITE,
        border: Border {
            color: border_color,
            width: BORDER_WIDTH,
            radius: Radius::new(0),
        },
        shadow: Shadow::default(),
        snap: true,
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
