use std::error::Error;
use std::fmt::Display;

use iced::alignment::Vertical;
use iced::widget::text::Shaping;
use iced::widget::{button, column, container, pick_list, row, scrollable, text, Row};
use iced::{Element, Length, Task, Theme};
use itertools::Itertools;
use so2_tool::api::model::{item, people};
use so2_tool::app::api_loader::APILoader;

pub fn main() -> iced::Result {
    iced::application(
        "SOLD OUT 2 tool by fal_rnd",
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

#[derive(Debug, Clone, Copy)]
enum LoadTarget {
    Item,
    People,
}

#[derive(Debug, Clone)]
enum Message {
    ThemeChanged(Theme),
    Load(LoadTarget),
    Loaded(String),
}

impl ItemsLabel {
    fn to_display<Iter>(v: Result<Iter, Box<dyn Error>>) -> String
    where
        Iter: IntoIterator,
        Iter::Item: Display,
    {
        v.map_or_else(|e| format!("error: {e}"), |v| v.into_iter().join("\n"))
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Load(target) => Task::perform(
                async move {
                    match target {
                        LoadTarget::Item => Self::to_display(
                            APILoader::new(item::Request {})
                                .load_cache_or_call()
                                .await
                                .map(|v| v.into_values()),
                        ),
                        LoadTarget::People => Self::to_display(
                            APILoader::new(people::Request {})
                                .load_cache_or_call()
                                .await
                                .map(|v| v.into_values()),
                        ),
                    }
                },
                Message::Loaded,
            ),
            Message::Loaded(v) => {
                self.display = v;
                Task::none()
            }
            Message::ThemeChanged(theme) => {
                println!("Theme changed to {} {{{:?}}}", theme, theme.palette());
                self.theme = theme;
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let items_load = button("Item").on_press(Message::Load(LoadTarget::Item));
        let people_load = button("People").on_press(Message::Load(LoadTarget::People));

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
