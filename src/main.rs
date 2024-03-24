use iced::{Alignment, alignment, Background, Border, clipboard, Color, executor, Length, Padding, Pixels, Shadow, Size, Vector, window};
use iced::widget::{column, container, slider, text, vertical_space, Column, checkbox, text_input, Scrollable, Row, Text, button, Container, Button};
use iced::{Application, Command, Element, Settings, Theme};
use emojis;
use iced::widget::container::Appearance;

pub fn main() -> iced::Result {
    Picker::run(Settings {
        window: window::Settings {
            size: Size { width: (530.0), height: (600.0) },
            position: Default::default(),
            min_size: None,
            max_size: None,
            visible: true,
            resizable: false,
            decorations: true,
            transparent: false,
            level: Default::default(),
            icon: None,
            platform_specific: Default::default(),
            exit_on_close_request: true,
        },
        ..Default::default() // Noob note: Makes the rest of the settings the struct default.
    })
}

struct Picker {
    happiness_level: f32,
    show_it: bool,
    search_val: String
}

#[derive(Debug, Clone)]
enum Message {
    HappinessLevelChanged(u8),
    ToggleTwo(bool),
    SearchInput(String),
    EmojiPressed(String)
}

impl Application for Picker {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Picker, Command<Self::Message>) {
        (Picker {
            happiness_level: 24.0,
            show_it: false,
            search_val: String::from("")
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Emoji Picker")
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
            Message::SearchInput(input) => {
                self.search_val = input;
                println!("{}", self.search_val);
                Command::none()
            }
            Message::EmojiPressed(input) => {
                println!("{}", input);
                return clipboard::write(input);
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let happiness_slider = slider(
            0..=100,
            self.happiness_level as u8,
            Message::HappinessLevelChanged,
        );
        let text_input = text_input("search!!", &*self.search_val).on_input(Message::SearchInput);
        let show_two = checkbox("show two", self.show_it).on_toggle(Message::ToggleTwo);
        let column = column![show_content_grid(self.happiness_level, self.show_it, &self.search_val), show_two, text(self.happiness_level), happiness_slider].width(Length::Fill);
        let scroll_me = Scrollable::new(column).height(Length::Fill);

        let scroll_container = Container::new(scroll_me).height(Length::Fill);
        let text_container = Container::new(text_input);

        let window_column = column![text_container, scroll_container];
        // container(column).padding(20).center_x().center_y().into()
        //let column = column![show_content(self.happiness_level)];
        container(window_column).into()
    }
}

fn show_content<'a>(happiness_level: f32, show_it: bool, search_val: &str) -> Element<'a, Message> {
    let rocket = emojis::get("🚀").unwrap();
    let stars = emojis::get("🤩").unwrap();
    let grape = emojis::get("🍇").unwrap();
    let text_thing = text(rocket).size(happiness_level).shaping(text::Shaping::Advanced);
    let mut col = Column::new().push(text_thing);
    if show_it == true {
        col = col.push(text(stars).size(happiness_level).shaping(text::Shaping::Advanced));
    }
    if search_val == "grape" {
        col = col.push(text(grape).size(happiness_level).shaping(text::Shaping::Advanced));
    }
    col.into()
}

fn show_content_grid<'a>(happiness_level: f32, show_it: bool, search_val: &str) -> Element<'a, Message> {
    let rocket = emojis::get("🚀").unwrap();
    let stars = emojis::get("🤩").unwrap();
    //let grape = emojis::get("🍇").unwrap();
    let text_thing = text(rocket).size(happiness_level).shaping(text::Shaping::Advanced);
    let mut col = Column::new().push(text_thing).width(Length::Fill);
    // if show_it == true {
    //     col = col.push(text(stars).size(happiness_level).shaping(text::Shaping::Advanced));
    // }
    // if search_val == "grape" {
    //     col = col.push(text(grape).size(happiness_level).shaping(text::Shaping::Advanced));
    // }

    // let mojis = emojis::iter().filter(|e| e.unicode_version() < emojis::UnicodeVersion::new(13, 0));
    //
    // for e in mojis {
    //     if search_val.len() <= 0 {
    //         col = col.push(text(e).size(happiness_level).shaping(text::Shaping::Advanced));
    //     } else if e.name().contains(search_val) {
    //         col = col.push(text(e).size(happiness_level).shaping(text::Shaping::Advanced));
    //     }
    // }

    // Library recommends to filter the list by the maximum Unicode version that you wish to support.
    let minimum_moji = emojis::iter().filter(|e| e.unicode_version() < emojis::UnicodeVersion::new(13, 0)).collect::<Vec<_>>();
    // Only render emoji that match name.
    let filtered_moji = minimum_moji.iter().filter(|e| e.name().contains(search_val)).collect::<Vec<_>>();

    // We collected iter into vector so we can break it into chunks for each row.
    for moji_row in filtered_moji.chunks(6) {
        let mut row: Row<Message> = Row::new().padding(4).spacing(8);
        for moji in moji_row {
            let txt = text(moji).size(happiness_level).shaping(text::Shaping::Advanced);
            let txt_container = Container::new(txt).center_x().width(iced::Length::Fill);
            let btn = Button::new(txt_container).on_press(Message::EmojiPressed(String::from(moji.as_str()))).width(Pixels(75.0));
            let btn_container = Container::new(btn).center_x().width(Pixels(75.0));
            // row = row.push(button(txt_container)
            //     .on_press(Message::EmojiPressed(String::from(moji.as_str()))));

            row = row.push(btn_container);
        }
        let row_holder = Container::new(row).center_x().align_x(alignment::Horizontal::Center).width(Length::Fixed(520.0));
        col = col.push(row_holder);
    }

    col.into()
}

// create the theme
// call .style(container_theme()) on any container for layout debugging.
// via https://discord.com/channels/628993209984614400/1213838081103237180/1213838081103237180
fn container_theme() -> Appearance {
    Appearance {
        border: Border {
            width: 2.0,
            color: Color::BLACK,
            ..Border::default()
        },
        background: Some(Background::Color(Color::from_rgb(0.0, 0.0, 0.0))),
        shadow: Shadow {
            color: Color::from_rgb(0.0, 0.0, 0.0),
            offset: Vector::new(0.0, 8.0),
            blur_radius: 2.0,
        },
        ..Appearance::default()
    }
}