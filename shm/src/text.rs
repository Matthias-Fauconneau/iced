use iced_native::{Color, Font, HorizontalAlignment, VerticalAlignment};
type Rectangle = iced_native::Rectangle<u32>;
pub struct Section<'t> {
    pub content: &'t str,
    pub bounds: Rectangle,
    pub color: Color,
    pub size: f32,
    pub font: Font,
    pub horizontal_alignment: HorizontalAlignment,
    pub vertical_alignment: VerticalAlignment,
}

#[cfg(feature = "fontkit")]
mod font;

use crate::Transformation;

pub const BUILTIN_ICONS: iced_native::Font = iced_native::Font::External {
    name: "iced_wgpu icons",
    bytes: include_bytes!("text/icons.ttf"),
};

pub const CHECKMARK_ICON: char = '\u{F00C}';

//const FALLBACK_FONT: &[u8] = include_bytes!("../fonts/Lato-Regular.ttf");

#[derive(Debug)]
pub struct Pipeline {}

impl Pipeline {
    pub fn new(_default_font: Option<&[u8]>) -> Self {
        /*let font_source = font::Source::new();

        let default_font =
            default_font.map(|slice| slice.to_vec()).unwrap_or_else(|| {
                font_source
                    .load(&[font::Family::SansSerif, font::Family::Serif])
                    .unwrap_or_else(|_| FALLBACK_FONT.to_vec())
            });*/

        Pipeline {}
    }

    pub fn queue(&mut self, section: Section) {
        let Section {
            content,
            bounds,
            color,
            size,
            font,
            horizontal_alignment,
            vertical_alignment,
        } = section;
        let _ = (
            content,
            bounds,
            color,
            size,
            font,
            horizontal_alignment,
            vertical_alignment,
        );
    }

    pub fn draw_queued(
        &mut self,
        _target: &(),
        _transformation: Transformation,
        _bounds: Rectangle,
    ) {
    }

    pub fn measure(
        &self,
        text: &str,
        _size: f32,
        _font: iced_native::Font,
        bounds: iced_native::Size,
    ) -> (f32, f32) {
        use framework::{
            text::{
                Attribute, Color, FontStyle, Style, Text, TextRange, TextSize,
            },
            vector::vec2,
            widget::Widget,
        };
        let style = vec![Attribute::<Style> {
            range: TextRange::up_to(TextSize::of(text)),
            attribute: Style {
                color: Color {
                    b: 1.,
                    r: 1.,
                    g: 1.,
                },
                style: FontStyle::Normal,
            },
        }];
        #[allow(non_camel_case_types)]
        type size2f = vec2;
        (<Text as Widget>::size(
            &mut Text::new(text, &style),
            ((bounds.into(): [f32; 2]).into(): size2f).into(),
        )
        .into(): size2f)
            .into()
    }

    pub fn space_width(&self, size: f32) -> f32 {
        let font = framework::text::default_font.suffix();
        let space_index = font.glyph_index(' ').unwrap();
        let space_advance = font.glyph_hor_advance(space_index).unwrap();
        size * space_advance as f32
    }
}
