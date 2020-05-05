use smithay_client_toolkit::{reexports::client::protocol::{wl_pointer::{self, ButtonState}, wl_surface::WlSurface}, seat::pointer::ThemedPointer};
use {iced_native::{Event::Mouse, mouse}, crate::conversion};

// Track focus and reconstruct scroll events
#[derive(Default)] pub struct Pointer {
    focus : Option<WlSurface>,
    axis_buffer: Option<(f32, f32)>,
    axis_discrete_buffer: Option<(i32, i32)>,
}

impl Pointer {
    pub fn handle(&mut self, event : wl_pointer::Event, pointer: ThemedPointer, events: &mut Vec<iced_native::Event>, /*surface: &WlSurface,*/ cursor: &'static str) {
        let Self{focus, axis_buffer, axis_discrete_buffer} = self;
        use wl_pointer::Event::*;
        match event {
            Enter { surface, surface_x:x,surface_y:y, .. } /*if surface == *window.surface()*/ => {
                *focus = Some(surface);
                //log::trace!("cursor: {}", cursor);
                let _ = (&pointer, cursor); //pointer.set_cursor(cursor, None).expect("Unknown cursor"); // wayland-cursor-0.26.4/src/lib.rs:124
                events.push(Mouse(mouse::Event::CursorEntered));
                events.push(Mouse(mouse::Event::CursorMoved{x: x as f32, y: y as f32}));
            }
            Leave { surface, .. } => {
                assert!(focus.as_ref().unwrap() == &surface);
                *focus = None;
                events.push(Mouse(mouse::Event::CursorLeft));
            }
            Motion { surface_x: x, surface_y: y, .. } if focus.is_some() => {
                events.push(Mouse(mouse::Event::CursorMoved{x: x as f32, y: y as f32}));
            }
            Button { button, state, .. } if focus.is_some() => {
                events.push(Mouse(if let ButtonState::Pressed = state { mouse::Event::ButtonPressed(conversion::button(button)) }
                                                                                                else { mouse::Event::ButtonReleased(conversion::button(button)) } ));

            }
            Axis { axis, value, .. } if focus.is_some() => {
                let (mut x, mut y) = axis_buffer.unwrap_or((0.0, 0.0));
                use wl_pointer::Axis::*;
                match axis {
                    // wayland vertical sign convention is the inverse of iced
                    VerticalScroll => y -= value as f32,
                    HorizontalScroll => x += value as f32,
                    _ => unreachable!(),
                }
                *axis_buffer = Some((x, y));
            }
            Frame if focus.is_some() => {
                let delta =
                    if let Some((x,y)) = axis_buffer.take() { mouse::ScrollDelta::Pixels{x:x as f32, y:y as f32} }
                    else if let Some((x,y)) = axis_discrete_buffer.take() { mouse::ScrollDelta::Lines{x:x as f32, y:y as f32} }
                    else { /*Enter*/ mouse::ScrollDelta::Pixels{x:0.,y:0.} };
                events.push(Mouse(mouse::Event::WheelScrolled{delta}));
            }
            AxisSource { .. } => (),
            AxisStop { .. } => (),
            AxisDiscrete { axis, discrete } if focus.is_some() => {
                let (mut x, mut y) = axis_discrete_buffer.unwrap_or((0, 0));
                use wl_pointer::Axis::*;
                match axis {
                    // wayland vertical sign convention is the inverse of iced
                    VerticalScroll => y -= discrete,
                    HorizontalScroll => x += discrete,
                    _ => unreachable!(),
                }
                *axis_discrete_buffer = Some((x, y));
            }
            _ => { /*log::trace!("Out of focus")*/ },
        }
    }
}
