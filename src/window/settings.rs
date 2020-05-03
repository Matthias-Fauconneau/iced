/// The window settings of an application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Settings {
    /// The size of the window.
    pub size: (u32, u32),

    /// Whether the window should be resizable or not.
    pub resizable: bool,

    /// Whether the window should have a border, a title bar, etc. or not.
    pub decorations: bool,

    /// Whether the surface should be a layer-shell overlay (unsupported by winit)
    pub overlay: bool,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            size: (1024, 768),
            resizable: true,
            decorations: true,
            overlay: false,
        }
    }
}
