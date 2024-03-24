use iced::{alignment, Background, Border, clipboard, Color, executor, Length, Padding, Pixels, Shadow, Size, Vector, widget, window};
use iced::widget::{column, container, text, Column, text_input, Scrollable, Row, Container, Button};
use iced::{Application, Command, Element, Settings, Theme};
use emojis;

pub fn main() -> iced::Result {
    Picker::run(Settings {
        window: window::Settings {
            size: Size { width: (450.0), height: (540.0) },
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
    search_val: String
}

#[derive(Debug, Clone)]
enum Message {
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
            search_val: String::from("")
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Emoji Picker")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
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
        let text_input = text_input("Search", &*self.search_val).on_input(Message::SearchInput);
        let column = column![show_content_grid(&self.search_val)].width(Length::Fill);
        let scroll_grid = Scrollable::new(column).height(Length::Fill);

        let scroll_container = Container::new(scroll_grid).height(Length::Fill);
        let text_container = Container::new(text_input).padding(Padding::from([12, 25, 8, 14]));

        let window_column = column![text_container, scroll_container];
        container(window_column).into()
    }
}

fn show_content_grid<'a>(search_val: &str) -> Element<'a, Message> {

    let mut col = Column::new();

    // Library recommends to filter the list by the maximum Unicode version that you wish to support.
    let minimum_moji = emojis::iter().filter(|e| e.unicode_version() < emojis::UnicodeVersion::new(15, 1)).collect::<Vec<_>>();
    // Only render emoji that match name.
    let filtered_moji = minimum_moji.iter().filter(|e| e.name().contains(search_val)).collect::<Vec<_>>();

    // We collected iter into vector, so we can break it into chunks for each row.
    for moji_row in filtered_moji.chunks(5) {
        let mut row: Row<Message> = Row::new().padding(4).spacing(8);
        for moji in moji_row {
            let txt = text(moji).size(24.0).shaping(text::Shaping::Advanced);
            let txt_container = Container::new(txt).center_x().width(iced::Length::Fill);
            let btn = Button::new(txt_container).on_press(Message::EmojiPressed(String::from(moji.as_str()))).width(Pixels(75.0));
            let btn_container = Container::new(btn).center_x().width(Pixels(75.0));

            row = row.push(btn_container);
        }
        let row_holder = Container::new(row).center_x().align_x(alignment::Horizontal::Center).width(Length::Fixed(440.0));
        col = col.push(row_holder);
    }

    col.into()
}

// create the theme
// call .style(container_theme()) on any container for layout debugging.
// via https://discord.com/channels/628993209984614400/1213838081103237180/1213838081103237180
fn container_theme() -> widget::container::Appearance {
    widget::container::Appearance {
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
        ..widget::container::Appearance::default()
    }
}