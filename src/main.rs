use std::error::Error;
use std::fmt::Display;

use iced::alignment::Vertical;
use iced::widget::text::Shaping;
use iced::widget::{button, column, container, pick_list, row, scrollable, text, Row};
use iced::{Element, Length, Task, Theme};
use itertools::Itertools;
use so2_tool::api::schema::request::{AreaSummary, OfficialItem, People, RecipeItem, ShopSummary};
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
    OfficialItem,
    RecipeItem,
    ShopSummary,
    People,
    AreaSummary,
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
                        LoadTarget::OfficialItem => Self::to_display(
                            APILoader::new(OfficialItem)
                                .get()
                                .await
                                .map(|v| v.0.into_values()),
                        ),
                        LoadTarget::RecipeItem => Self::to_display(
                            APILoader::new(RecipeItem)
                                .get()
                                .await
                                .map(|v| v.0.into_values()),
                        ),
                        LoadTarget::ShopSummary => {
                            //...?
                            Self::to_display(APILoader::new(ShopSummary).get().await.map(|v| [v]))
                        }
                        LoadTarget::People => {
                            Self::to_display(APILoader::new(People).get().await.map(|v| v.0))
                        }
                        LoadTarget::AreaSummary => {
                            Self::to_display(APILoader::new(AreaSummary).get().await.map(|v| v.0))
                        }
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
        let load_button = |label, target| button(label).on_press(Message::Load(target));

        column![
            row![
                load_button("item(official)", LoadTarget::OfficialItem),
                load_button("item(recipe)", LoadTarget::RecipeItem),
                load_button("shop summary", LoadTarget::ShopSummary),
                load_button("people", LoadTarget::People),
                load_button("area summary", LoadTarget::AreaSummary),
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
