use iced::border::Radius;
use iced::{Alignment, Background, Border, Color, Font, Shadow};
use iced::widget::{Button, Image, Space, button, column, row, text};
use iced::{
    Element, Length, Size, Task, Theme,
    widget::{image, stack},
};
use std::path::PathBuf;


const RODIN_NTLG_PRO_EB: Font = Font {
    family: iced::font::Family::Name("FOT-RodinNTLG Pro"),
    weight: iced::font::Weight::ExtraBold,
    ..Font::DEFAULT
};

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
    .font(include_bytes!("C:/repos/rkg-inspector/src/fonts/FOT-RodinNTLG Pro EB.otf").as_slice())
    .run()
}

#[derive(Debug, Clone)]
enum Message {
    LoadFile,
    FilePicked(Option<PathBuf>),
}

struct RkgInspector {
    active_file: Option<PathBuf>,
}

impl RkgInspector {
    pub fn new() -> Self {
        Self { 
            active_file: None,
        }
    }

    pub fn title(&self) -> String {
        String::from("RKG Inspector")
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
        let common_button_style: button::Style = button::Style {
            background: Some(Background::Color(Color::from_rgba8(0, 0, 0, 0.5))),
            text_color: Color::WHITE,
            border: Border {
                color: Color::from_rgba8(255, 255, 0, 1.0),
                width: 3.5,
                radius: Radius::new(0),
            },
            shadow: Shadow::default(),
            snap: true,
        };


        let background_image = image("C:/repos/rkg-inspector/src/images/background.png").scale(1.0);
        let ghost_box: Image = image("C:/repos/rkg-inspector/src/images/ghost_box.png").scale(0.5);

        let background = stack!(
            background_image,
            ghost_box
        );

        let select_ghost_text = text("Select Ghost")
            .font(RODIN_NTLG_PRO_EB)
            .size(16)
            .center();
        
        let select_ghost_button: Button<Message> = button(select_ghost_text)
            .width(150)
            .height(40)
            .on_press(Message::LoadFile)
            .style(move |_theme, status| match status {
                button::Status::Hovered => button::Style {
                    background: Some(Background::Color(Color::from_rgba8(100, 100, 0, 0.5))),
                    ..common_button_style
                },
                _ => common_button_style,
            });

        let select_ghost_button = row![
            empty_width(490),
            select_ghost_button,
        ]
        .width(Length::Fill)
        .align_y(Alignment::Start)
        .spacing(0);

        let select_ghost_button = column![
            empty_height(80),
            select_ghost_button,
        ]
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .spacing(0);

        stack!(background, select_ghost_button)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl Default for RkgInspector {
    fn default() -> Self {
        Self::new()
    }
}

fn empty_height(height: u32) -> Space {
    Space::new().height(height)
}

fn empty_width(width: u32) -> Space {
    Space::new().width(width)
}
