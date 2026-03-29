use iced::{
    Length, Rectangle,
    advanced::{
        Layout, layout, renderer, text as adv_text,
        widget::{self, Widget},
    },
};

pub struct FitText<'a> {
    pub content: &'a str,
    pub font: iced::Font,
    pub max_size: f32,
    pub min_size: f32,
    pub width: f32,
    pub height: f32,
}

impl<'a> FitText<'a> {
    pub fn fit_size<R>(&self) -> f32
    where
        R: adv_text::Renderer<Font = iced::Font>,
    {
        use adv_text::Paragraph as _;
        let mut size = self.max_size;
        while size > self.min_size {
            let paragraph = <R::Paragraph as adv_text::Paragraph>::with_text(adv_text::Text {
                content: self.content,
                bounds: iced::Size::new(self.width, f32::INFINITY),
                size: iced::Pixels(size),
                font: self.font,
                align_x: iced::Alignment::Center.into(),
                align_y: iced::alignment::Vertical::Center,
                line_height: adv_text::LineHeight::default(),
                shaping: adv_text::Shaping::Advanced,
                wrapping: adv_text::Wrapping::None,
            });
            if paragraph.min_bounds().width <= self.width {
                return size;
            }
            size -= 1.0;
        }
        self.min_size
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for FitText<'a>
where
    Renderer: adv_text::Renderer<Font = iced::Font>,
{
    fn tag(&self) -> widget::tree::Tag {
        widget::tree::Tag::of::<adv_text::paragraph::Plain<Renderer::Paragraph>>()
    }

    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(adv_text::paragraph::Plain::<Renderer::Paragraph>::default())
    }

    fn size(&self) -> iced::Size<Length> {
        iced::Size::new(Length::Fixed(self.width), Length::Fixed(self.height))
    }

    fn layout(
        &mut self,
        tree: &mut widget::Tree,
        _renderer: &Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        let state = tree
            .state
            .downcast_mut::<adv_text::paragraph::Plain<Renderer::Paragraph>>();

        let size = self.fit_size::<Renderer>();

        state.update(adv_text::Text {
            content: self.content,
            bounds: iced::Size::new(self.width, self.height),
            size: iced::Pixels(size),
            font: self.font,
            align_x: iced::Alignment::Center.into(),
            align_y: iced::alignment::Vertical::Center,
            line_height: adv_text::LineHeight::default(),
            shaping: adv_text::Shaping::Advanced,
            wrapping: adv_text::Wrapping::None,
        });

        layout::Node::new(iced::Size::new(self.width, self.height))
    }

    fn draw(
        &self,
        state: &widget::Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: iced::mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        use adv_text::Paragraph as _;
        let paragraph = state
            .state
            .downcast_ref::<adv_text::paragraph::Plain<Renderer::Paragraph>>()
            .raw();
        let bounds = layout.bounds();

        let anchor = bounds.anchor(
            paragraph.min_bounds(),
            paragraph.align_x(),
            paragraph.align_y(),
        );
        renderer.fill_paragraph(paragraph, anchor, style.text_color, bounds);
    }
}
