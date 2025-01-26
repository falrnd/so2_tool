use iced::alignment::Vertical;
use iced::widget::text::Shaping;
use iced::widget::{button, column, container, pick_list, row, scrollable, text};
use iced::{Element, Length, Task, Theme};
use itertools::Itertools;
use so2_tool::api::item_definition::ItemDefinition;
use so2_tool::api::people::PeopleResponse;

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
    items: String,
    people: String,
    theme: Theme,
}

impl Default for ItemsLabel {
    fn default() -> Self {
        Self {
            items: "Not Loaded".to_string(),
            people: "Not Loaded".to_string(),
            theme: Theme::Dark,
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
                        ItemDefinition::get().await.map_or_else(
                            |e| format!("error: {e}"),
                            |v| v.values().map(ToString::to_string).join("\n"),
                        )
                    },
                    Message::ItemsLoaded,
                )
            }
            Message::ItemsLoaded(v) => {
                println!("Items loaded");
                self.items = v;
            }
            Message::LoadPeople => {
                return Task::perform(
                    async {
                        PeopleResponse::get().await.map_or_else(
                            |e| format!("error: {e}"),
                            |v| v.values().map(|v| format!("{v:?}")).join("\n"),
                        )
                    },
                    Message::PeopleLoaded,
                )
            }
            Message::PeopleLoaded(v) => {
                println!("People loaded");
                self.people = v;
            }
            Message::ThemeChanged(theme) => {
                println!("Theme changed to {} {{{:?}}}", theme, theme.palette());
                self.theme = theme;
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let items_load = button("Load Items").on_press(Message::LoadItems);
        let people_load = button("Load People").on_press(Message::LoadPeople);

        column![
            self.theme_selector_view(),
            row![
                column![items_load, people_load].spacing(5),
                self.scrollable_text_view(&self.items),
                self.scrollable_text_view(&self.people),
            ]
            .spacing(10)
        ]
        .spacing(10)
        .padding(20)
        .into()
    }

    fn element(&self) -> Element<Message> {
        self.view() //.explain(iced::Color::WHITE)
    }

    fn theme_selector_view(&self) -> Element<Message> {
        let title = text("テーマ：")
            .shaping(Shaping::Advanced)
            .width(Length::Shrink);
        let list = pick_list(Theme::ALL, Some(&self.theme), Message::ThemeChanged);

        row![title, list].align_y(Vertical::Center).into()
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
