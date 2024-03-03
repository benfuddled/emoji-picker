use iced::executor;
use iced::widget::{column, container, slider, text, vertical_space};
use iced::{Application, Command, Element, Settings, Theme };
use emojis;
use iced::widget::Slider;

pub fn main() -> iced::Result {
    Picker::run(Settings::default())
}

struct Picker {
    happiness_level: f32,
}

#[derive(Debug, Clone)]
enum Message {
    HappinessLevelChanged(u8)
}

impl Application for Picker {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Picker, Command<Self::Message>) {
        (Picker {
            happiness_level: 10.0
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Picker")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::HappinessLevelChanged(level) => {
                self.happiness_level = level as f32;

                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let rocket = emojis::get("🚀").unwrap();
        let happiness_slider = slider(
            0..=100,
            self.happiness_level as u8,
            Message::HappinessLevelChanged,
        );
        let column = column![text(rocket).size(self.happiness_level).shaping(text::Shaping::Advanced), text(self.happiness_level), happiness_slider];
        container(column).padding(20).center_x().center_y().into()
    }
}