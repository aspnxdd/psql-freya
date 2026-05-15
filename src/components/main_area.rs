use crate::db::run_query;
use crate::models::{AppChannel, QueryResult};
use freya::prelude::*;
use freya::radio::*;
use regex::Regex;

#[derive(PartialEq)]
pub struct MainArea;

impl Component for MainArea {
    fn render(&self) -> impl IntoElement {
        let mut radio = use_radio(AppChannel::Ui);
        let client = radio.read().client.clone();
        let query_results = radio.read().query_results.clone();
        let error_message = radio.read().error_message.clone();
        let mut query_text = use_state(|| String::new());

        let on_run = move |_| {
            let raw_sql = query_text.read().clone();
            if raw_sql.is_empty() {
                return;
            }
            let sql = auto_quote_sql(&raw_sql);
            if sql != raw_sql {
                query_text.set(sql.clone());
            }
            if let Some(client) = &client {
                let client = client.clone();
                spawn(async move {
                    match run_query(&client, &sql).await {
                        Ok(result) => {
                            let mut state = radio.write();
                            state.query_results = Some(result);
                            state.error_message = None;
                        }
                        Err(e) => {
                            radio.write().error_message = Some(format!("Query failed: {}", e));
                        }
                    }
                });
            }
        };

        rect()
            .expanded()
            .height(Size::fill())
            .background((30, 30, 30))
            .direction(Direction::Vertical)
            .padding(Gaps::new_all(12.))
            .child(
                rect()
                    .direction(Direction::Horizontal)
                    .spacing(8.)
                    .height(Size::px(47.))
                    .content(Content::Flex)
                    .child(
                        Input::new(query_text.into_writable())
                            .placeholder("SQL query...")
                            .width(Size::flex(1.))
                            .expanded(),
                    )
                    .child(
                        Button::new()
                            .height(Size::fill())
                            .on_press(on_run)
                            .filled()
                            .child("Run"),
                    ),
            )
            .map(error_message, |el, err| {
                el.child(
                    rect()
                        .background((80, 30, 30))
                        .corner_radius(CornerRadius::new_all(6.))
                        .padding(Gaps::new_all(8.))
                        .child(label().text(err).color((255, 150, 150)).font_size(12.)),
                )
            })
            .child(results_grid(query_results))
    }
}

fn results_grid(results: Option<QueryResult>) -> impl IntoElement {
    match results {
        Some(result) => {
            let cols = result.columns.clone();
            let rows = result.rows.clone();

            rect()
                .expanded()
                .child(
                    VirtualScrollView::new_with_data(
                        (cols.clone(), rows.clone()),
                        move |i, (cols, rows)| {
                            if i == 0 {
                                rect()
                                    .key(0)
                                    .height(Size::px(24.))
                                    .background((40, 40, 40))
                                    .direction(Direction::Horizontal)
                                    .children(cols.iter().enumerate().map(|(j, col)| {
                                        rect()
                                            .key(j)
                                            .width(Size::px(140.))
                                            .padding(Gaps::new_all(4.))
                                            .child(label().text(col.clone()).color(Color::WHITE).font_size(11.))
                                            .into_element()
                                    }))
                                    .into()
                            } else {
                                let row = &rows[i - 1];
                                rect()
                                    .key(i)
                                    .height(Size::px(24.))
                                    .direction(Direction::Horizontal)
                                    .children(row.iter().enumerate().map(|(j, cell)| {
                                        rect()
                                            .key(j)
                                            .width(Size::px(140.))
                                            .padding(Gaps::new_all(4.))
                                            .child(
                                                label()
                                                    .text(cell.clone())
                                                    .color((200, 200, 200))
                                                    .font_size(11.),
                                            )
                                            .into_element()
                                    }))
                                    .into()
                            }
                        }
                    )
                    .length(rows.len() + 1)
                    .item_size(24.)
                    .expanded()
                )
        }
        None => rect().expanded().center().child(
            label()
                .text("Run a query or select a table")
                .color((120, 120, 120))
                .font_size(14.),
        ),
    }
}

fn auto_quote_sql(sql: &str) -> String {
    let re = Regex::new(r#"(?i)\b(from|join)\s+([a-zA-Z_][a-zA-Z0-9_]*)\.([a-zA-Z_][a-zA-Z0-9_]*)"#).unwrap();
    re.replace_all(sql, |caps: &regex::Captures| {
        let keyword = caps.get(1).unwrap().as_str();
        let schema = caps.get(2).unwrap().as_str();
        let table = caps.get(3).unwrap().as_str();
        format!(r#"{} "{}"."{}""#, keyword, schema, table)
    })
    .to_string()
}
