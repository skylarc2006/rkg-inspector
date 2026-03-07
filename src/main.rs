use iced::Font;
use iced::Size;

use crate::app::RkgInspector;

const RODIN_NTLG_PRO_EB: Font = Font {
    family: iced::font::Family::Name("FOT-RodinNTLG Pro"),
    weight: iced::font::Weight::ExtraBold,
    ..Font::DEFAULT
};

const VERSION: &str = "v0.1";

pub mod app;
pub mod files;
pub mod ui;

pub fn main() -> iced::Result {
    iced::application(
        RkgInspector::default,
        RkgInspector::update,
        RkgInspector::view,
    )
    .title(RkgInspector::title)
    .theme(RkgInspector::theme)
    .window_size(Size::new(1280.0, 720.0))
    .resizable(false)
    .font(include_bytes!("../fonts/FOT-RodinNTLG Pro EB.otf").as_slice())
    .run()
}
