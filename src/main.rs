use iced::Size;

use crate::app::RkgInspector;

pub mod app;
pub mod files;
pub mod helpers;
pub mod message;
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
    .font(include_bytes!("../fonts/ctmkf.ttf").as_slice())
    .run()
}
