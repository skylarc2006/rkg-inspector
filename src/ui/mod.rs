use iced::{Element, Length, widget::{Space, column, row}};

pub mod assets;
pub mod constants;
pub mod image_handles;
pub mod styles;
pub mod widgets;

pub fn empty_height(height: u32) -> Space {
    Space::new().height(height)
}

pub fn empty_width(width: u32) -> Space {
    Space::new().width(width)
}

pub fn positioned<'a, M: 'a>(widget: impl Into<Element<'a, M>>, x: u32, y: u32) -> Element<'a, M> {
    let widget: Element<'a, M> = widget.into();
    column![
        empty_height(y),
        row![empty_width(x), widget].width(Length::Fill).spacing(0),
    ]
    .width(Length::Fill)
    .spacing(0)
    .into()
}
