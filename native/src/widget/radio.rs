//! Create choices using radio buttons.
use crate::input::{mouse, ButtonState};
use crate::{layout, Element, Event, Hasher, Layout, Length, Point, Widget};

use std::hash::Hash;

pub use iced_core::Radio;

impl<Message, Renderer> Widget<Message, Renderer> for Radio<Message>
where
    Renderer: self::Renderer,
    Message: Clone + std::fmt::Debug,
{
    fn width(&self) -> Length {
        Length::Fill
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        renderer.layout(&self, limits)
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        messages: &mut Vec<Message>,
        _renderer: &Renderer,
    ) {
        match event {
            Event::Mouse(mouse::Event::Input {
                button: mouse::Button::Left,
                state: ButtonState::Pressed,
            }) => {
                if layout.bounds().contains(cursor_position) {
                    messages.push(self.on_click.clone());
                }
            }
            _ => {}
        }
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        layout: Layout<'_>,
        cursor_position: Point,
    ) -> Renderer::Output {
        renderer.draw(&self, layout, cursor_position)
    }

    fn hash_layout(&self, state: &mut Hasher) {
        self.label.hash(state);
    }
}

/// The renderer of a [`Radio`] button.
///
/// Your [renderer] will need to implement this trait before being
/// able to use a [`Radio`] button in your user interface.
///
/// [`Radio`]: struct.Radio.html
/// [renderer]: ../../renderer/index.html
pub trait Renderer: crate::Renderer {
    /// Creates a [`Node`] for the provided [`Radio`].
    ///
    /// [`Node`]: ../../struct.Node.html
    /// [`Radio`]: struct.Radio.html
    fn layout<Message>(
        &self,
        radio: &Radio<Message>,
        limits: &layout::Limits,
    ) -> layout::Node;

    /// Draws a [`Radio`] button.
    ///
    /// It receives:
    ///   * the current cursor position
    ///   * the bounds of the [`Radio`]
    ///   * the bounds of the label of the [`Radio`]
    ///   * whether the [`Radio`] is selected or not
    ///
    /// [`Radio`]: struct.Radio.html
    fn draw<Message>(
        &mut self,
        radio: &Radio<Message>,
        layout: Layout<'_>,
        cursor_position: Point,
    ) -> Self::Output;
}

impl<'a, Message, Renderer> From<Radio<Message>>
    for Element<'a, Message, Renderer>
where
    Renderer: self::Renderer,
    Message: 'static + Clone + std::fmt::Debug,
{
    fn from(checkbox: Radio<Message>) -> Element<'a, Message, Renderer> {
        Element::new(checkbox)
    }
}
