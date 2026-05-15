use crate::db::{fetch_tables, run_query};
use crate::models::{AppChannel, AppState, TableInfo};
use crate::theme::{divider, select};
use freya::prelude::*;
use freya::radio::*;

#[derive(PartialEq)]
pub struct SchemaTableList;

impl Component for SchemaTableList {
    fn render(&self) -> impl IntoElement {
        let radio = use_radio(AppChannel::DbMeta);
        let colors = use_theme().read().colors.clone();

        let schemas = radio.read().schemas.clone();
        let selected_schema = radio.read().selected_schema.clone();
        let tables = radio.read().tables.clone();
        let selected_table = radio.read().selected_table.clone();

        rect()
            .width(Size::px(220.))
            .height(Size::fill())
            .background(colors.surface_secondary)
            .padding(Gaps::new_all(12.))
            .child(label().text("Schemas").font_size(16.).theme_color())
            .child(divider())
            .child(rect().height(Size::px(150.)).spacing(4.).children(
                schemas.iter().enumerate().map(|(i, schema)| {
                    schema_item(
                        i,
                        schema,
                        selected_schema.as_deref() == Some(schema.as_str()),
                        radio,
                    )
                    .into_element()
                }),
            ))
            .child(divider())
            .child(label().text("Tables").font_size(16.).theme_color())
            .child(divider())
            .child(
                rect()
                    .expanded()
                    .spacing(4.)
                    .children(tables.iter().enumerate().map(|(i, table)| {
                        table_item(i, table, selected_table.as_ref() == Some(table), radio)
                            .into_element()
                    })),
            )
    }
}

fn schema_item(
    index: usize,
    schema: &str,
    is_selected: bool,
    mut radio: Radio<AppState, AppChannel>,
) -> SideBarItem {
    let schema = schema.to_string();
    let on_select = {
        let schema = schema.clone();
        move |_| {
            let Some(client) = radio.read().client.clone() else {
                return;
            };
            let schema = schema.clone();
            spawn(async move {
                match fetch_tables(&client, &schema).await {
                    Ok(tables) => {
                        let mut state = radio.write();
                        state.selected_schema = Some(schema);
                        state.tables = tables;
                        state.selected_table = None;
                    }
                    Err(e) => {
                        radio.write_channel(AppChannel::Query).error_message =
                            Some(format!("Failed to fetch tables: {e}"));
                    }
                }
            });
        }
    };

    select(
        SideBarItem::new()
            .key(index)
            .on_press(on_select)
            .child(label().text(schema).font_size(12.)),
        is_selected,
    )
}

fn table_item(
    index: usize,
    table: &TableInfo,
    is_selected: bool,
    mut radio: Radio<AppState, AppChannel>,
) -> SideBarItem {
    let table = table.clone();
    let name = table.name.clone();
    let on_select = {
        let table = table.clone();
        move |_| {
            let Some(client) = radio.read().client.clone() else {
                return;
            };
            let sql = format!(r#"SELECT * FROM "{}"."{}""#, table.schema, table.name);
            let table = table.clone();
            spawn(async move {
                match run_query(&client, &sql).await {
                    Ok(result) => {
                        radio.write().selected_table = Some(table);
                        let mut q = radio.write_channel(AppChannel::Query);
                        q.query_text = sql;
                        q.query_results = Some(result);
                        q.error_message = None;
                    }
                    Err(e) => {
                        radio.write_channel(AppChannel::Query).error_message =
                            Some(format!("Query failed: {e}"));
                    }
                }
            });
        }
    };

    select(
        SideBarItem::new()
            .key(index)
            .on_press(on_select)
            .child(label().text(name).font_size(12.)),
        is_selected,
    )
}
