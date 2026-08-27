use iced::Font;

pub const RODIN_NTLG_PRO_EB: Font = Font {
    family: iced::font::Family::Name("FOT-RodinNTLG Pro"),
    weight: iced::font::Weight::ExtraBold,
    ..Font::DEFAULT
};

pub const CTMKF: Font = Font {
    family: iced::font::Family::Name("CTMKF"),
    weight: iced::font::Weight::Bold,
    ..Font::DEFAULT
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

pub const VERSION: Version = Version {
    major: 1,
    minor: 1,
    patch: 1,
};


pub const DESIGN_WIDTH: f32 = 1280.0;
pub const DESIGN_HEIGHT: f32 = 720.0;
pub const ASPECT_RATIO: f32 = DESIGN_WIDTH / DESIGN_HEIGHT;

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "v{}.{}.{}", self.major, self.minor, self.patch)
    }
}
