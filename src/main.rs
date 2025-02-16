use iced::alignment::Vertical;
use iced::widget::text::Shaping;
use iced::widget::{button, column, container, pick_list, row, scrollable, text, Row};
use iced::{Element, Length, Task, Theme};
use itertools::Itertools;
use so2_tool::api::schema::{item, people};

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
    display: String,
    theme: Theme,
}

impl Default for ItemsLabel {
    fn default() -> Self {
        Self {
            display: "press button".to_string(),
            theme: Theme::TokyoNightStorm,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    ThemeChanged(Theme),
    LoadItems,
    ItemsLoaded(String),
    LoadPeople,
    PeopleLoaded(String),
}

impl ItemsLabel {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::LoadItems => {
                return Task::perform(
                    async {
                        item::Response::get()
                            .await
                            .map_or_else(|e| format!("error: {e}"), |v| v.values().join("\n"))
                    },
                    Message::ItemsLoaded,
                )
            }
            Message::ItemsLoaded(v) => {
                println!("Items loaded");
                self.display = v;
            }
            Message::LoadPeople => {
                return Task::perform(
                    async {
                        people::Response::get()
                            .await
                            .map_or_else(|e| format!("error: {e}"), |v| v.values().join("\n"))
                    },
                    Message::PeopleLoaded,
                )
            }
            Message::PeopleLoaded(v) => {
                println!("People loaded");
                self.display = v;
            }
            Message::ThemeChanged(theme) => {
                println!("Theme changed to {} {{{:?}}}", theme, theme.palette());
                self.theme = theme;
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let items_load = button("Item").on_press(Message::LoadItems);
        let people_load = button("People").on_press(Message::LoadPeople);

        column![
            row![
                items_load,
                people_load,
                container(self.theme_selector_view()).align_right(Length::Fill)
            ]
            .width(Length::Fill)
            .spacing(5),
            self.scrollable_text_view(&self.display),
        ]
        .spacing(10)
        .padding(20)
        .into()
    }

    fn element(&self) -> Element<Message> {
        self.view() //.explain(iced::Color::WHITE)
    }

    fn theme_selector_view(&self) -> Row<Message> {
        let title = text("ColorTheme: ")
            .shaping(Shaping::Advanced)
            .width(Length::Shrink);
        let list = pick_list(Theme::ALL, Some(&self.theme), Message::ThemeChanged);

        row![title, list].align_y(Vertical::Center)
    }

    fn scrollable_text_view<'a>(&self, str: &'a str) -> Element<'a, Message> {
        let text = text(str).size(10).shaping(Shaping::Advanced);
        container(scrollable(text).spacing(5))
            .style(container::rounded_box)
            .padding(5)
            .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
