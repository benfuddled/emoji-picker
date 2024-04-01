use cosmic::app::{Command, Core, Settings};
use cosmic::iced_core::Size;
use cosmic::widget::{Container, container, text_input, Column, Row, text, Button};
use cosmic::{executor, iced, ApplicationExt, Element};
use cosmic::iced::{Length, Padding, Pixels, alignment, font};
use cosmic::iced_widget::Scrollable;
use arboard::Clipboard;
use emojis;

const EMOJI_FONT_FAMILY: iced::Font = iced::Font::with_name("Noto Color Emoji");

/// Runs application with these settings
#[rustfmt::skip]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let _ = tracing_log::LogTracer::init();

    let settings = Settings::default()
        .antialiasing(true)
        .client_decorations(true)
        .debug(false)
        .default_icon_theme("Pop")
        .default_text_size(16.0)
        .scale_factor(1.0)
        .resizable(Option::from(0.0))
        .size(Size::new(454.0, 544.0));

    cosmic::app::run::<App>(settings, ())?;

    Ok(())
}

/// Messages that are used specifically by our [`App`].
#[derive(Clone, Debug)]
pub enum Message {
    SearchInput(String),
    EmojiPressed(String),
    FontLoaded(Result<(), font::Error>),
}

/// The [`App`] stores application-specific state.
pub struct App {
    core: Core,
    clipboard: Clipboard,
    search_val: String
}

/// Implement [`cosmic::Application`] to integrate with COSMIC.
impl cosmic::Application for App {
    /// Default async executor to use with the app.
    type Executor = executor::Default;

    /// Argument received [`cosmic::Application::new`].
    type Flags = ();

    /// Message type specific to our [`App`].
    type Message = Message;

    /// The unique application ID to supply to the window manager.
    const APP_ID: &'static str = "org.cosmic.AppDemo";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    /// Creates the application, and optionally emits command on initialize.
    fn init(core: Core, _input: Self::Flags) -> (Self, Command<Self::Message>) {
        // Using arboard due to weirdness with iced clipboard and libcosmic.
        // TODO: Figure out why iced clipboard isn't working.
        let clipboard = Clipboard::new().unwrap();

        let mut app = App { core, search_val: String::from(""), clipboard: clipboard };

        app.set_header_title(String::from("Emoji Picker"));

        (app, Command::batch([
            Command::none()
        ]))
    }

    /// Handle application events here.
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::FontLoaded(_) => {
                Command::none()
            }
            Message::SearchInput(input) => {
                self.search_val = input;
                println!("{}", self.search_val);
                Command::none()
            }
            Message::EmojiPressed(input) => {
                self.clipboard.set_text(input).unwrap();
                Command::none()
            }
        }
    }

    /// Creates a view after each update.
    fn view(&self) -> Element<Self::Message> {
        let text_input = text_input("Search", &*self.search_val).on_input(Message::SearchInput);
        //let column = column![show_content_grid(&self.search_val)].width(Length::Fill);
        let mut column = Column::new().width(Length::Fill);
        column = column.push(show_content_grid(&self.search_val));
        let scroll_grid = Scrollable::new(column).height(Length::Fill);

        let scroll_container = Container::new(scroll_grid).height(Length::Fill);
        let text_container = Container::new(text_input).padding(Padding::from([12, 16, 8, 14]));

        // let window_column = column(text_container, scroll_container);
        let window_column = Column::new();
        let window_column = window_column.push(text_container);
        let window_column = window_column.push(scroll_container);
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
            let txt = text(moji.as_str()).size(24.0).font(EMOJI_FONT_FAMILY);
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