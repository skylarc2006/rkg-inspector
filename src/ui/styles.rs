use iced::{
    Background, Border, Color, Shadow,
    border::Radius,
    widget::{button, container},
};

const BORDER_WIDTH: f32 = 3.5;

pub fn grey_text() -> Color {
    Color::from_rgba8(128, 128, 128, 1.0)
}

pub fn alarm_color() -> Color {
    Color::from_rgba8(255, 0, 0, 1.0)
}

pub fn notice_color() -> Color {
    Color::from_rgba8(128, 0, 128, 1.0)
}

/// The button style/hover-style pair used by every plain (non-toggle) button.
pub fn common_button_theme() -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    |_, status| match status {
        button::Status::Hovered => hovered_button_style(),
        _ => common_button_style(),
    }
}

pub fn common_button_style() -> button::Style {
    button::Style {
        background: Some(Background::Color(Color::from_rgba8(0, 0, 0, 0.5))),
        text_color: Color::from_rgba8(255, 255, 255, 0.8),
        border: Border {
            color: Color::from_rgba8(255, 255, 0, 0.8),
            width: BORDER_WIDTH / 1.35,
            radius: Radius::new(0),
        },
        shadow: Shadow::default(),
        snap: true,
    }
}

pub fn hovered_button_style() -> button::Style {
    button::Style {
        background: Some(Background::Color(Color::from_rgba8(100, 100, 0, 0.5))),
        text_color: Color::WHITE,
        border: Border {
            color: Color::from_rgba8(255, 255, 0, 1.0),
            width: BORDER_WIDTH / 1.15,
            radius: Radius::new(0),
        },
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
        Color::from_rgba8(0, 255, 0, 0.8)
    } else {
        Color::from_rgba8(255, 0, 0, 0.8)
    };

    button::Style {
        background: Some(Background::Color(background_color)),
        text_color: Color::from_rgba8(255, 255, 255, 0.8),
        border: Border {
            color: border_color,
            width: BORDER_WIDTH / 1.35,
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
            width: BORDER_WIDTH / 1.35,
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
            width: BORDER_WIDTH / 1.1,
            radius: Radius::new(0),
        },
        shadow: Shadow::default(),
        snap: true,
    }
}

/// Border width used by [`capsule_button_style`], exposed so
/// [`black_backing_style`] can size its own border to reach under it.
pub const FACE_BORDER_WIDTH: f32 = 4.5;

pub fn capsule_button_style(active: bool, radius: f32) -> button::Style {
    let (background_color, text_color) = if active {
        (Color::WHITE, Color::BLACK)
    } else {
        (Color::TRANSPARENT, Color::WHITE)
    };

    button::Style {
        background: Some(Background::Color(background_color)),
        text_color,
        border: Border {
            color: Color::WHITE,
            width: FACE_BORDER_WIDTH,
            radius: Radius::new(radius),
        },
        shadow: Shadow::default(),
        snap: true,
    }
}

/// A transparent shape with a thick black border, drawn slightly larger
/// than and behind a [`capsule_button_style`] shape. The border is wide
/// enough to reach under that shape's own white border, so only a thin
/// black ring peeks out around it rather than filling the whole shape.
pub fn black_backing_style(radius: f32, pad: f32) -> button::Style {
    button::Style {
        background: Some(Background::Color(Color::TRANSPARENT)),
        text_color: Color::TRANSPARENT,
        border: Border {
            color: Color::BLACK,
            width: pad + FACE_BORDER_WIDTH,
            radius: Radius::new(radius),
        },
        shadow: Shadow::default(),
        snap: true,
    }
}

/// A thin black ring drawn just inside a [`capsule_button_style`] shape's
/// own white border (its transparent interior reveals whatever is
/// underneath). Sits *above* that shape in the stack, so this inner edge
/// stays visible even when the shape is filled solid white for its active
/// state, rather than disappearing under the fill.
pub fn inner_ring_style(radius: f32) -> button::Style {
    button::Style {
        background: Some(Background::Color(Color::TRANSPARENT)),
        text_color: Color::TRANSPARENT,
        border: Border {
            color: Color::BLACK,
            width: INNER_RING_WIDTH,
            radius: Radius::new(radius),
        },
        shadow: Shadow::default(),
        snap: true,
    }
}

/// Thickness of [`inner_ring_style`]'s black ring.
pub const INNER_RING_WIDTH: f32 = 3.0;
