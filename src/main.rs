use iced::{Size, window};

use crate::app::RkgInspector;

pub mod app;
pub mod files;
pub mod helpers;
pub mod message;
pub mod mii_rendering;
pub mod ui;

pub fn main() -> iced::Result {
    iced::application(
        RkgInspector::default,
        RkgInspector::update,
        RkgInspector::view,
    )
    .title(RkgInspector::title)
    .theme(RkgInspector::theme)
    .window(window::Settings {
        icon: Some(
            window::icon::from_file_data(include_bytes!("../images/icon.ico"), None).unwrap(),
        ),
        ..Default::default()
    })
    .window_size(Size::new(1280.0, 720.0))
    .resizable(false)
    .font(include_bytes!("../fonts/FOT-RodinNTLG Pro EB.otf").as_slice())
    .font(include_bytes!("../fonts/ctmkf.ttf").as_slice())
    .run()
}
