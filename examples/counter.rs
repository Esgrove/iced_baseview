use iced_baseview::{
    Center, IcedWindowSettings, PollSubNotifier, Theme, application,
    baseview::dpi::LogicalSize,
    widget::{Column, button, column, text},
};

fn main() {
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(tracing::Level::DEBUG)
            .finish(),
    )
    .unwrap();

    let (window, _message_sender) = iced_baseview::create_window(
        IcedWindowSettings::new()
            .with_title("iced_baseview counter")
            .with_size(LogicalSize::new(500.0, 300.0)),
        PollSubNotifier::new(),
        |_window_size_sub| {
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

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

#[derive(Default)]
struct MyProgram {
    value: i64,
}

impl MyProgram {
    pub fn theme(&self) -> Option<Theme> {
        Some(iced_baseview::Theme::Dark)
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }

    pub fn view(&self) -> Column<'_, Message> {
        column![
            button("Increment").on_press(Message::Increment),
            text(self.value).size(50),
            button("Decrement").on_press(Message::Decrement)
        ]
        .padding(20)
        .align_x(Center)
    }
}
