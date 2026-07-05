use iced_baseview::{
    Alignment, IcedBaseviewSettings, Length, PollSubNotifier, Theme, application,
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

    iced_baseview::open_blocking(
        IcedBaseviewSettings::new()
            .with_title("iced_baseview hello world")
            .with_size(LogicalSize::new(500.0, 300.0)),
        PollSubNotifier::new(),
        || {
            application(MyProgram::default, MyProgram::update, MyProgram::view)
                .theme(MyProgram::theme)
                .run()
        },
    );
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
