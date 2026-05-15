use crate::db::{fetch_tables, run_query};
use crate::models::AppChannel;
use freya::prelude::*;
use freya::radio::*;

#[derive(PartialEq)]
pub struct SchemaTableList;

impl Component for SchemaTableList {
    fn render(&self) -> impl IntoElement {
        let radio = use_radio(AppChannel::Ui);
        let schemas = radio.read().schemas.clone();
        let selected_schema = radio.read().selected_schema.clone();
        let tables = radio.read().tables.clone();
        let selected_table = radio.read().selected_table.clone();

        rect()
            .width(Size::px(220.))
            .height(Size::fill())
            .background((40, 40, 40))
            .direction(Direction::Vertical)
            .padding(Gaps::new_all(12.))
            .child(label().text("Schemas").font_size(16.).color(Color::WHITE))
            .child(
                rect()
                    .height(Size::px(1.))
                    .background((55, 55, 55))
                    .margin(Gaps::new(8., 0., 8., 0.)),
            )
            .child(
                rect()
                    .height(Size::px(150.))
                    .direction(Direction::Vertical)
                    .children(schemas.iter().enumerate().map(|(i, schema)| {
                        let is_selected = selected_schema.as_ref() == Some(schema);
                        rect()
                            .key(i)
                            .height(Size::px(28.))
                            .corner_radius(CornerRadius::new_all(4.))
                            .background(if is_selected {
                                (50, 50, 50)
                            } else {
                                (40, 40, 40)
                            })
                            .padding(Gaps::new(0., 6., 0., 6.))
                            .main_align(Alignment::Center)
                            .child(
                                label()
                                    .text(schema.clone())
                                    .color(Color::WHITE)
                                    .font_size(12.),
                            )
                            .on_mouse_up({
                                let schema = schema.clone();
                                let mut radio = radio;
                                move |_| {
                                    if let Some(client) = radio.read().client.clone() {
                                        let schema = schema.clone();
                                        spawn(async move {
                                            match fetch_tables(&client, &schema).await {
                                                Ok(tables) => {
                                                    let mut state = radio.write();
                                                    state.selected_schema = Some(schema);
                                                    state.tables = tables;
                                                }
                                                Err(e) => {
                                                    radio.write().error_message = Some(format!(
                                                        "Failed to fetch tables: {}",
                                                        e
                                                    ));
                                                }
                                            }
                                        });
                                    }
                                }
                            })
                            .into_element()
                    })),
            )
            .child(
                rect()
                    .height(Size::px(1.))
                    .background((55, 55, 55))
                    .margin(Gaps::new(8., 0., 8., 0.)),
            )
            .child(label().text("Tables").font_size(16.).color(Color::WHITE))
            .child(
                rect()
                    .height(Size::px(1.))
                    .background((55, 55, 55))
                    .margin(Gaps::new(8., 0., 8., 0.)),
            )
            .child(rect().expanded().direction(Direction::Vertical).children(
                tables.iter().enumerate().map(|(i, table)| {
                    let is_selected = selected_table.as_ref() == Some(table);
                    rect()
                        .key(i)
                        .height(Size::px(28.))
                        .corner_radius(CornerRadius::new_all(4.))
                        .background(if is_selected {
                            (50, 50, 50)
                        } else {
                            (40, 40, 40)
                        })
                        .padding(Gaps::new(0., 6., 0., 6.))
                        .main_align(Alignment::Center)
                        .child(
                            label()
                                .text(table.name.clone())
                                .color(Color::WHITE)
                                .font_size(12.),
                        )
                        .on_mouse_up({
                            let table = table.clone();
                            let mut radio = radio;
                            move |_| {
                                if let Some(client) = radio.read().client.clone() {
                                    let sql = format!(
                                        r#"SELECT * FROM "{}"."{}""#,
                                        table.schema, table.name
                                    );
                                    let table = table.clone();
                                    spawn(async move {
                                        match run_query(&client, &sql).await {
                                            Ok(result) => {
                                                let mut state = radio.write();
                                                state.selected_table = Some(table);
                                                state.query_text = sql;
                                                state.query_results = Some(result);
                                                state.error_message = None;
                                            }
                                            Err(e) => {
                                                radio.write().error_message =
                                                    Some(format!("Query failed: {}", e));
                                            }
                                        }
                                    });
                                }
                            }
                        })
                        .into_element()
                }),
            ))
    }
}
