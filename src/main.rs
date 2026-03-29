#![windows_subsystem = "windows"]

use iced::{Size, Task, window};

use crate::app::RkgInspector;
use crate::message::Message;

pub mod app;
pub mod files;
pub mod helpers;
pub mod message;
pub mod mii_rendering;
pub mod ui;

pub fn main() -> iced::Result {
    let initial_path: Option<std::path::PathBuf> = std::env::args().nth(1).map(Into::into);

    iced::application(
        move || {
            let task = initial_path
                .clone()
                .map(|p| Task::done(Message::GhostPicked(Some(p))))
                .unwrap_or(Task::none());
            (RkgInspector::new(), task)
        },
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
    .subscription(RkgInspector::subscription)
    .font(include_bytes!("../fonts/FOT-RodinNTLG Pro EB.otf").as_slice())
    .font(include_bytes!("../fonts/ctmkf.ttf").as_slice())
    .run()
}
