use crate::row::{self, Row};
use sauron::html::attributes::*;
use sauron::html::events::*;
use sauron::html::*;
use sauron::Component;
use sauron::Node;

#[derive(Debug, Clone)]
pub enum Msg {
    TabClick,
    RowMsg(usize, row::Msg),
}

pub struct Tab {
    tab_clicks: u32,
    rows: Vec<Row>,
    is_active: bool,
    pub name: String,
    pub color: String,
}

impl Tab {
    pub fn new(name: &str, color: &str) -> Self {
        Tab {
            tab_clicks: 0,
            rows: (0..10)
                .into_iter()
                .map(|index| Row::new(format!("Row {}", index)))
                .collect(),
            is_active: false,
            name: name.to_string(),
            color: color.to_string(),
        }
    }

    pub fn show(&mut self) {
        self.is_active = true;
        self.tab_clicks += 1;
    }

    pub fn hide(&mut self) {
        self.is_active = false;
    }
}

impl Component<Msg> for Tab {
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::TabClick => self.tab_clicks += 1,
            Msg::RowMsg(index, row_msg) => self.rows[index].update(row_msg),
        }
    }

    fn view(&self) -> Node<Msg> {
        div(
            [
                class("tab tabcontent"),
                styles([("background-color", &self.color)]),
                styles_flag([
                    ("display", "block", self.is_active),
                    ("display", "none", !self.is_active),
                ]),
            ],
            [
                button(
                    [onclick(|_| Msg::TabClick)],
                    [text(format!(
                        "{} is clicked for {} times",
                        self.name, self.tab_clicks
                    ))],
                ),
                div(
                    [class("rows")],
                    self.rows
                        .iter()
                        .enumerate()
                        .map(|(index, row)| {
                            row.view().map(move |row_msg| Msg::RowMsg(index, row_msg))
                        })
                        .collect::<Vec<Node<Msg>>>(),
                ),
            ],
        )
    }
}
