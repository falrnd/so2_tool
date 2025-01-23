use iced::widget::text::Shaping;
use iced::widget::{self, column, row};
use iced::{Element, Task};
use so2_tool::api::item_definition;

pub fn main() -> iced::Result {
    iced::run(
        "SOLD OUT 2 tools by fal_rnd",
        ItemsLabel::update,
        ItemsLabel::view,
    )
}

struct ItemsLabel {
    text: String,
}

impl Default for ItemsLabel {
    fn default() -> Self {
        Self {
            text: "Not Loaded".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    LoadButtonPushed,
    Loaded(String),
}

impl ItemsLabel {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::LoadButtonPushed => Task::perform(
                async {
                    item_definition::get()
                        .await
                        .map_or_else(|e| format!("error: {e}"), |v| v.value)
                },
                Message::Loaded,
            ),
            Message::Loaded(items) => {
                self.text = items;
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![row![
            widget::button("Load").on_press(Message::LoadButtonPushed),
            widget::scrollable(widget::text(&self.text).size(10).shaping(Shaping::Advanced)),
        ],]
        .padding(20)
        .into()
    }
}
