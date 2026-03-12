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

pub const VERSION: &str = "pre-release"; // "v0.1"
