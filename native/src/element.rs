use crate::{
    layout, renderer, Color, Event, Hasher, Layout, Length, Point, Widget,
};

/// A generic [`Widget`].
///
/// It is useful to build composable user interfaces that do not leak
/// implementation details in their __view logic__.
///
/// If you have a [built-in widget], you should be able to use `Into<Element>`
/// to turn it into an [`Element`].
///
/// [built-in widget]: widget/index.html#built-in-widgets
/// [`Widget`]: widget/trait.Widget.html
/// [`Element`]: struct.Element.html
pub struct Element<'a, Message, Renderer> {
    pub(crate) widget: Box<dyn Widget<Message, Renderer> + 'a>,
}

impl<'a, Message, Renderer> std::fmt::Debug for Element<'a, Message, Renderer> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Element")
            .field("widget", &self.widget)
            .finish()
    }
}

impl<'a, Message, Renderer> Element<'a, Message, Renderer>
where
    Renderer: crate::Renderer,
{
    /// Create a new [`Element`] containing the given [`Widget`].
    ///
    /// [`Element`]: struct.Element.html
    /// [`Widget`]: widget/trait.Widget.html
    pub fn new(
        widget: impl Widget<Message, Renderer> + 'a,
    ) -> Element<'a, Message, Renderer> {
        Element {
            widget: Box::new(widget),
        }
    }

    pub fn width(&self) -> Length {
        self.widget.width()
    }

    pub fn height(&self) -> Length {
        self.widget.height()
    }

    pub fn layout(
        &self,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.widget.layout(renderer, limits)
    }

    pub fn draw(
        &self,
        renderer: &mut Renderer,
        layout: Layout<'_>,
        cursor_position: Point,
    ) -> Renderer::Output {
        self.widget.draw(renderer, layout, cursor_position)
    }

    /// Applies a transformation to the produced message of the [`Element`].
    ///
    /// This method is useful when you want to decouple different parts of your
    /// UI and make them __composable__.
    ///
    /// [`Element`]: struct.Element.html
    ///
    /// # Example
    /// Imagine we want to use [our counter](index.html#usage). But instead of
    /// showing a single counter, we want to display many of them. We can reuse
    /// the `Counter` type as it is!
    ///
    /// We use composition to model the __state__ of our new application:
    ///
    /// ```
    /// # mod counter {
    /// #     pub struct Counter;
    /// # }
    /// use counter::Counter;
    ///
    /// struct ManyCounters {
    ///     counters: Vec<Counter>,
    /// }
    /// ```
    ///
    /// We can store the state of multiple counters now. However, the
    /// __messages__ we implemented before describe the user interactions
    /// of a __single__ counter. Right now, we need to also identify which
    /// counter is receiving user interactions. Can we use composition again?
    /// Yes.
    ///
    /// ```
    /// # mod counter {
    /// #     #[derive(Debug, Clone, Copy)]
    /// #     pub enum Message {}
    /// # }
    /// #[derive(Debug, Clone, Copy)]
    /// pub enum Message {
    ///     Counter(usize, counter::Message)
    /// }
    /// ```
    ///
    /// We compose the previous __messages__ with the index of the counter
    /// producing them. Let's implement our __view logic__ now:
    ///
    /// ```
    /// # mod counter {
    /// #     use iced_native::{text, Text};
    /// #
    /// #     #[derive(Debug, Clone, Copy)]
    /// #     pub enum Message {}
    /// #     pub struct Counter;
    /// #
    /// #     impl Counter {
    /// #         pub fn view(&mut self) -> Text {
    /// #             Text::new("")
    /// #         }
    /// #     }
    /// # }
    /// #
    /// # mod iced_wgpu {
    /// #     use iced_native::{
    /// #         text, row, layout, Text, Size, Point, Rectangle, Layout, Row
    /// #     };
    /// #     pub struct Renderer;
    /// #
    /// #     impl iced_native::Renderer for Renderer { type Output = (); }
    /// #
    /// #     impl iced_native::row::Renderer for Renderer {
    /// #         fn draw<Message>(
    /// #             &mut self,
    /// #             _column: &Row<'_, Message, Self>,
    /// #             _layout: Layout<'_>,
    /// #             _cursor_position: Point,
    /// #         ) {}
    /// #     }
    /// #
    /// #     impl text::Renderer for Renderer {
    /// #         fn layout(
    /// #             &self,
    /// #             _text: &Text,
    /// #             _limits: &layout::Limits,
    /// #         ) -> layout::Node {
    /// #             layout::Node::new(Size::ZERO)
    /// #         }
    /// #
    /// #         fn draw(
    /// #             &mut self,
    /// #             _text: &Text,
    /// #             _layout: Layout<'_>,
    /// #         ) {}
    /// #     }
    /// # }
    /// #
    /// # use counter::Counter;
    /// #
    /// # struct ManyCounters {
    /// #     counters: Vec<Counter>,
    /// # }
    /// #
    /// # #[derive(Debug, Clone, Copy)]
    /// # pub enum Message {
    /// #    Counter(usize, counter::Message)
    /// # }
    /// use iced_native::{Element, Row};
    /// use iced_wgpu::Renderer;
    ///
    /// impl ManyCounters {
    ///     pub fn view(&mut self) -> Row<Message, Renderer> {
    ///         // We can quickly populate a `Row` by folding over our counters
    ///         self.counters.iter_mut().enumerate().fold(
    ///             Row::new().spacing(20),
    ///             |row, (index, counter)| {
    ///                 // We display the counter
    ///                 let element: Element<counter::Message, Renderer> =
    ///                     counter.view().into();
    ///
    ///                 row.push(
    ///                     // Here we turn our `Element<counter::Message>` into
    ///                     // an `Element<Message>` by combining the `index` and the
    ///                     // message of the `element`.
    ///                     element.map(move |message| Message::Counter(index, message))
    ///                 )
    ///             }
    ///         )
    ///     }
    /// }
    /// ```
    ///
    /// Finally, our __update logic__ is pretty straightforward: simple
    /// delegation.
    ///
    /// ```
    /// # mod counter {
    /// #     #[derive(Debug, Clone, Copy)]
    /// #     pub enum Message {}
    /// #     pub struct Counter;
    /// #
    /// #     impl Counter {
    /// #         pub fn update(&mut self, _message: Message) {}
    /// #     }
    /// # }
    /// #
    /// # use counter::Counter;
    /// #
    /// # struct ManyCounters {
    /// #     counters: Vec<Counter>,
    /// # }
    /// #
    /// # #[derive(Debug, Clone, Copy)]
    /// # pub enum Message {
    /// #    Counter(usize, counter::Message)
    /// # }
    /// impl ManyCounters {
    ///     pub fn update(&mut self, message: Message) {
    ///         match message {
    ///             Message::Counter(index, counter_msg) => {
    ///                 if let Some(counter) = self.counters.get_mut(index) {
    ///                     counter.update(counter_msg);
    ///                 }
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    pub fn map<F, B>(self, f: F) -> Element<'a, B, Renderer>
    where
        Message: 'static + Clone,
        Renderer: 'a,
        B: 'static,
        F: 'static + Fn(Message) -> B,
    {
        Element {
            widget: Box::new(Map::new(self.widget, f)),
        }
    }

    /// Marks the [`Element`] as _to-be-explained_.
    ///
    /// The [`Renderer`] will explain the layout of the [`Element`] graphically.
    /// This can be very useful for debugging your layout!
    ///
    /// [`Element`]: struct.Element.html
    /// [`Renderer`]: trait.Renderer.html
    pub fn explain<C: Into<Color>>(
        self,
        color: C,
    ) -> Element<'a, Message, Renderer>
    where
        Message: 'static,
        Renderer: 'a + renderer::Debugger,
    {
        Element {
            widget: Box::new(Explain::new(self, color.into())),
        }
    }

    pub(crate) fn hash_layout(&self, state: &mut Hasher) {
        self.widget.hash_layout(state);
    }
}

struct Map<'a, A, B, Renderer> {
    widget: Box<dyn Widget<A, Renderer> + 'a>,
    mapper: Box<dyn Fn(A) -> B>,
}

impl<'a, A, B, Renderer> std::fmt::Debug for Map<'a, A, B, Renderer> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Map").field("widget", &self.widget).finish()
    }
}

impl<'a, A, B, Renderer> Map<'a, A, B, Renderer> {
    pub fn new<F>(
        widget: Box<dyn Widget<A, Renderer> + 'a>,
        mapper: F,
    ) -> Map<'a, A, B, Renderer>
    where
        F: 'static + Fn(A) -> B,
    {
        Map {
            widget,
            mapper: Box::new(mapper),
        }
    }
}

impl<'a, A, B, Renderer> Widget<B, Renderer> for Map<'a, A, B, Renderer>
where
    A: Clone,
    Renderer: crate::Renderer,
{
    fn layout(
        &self,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.widget.layout(renderer, limits)
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        messages: &mut Vec<B>,
        renderer: &Renderer,
    ) {
        let mut original_messages = Vec::new();

        self.widget.on_event(
            event,
            layout,
            cursor_position,
            &mut original_messages,
            renderer,
        );

        original_messages
            .iter()
            .cloned()
            .for_each(|message| messages.push((self.mapper)(message)));
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        layout: Layout<'_>,
        cursor_position: Point,
    ) -> Renderer::Output {
        self.widget.draw(renderer, layout, cursor_position)
    }

    fn hash_layout(&self, state: &mut Hasher) {
        self.widget.hash_layout(state);
    }
}

struct Explain<'a, Message, Renderer: crate::Renderer> {
    element: Element<'a, Message, Renderer>,
    color: Color,
}

impl<'a, Message, Renderer> std::fmt::Debug for Explain<'a, Message, Renderer>
where
    Renderer: crate::Renderer,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Explain")
            .field("element", &self.element)
            .finish()
    }
}

impl<'a, Message, Renderer> Explain<'a, Message, Renderer>
where
    Renderer: crate::Renderer,
{
    fn new(element: Element<'a, Message, Renderer>, color: Color) -> Self {
        Explain { element, color }
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer>
    for Explain<'a, Message, Renderer>
where
    Renderer: crate::Renderer + renderer::Debugger,
{
    fn layout(
        &self,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.element.widget.layout(renderer, limits)
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        messages: &mut Vec<Message>,
        renderer: &Renderer,
    ) {
        self.element.widget.on_event(
            event,
            layout,
            cursor_position,
            messages,
            renderer,
        )
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        layout: Layout<'_>,
        cursor_position: Point,
    ) -> Renderer::Output {
        renderer.explain(
            self.element.widget.as_ref(),
            layout,
            cursor_position,
            self.color,
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        self.element.widget.hash_layout(state);
    }
}
