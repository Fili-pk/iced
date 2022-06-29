//! Write some text for your users to read.
use crate::widget::Tree;
use crate::{Element, Widget};

use iced_native::layout::{self, Layout};
use iced_native::renderer;
use iced_native::text;
use iced_native::widget;
use iced_native::{Length, Point, Rectangle};

pub use iced_native::widget::text::{Appearance, StyleSheet, Text};

impl<Message, Renderer> Widget<Message, Renderer> for Text<Renderer>
where
    Renderer: text::Renderer,
    Renderer::Theme: widget::text::StyleSheet,
{
    fn width(&self) -> Length {
        <Self as iced_native::Widget<Message, Renderer>>::width(self)
    }

    fn height(&self) -> Length {
        <Self as iced_native::Widget<Message, Renderer>>::height(self)
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        <Self as iced_native::Widget<Message, Renderer>>::layout(
            self, renderer, limits,
        )
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) {
        <Self as iced_native::Widget<Message, Renderer>>::draw(
            self,
            renderer,
            theme,
            style,
            layout,
            cursor_position,
            viewport,
        )
    }
}

impl<'a, Message, Renderer> Into<Element<'a, Message, Renderer>>
    for Text<Renderer>
where
    Renderer: text::Renderer + 'a,
    Renderer::Theme: widget::text::StyleSheet,
{
    fn into(self) -> Element<'a, Message, Renderer> {
        Element::new(self)
    }
}

impl<'a, Message, Renderer> Into<Element<'a, Message, Renderer>> for &'a str
where
    Renderer: text::Renderer + 'a,
    Renderer::Theme: widget::text::StyleSheet,
{
    fn into(self) -> Element<'a, Message, Renderer> {
        Text::new(self).into()
    }
}
