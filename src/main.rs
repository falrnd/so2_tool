use std::error::Error;
use std::fmt::{Debug, Display};

use chrono::Timelike;
use iced::alignment::Vertical;
use iced::widget::text::Shaping;
use iced::widget::{Row, button, column, container, pick_list, row, scrollable, text};
use iced::{Element, Length, Task, Theme};
use itertools::Itertools;
use so2_tool::api::schema::{
    Area, AreaSummary, OfficialItem, People, RankingAllMonthly, RankingSectionDaily,
    RankingSectionMonthly, RecipeItem, Report, Request, RequestReport, Sale, Schema, Shop,
    ShopSummary,
};
use so2_tool::app::api_loader::APILoader;
use so2_tool::app::cache::DEFAULT_CACHE_ROOT;
use so2_tool::app::delete_expired_cache;

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
    Area,
    Report,
    Ranking(Ranking),
    Sale,
    Request,
    ShopSummary,
    Shop,
    People,
    RequestReport,
    AreaSummary,
}

#[derive(Debug, Clone, Copy)]
enum Ranking {
    All,
    Section,
    Daily,
}

#[derive(Debug, Clone)]
enum Message {
    ThemeChanged(Theme),
    Load(LoadTarget),
    Loaded(String),
    DeleteCache,
}

impl ItemsLabel {
    fn to_display<Iter>(v: Result<Iter, Box<dyn Error>>) -> String
    where
        Iter: IntoIterator,
        Iter::Item: Display,
    {
        v.inspect_err(|e| eprintln!("{e}"))
            .map_or_else(|e| format!("error: {e}"), |v| v.into_iter().join("\n"))
    }

    fn to_debug<Iter>(v: Result<Iter, Box<dyn Error>>) -> String
    where
        Iter: IntoIterator,
        Iter::Item: Debug,
    {
        Self::to_display(v.map(|v| v.into_iter().map(|v| format!("{v:?}"))))
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
                        LoadTarget::Area => Self::to_debug(
                            APILoader::new(Area).get().await.map(|v| v.into_values()),
                        ),
                        LoadTarget::Report => {
                            let instant = chrono::Local::now() - RankingAllMonthly::min_interval();
                            Self::to_debug(
                                APILoader::new(Report(instant.date_naive().pred_opt().unwrap()))
                                    .get()
                                    .await
                                    .map(|v| [v]),
                            )
                        }
                        LoadTarget::Ranking(r) => {
                            let instant = chrono::Local::now() - RankingAllMonthly::min_interval();

                            match r {
                                Ranking::All => Self::to_debug(
                                    APILoader::new(RankingAllMonthly {
                                        ym: instant.date_naive(),
                                    })
                                    .get()
                                    .await
                                    .map(|v| v.0),
                                ),
                                Ranking::Section => Self::to_debug(
                                    APILoader::new(RankingSectionMonthly {
                                        ym: instant.date_naive(),
                                        section: "exp_62".to_string(),
                                    })
                                    .get()
                                    .await
                                    .map(|v| v.0),
                                ),
                                Ranking::Daily => Self::to_debug(
                                    APILoader::new(RankingSectionDaily {
                                        date: instant.date_naive(),
                                        section: "exp_62".to_string(),
                                    })
                                    .get()
                                    .await
                                    .map(|v| v.0),
                                ),
                            }
                        }
                        LoadTarget::Sale => Self::to_debug(APILoader::new(Sale).get().await),
                        LoadTarget::Request => Self::to_debug(APILoader::new(Request).get().await),
                        LoadTarget::ShopSummary => {
                            Self::to_display(APILoader::new(ShopSummary).get().await.map(|v| [v]))
                        }
                        LoadTarget::Shop => Self::to_debug(APILoader::new(Shop).get().await),
                        LoadTarget::People => {
                            Self::to_display(APILoader::new(People).get().await.map(|v| v.0))
                        }

                        LoadTarget::RequestReport => {
                            let instant = chrono::Local::now() - RequestReport::min_interval();
                            let date = instant.date_naive();
                            let hour = instant.hour() as u8;
                            Self::to_display(
                                APILoader::new(RequestReport::All { date, hour })
                                    .get()
                                    .await
                                    .map(|v| v.0),
                            )
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
            Message::DeleteCache => {
                delete_expired_cache::delete(&DEFAULT_CACHE_ROOT).unwrap();
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let load_button = |label, target| button(label).on_press(Message::Load(target));

        column![
            container(
                row![
                    button("delete cache").on_press(Message::DeleteCache),
                    container(self.theme_selector_view())
                ]
                .spacing(5)
            )
            .align_y(Vertical::Center)
            .align_right(Length::Fill)
            .width(Length::Fill),
            row![
                column![
                    load_button("item(official)", LoadTarget::OfficialItem),
                    load_button("item(recipe)", LoadTarget::RecipeItem),
                    load_button("area", LoadTarget::Area),
                    load_button("report", LoadTarget::Report),
                    load_button("ranking(all)", LoadTarget::Ranking(Ranking::All)),
                    load_button("ranking(section)", LoadTarget::Ranking(Ranking::Section)),
                    load_button("ranking(daily)", LoadTarget::Ranking(Ranking::Daily)),
                    load_button("sale", LoadTarget::Sale),
                    load_button("request", LoadTarget::Request),
                    load_button("shop summary", LoadTarget::ShopSummary),
                    load_button("shop", LoadTarget::Shop),
                    load_button("people", LoadTarget::People),
                    load_button("request report", LoadTarget::RequestReport),
                    load_button("area summary", LoadTarget::AreaSummary),
                ]
                .spacing(5),
                self.scrollable_text_view(&self.display),
            ]
            .spacing(5)
        ]
        .spacing(10)
        .padding(20)
        .into()
    }

    fn element(&self) -> Element<Message> {
        self.view() //.explain(iced::Color::WHITE)
    }

    fn theme_selector_view(&self) -> Row<Message> {
        let title = text("ColorTheme: ").shaping(Shaping::Advanced);
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
