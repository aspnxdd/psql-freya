use crate::db::run_query;
use crate::models::{AppChannel, QueryResult};
use freya::prelude::*;
use freya::radio::*;

#[derive(PartialEq)]
pub struct MainArea;

impl Component for MainArea {
    fn render(&self) -> impl IntoElement {
        let mut radio = use_radio(AppChannel::Ui);
        let client = radio.read().client.clone();
        let query_results = radio.read().query_results.clone();
        let error_message = radio.read().error_message.clone();
        let query_text = use_state(|| String::new());

        let on_run = move |_| {
            let sql = query_text.read().clone();
            if sql.is_empty() {
                return;
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
            .background((30, 30, 35))
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
                .direction(Direction::Vertical)
                .child(
                    rect()
                        .direction(Direction::Horizontal)
                        .background((50, 50, 60))
                        .children(cols.iter().map(|col| {
                            rect()
                                .width(Size::px(140.))
                                .padding(Gaps::new_all(6.))
                                .child(label().text(col.clone()).color(Color::WHITE).font_size(11.))
                                .into_element()
                        })),
                )
                .child(
                    VirtualScrollView::new({
                        let rows = rows.clone();
                        move |i, _| {
                            let row = &rows[i];
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
                    })
                    .length(rows.len())
                    .item_size(24.)
                    .expanded(),
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
