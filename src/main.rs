use iced::widget::{column, button, text};
use iced::{Element, Task, Theme};
use std::path::PathBuf;

pub fn main() -> iced::Result {
    iced::application(RkgTool::default, RkgTool::update, RkgTool::view)
        .title(RkgTool::title)
        .theme(RkgTool::theme)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    LoadFile,
    FilePicked(Option<PathBuf>),
}

struct RkgTool {
    active_file: Option<PathBuf>,
}

impl RkgTool {
    pub fn new() -> Self {
        Self { active_file: None }
    }

    pub fn title(&self) -> String {
        String::from("RKG Tool")
    }

    pub fn theme(&self) -> Theme {
        Theme::Dark
    }

    pub async fn pick_file() -> Option<PathBuf> {
        rfd::FileDialog::new().pick_file()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::LoadFile => Task::perform(Self::pick_file(), Message::FilePicked),
            Message::FilePicked(path) => {
                self.active_file = path;
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let content = column![
            button("Load RKG").on_press(Message::LoadFile),
            
            text(format!(
                "Current loaded file path: {}",
                if let Some(path) = &self.active_file {
                    path.to_str().unwrap()
                } else {
                    "None"
                }
            )),
        ];

        content.into()
    }
}

impl Default for RkgTool {
    fn default() -> Self {
        Self::new()
    }
}
