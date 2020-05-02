use {iced_native::{window::Backend, Executor, Command, Element, Subscription}, super::{Mode, Settings, async_sctk}};

pub trait Application: Sized {
    type Backend: Backend + 'static; // wayland-client/DispatchData:Any:'static
    type Executor: Executor;
    type Message: std::fmt::Debug + Send + 'static; // wayland-client/DispatchData:Any:'static
    type Flags;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>);
    fn title(&self) -> String;
    fn update(&mut self, message: Self::Message) -> Command<Self::Message>;
    fn subscription(&self) -> Subscription<Self::Message>;
    fn view(&mut self) -> Element<'_, Self::Message, <Self::Backend as Backend>::Renderer>;
    fn mode(&self) -> Mode { Mode::Windowed }
    fn run(settings: Settings<Self::Flags>, backend: <Self::Backend as Backend>::Settings) {
        smol::run(async_sctk::application(settings, backend));
    }
}
