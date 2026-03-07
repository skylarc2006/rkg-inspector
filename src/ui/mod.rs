use iced::widget::Space;

pub mod assets;
pub mod styles;
pub mod widgets;
pub mod image_handles;

pub fn empty_height(height: u32) -> Space {
    Space::new().height(height)
}

pub fn empty_width(width: u32) -> Space {
    Space::new().width(width)
}
