//! Display information and interactive controls in your application.
//!
//! # Re-exports
//! For convenience, the contents of this module are available at the root
//! module. Therefore, you can directly type:
//!
//! ```
//! use iced::{button, Button};
//! ```
//!
//! # Stateful widgets
//! Some widgets need to keep track of __local state__.
//!
//! These widgets have their own module with a `State` type. For instance, a
//! [`TextInput`] has some [`text_input::State`].
//!
//! [`TextInput`]: text_input/struct.TextInput.html
//! [`text_input::State`]: text_input/struct.State.html
#[cfg(feature = "iced_shm")]
mod platform {
    pub use iced_shm::widget::{
        button, checkbox, container, pane_grid, progress_bar, radio,
        scrollable, slider, text_input,
    };

    #[cfg(feature = "canvas")]
    #[cfg_attr(docsrs, doc(cfg(feature = "canvas")))]
    pub use iced_shm::widget::canvas;

    #[cfg_attr(docsrs, doc(cfg(feature = "image")))]
    pub mod image {
        //! Display images in your user interface.
        #[cfg(feature = "image")]
        pub use iced_shm::image::{Handle, Image};
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "svg")))]
    pub mod svg {
        //! Display vector graphics in your user interface.
        #[cfg(feature = "svg")]
        pub use iced_shm::svg::{Handle, Svg};
    }

    use crate::runtime;
    pub use runtime::Space;

    #[doc(no_inline)]
    pub use text_input::TextInput;

    #[cfg(feature = "canvas")]
    #[doc(no_inline)]
    pub use canvas::Canvas;

    /// A container that distributes its contents vertically.
    ///
    /// This is an alias of an `iced_native` column with a default `Renderer`.
    pub type Column<'a, Message> = runtime::Column<'a, Message, iced_shm::Renderer>;

    /// A container that distributes its contents horizontally.
    ///
    /// This is an alias of an `iced_native` row with a default `Renderer`.
    pub type Row<'a, Message> = runtime::Row<'a, Message, iced_shm::Renderer>;
}

#[cfg(feature = "iced_wgpu")]
mod platform {
    pub use iced_wgpu::widget::{
        button, checkbox, container, pane_grid, progress_bar, radio,
        scrollable, slider, text_input, Text,
    };

    #[cfg(feature = "canvas")]
    #[cfg_attr(docsrs, doc(cfg(feature = "canvas")))]
    pub use iced_wgpu::widget::canvas;

    #[cfg_attr(docsrs, doc(cfg(feature = "image")))]
    pub mod image {
        //! Display images in your user interface.
        pub use crate::runtime::image::{Handle, Image};
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "svg")))]
    pub mod svg {
        //! Display vector graphics in your user interface.
        pub use crate::runtime::svg::{Handle, Svg};
    }

    use crate::runtime;
    pub use runtime::Space;

    #[doc(no_inline)]
    pub use {
        button::Button, checkbox::Checkbox, container::Container, image::Image,
        pane_grid::PaneGrid, progress_bar::ProgressBar, radio::Radio,
        scrollable::Scrollable, slider::Slider, svg::Svg,
        text_input::TextInput,
    };

    #[cfg(feature = "canvas")]
    #[doc(no_inline)]
    pub use canvas::Canvas;

    /// A container that distributes its contents vertically.
    ///
    /// This is an alias of an `iced_native` column with a default `Renderer`.
    pub type Column<'a, Message> =
        runtime::Column<'a, Message, iced_wgpu::Renderer>;

    /// A container that distributes its contents horizontally.
    ///
    /// This is an alias of an `iced_native` row with a default `Renderer`.
    pub type Row<'a, Message> =
        runtime::Row<'a, Message, iced_wgpu::Renderer>;
}

#[cfg(target_arch = "wasm32")]
mod platform {
    pub use iced_web::widget::*;
}

pub use platform::*;
