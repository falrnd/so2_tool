use iced::alignment::Vertical;
use iced::widget::text::Shaping;
use iced::widget::{button, column, container, pick_list, row, scrollable, text};
use iced::{Element, Length, Task, Theme};
use itertools::Itertools;
use so2_tool::api::item_definition;

pub fn main() -> iced::Result {
    iced::application(
        "SOLD OUT 2 tools by fal_rnd",
        ItemsLabel::update,
        ItemsLabel::element,
    )
    .theme(ItemsLabel::theme)
    .run()
}

struct ItemsLabel {
    text: String,
    theme: Theme,
}

impl Default for ItemsLabel {
    fn default() -> Self {
        Self {
            text: "Not Loaded".to_string(),
            theme: Theme::Dark,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    ThemeChanged(Theme),
    LoadButtonPushed,
    Loaded(String),
}

impl ItemsLabel {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::LoadButtonPushed => {
                return Task::perform(
                    async {
                        item_definition::get().await.map_or_else(
                            |e| format!("error: {e}"),
                            |v| v.values().map(ToString::to_string).join("\n"),
                        )
                    },
                    Message::Loaded,
                )
            }
            Message::Loaded(items) => {
                self.text = items;
            }
            Message::ThemeChanged(theme) => {
                println!("Theme changed to {} {{{:?}}}", theme, theme.palette());
                self.theme = theme;
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let load_button = button("Load").on_press(Message::LoadButtonPushed);

        let items_text = text(&self.text).size(10).shaping(Shaping::Advanced);

        let scrolled_items = container(scrollable(items_text))
            .style(iced::widget::container::rounded_box)
            .padding(5);

        column![
            self.theme_selector(),
            row![load_button, scrolled_items].spacing(5)
        ]
        .spacing(10)
        .padding(20)
        .into()
    }

    fn element(&self) -> Element<Message> {
        self.view() //.explain(iced::Color::WHITE)
    }

    fn theme_selector(&self) -> Element<Message> {
        let title = text("テーマ：")
            .shaping(Shaping::Advanced)
            .width(Length::Shrink);
        let list = pick_list(Theme::ALL, Some(&self.theme), Message::ThemeChanged);

        row![title, list].align_y(Vertical::Center).into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
