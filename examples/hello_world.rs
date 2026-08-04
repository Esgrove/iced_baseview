use iced_baseview::{
    Alignment, IcedWindowSettings, Length, PollSubNotifier, Theme, application,
    baseview::dpi::LogicalSize,
    widget::{Column, Container, Text},
};

fn main() {
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(tracing::Level::DEBUG)
            .finish(),
    )
    .unwrap();

    let (window, _iced_window_handle) = iced_baseview::create_window(
        IcedWindowSettings::new()
            .with_title("iced_baseview hello world")
            .with_size(LogicalSize::new(500.0, 300.0)),
        PollSubNotifier::new(),
        || {
            Ok(
                application(MyProgram::default, MyProgram::update, MyProgram::view)
                    .theme(MyProgram::theme)
                    .run(),
            )
        },
        None,
    )
    .unwrap();

    window.run_until_closed().unwrap();
}

#[derive(Default)]
struct MyProgram;

impl MyProgram {
    pub fn theme(&self) -> Option<Theme> {
        Some(iced_baseview::Theme::Dark)
    }

    pub fn update(&mut self, _message: ()) {}

    pub fn view(&self) -> Container<'_, ()> {
        let content = Column::new()
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .push(Text::new("Hello World!"));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center(Length::Fill)
            .into()
    }
}
