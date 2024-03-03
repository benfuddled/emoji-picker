use iced::executor;
use iced::widget::{column, container, slider, text, vertical_space, Column, checkbox};
use iced::{Application, Command, Element, Settings, Theme};
use emojis;
use iced::widget::Slider;

pub fn main() -> iced::Result {
    Picker::run(Settings::default())
}

struct Picker {
    happiness_level: f32,
    show_it: bool,
}

#[derive(Debug, Clone)]
enum Message {
    HappinessLevelChanged(u8),
    ToggleTwo(bool)
}

impl Application for Picker {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Picker, Command<Self::Message>) {
        (Picker {
            happiness_level: 10.0,
            show_it: false
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
            Message::ToggleTwo(show_it) => {
                self.show_it = show_it;

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
        let show_two = checkbox("show two", self.show_it).on_toggle(Message::ToggleTwo);
        let column = column![show_content(self.happiness_level, self.show_it), show_two, text(self.happiness_level), happiness_slider];
        // container(column).padding(20).center_x().center_y().into()
        //let column = column![show_content(self.happiness_level)];
        container(column).into()
    }
}

fn show_content<'a>(happiness_level: f32, show_it: bool) -> Element<'a, Message> {
    let rocket = emojis::get("🚀").unwrap();
    let text_thing = text(rocket).size(happiness_level).shaping(text::Shaping::Advanced);
    let mut col = Column::new().push(text_thing);
    if show_it == true {
        col = col.push(text(rocket).size(happiness_level).shaping(text::Shaping::Advanced));
    }
    col.into()
}