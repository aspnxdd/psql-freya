use crate::db::connect;
use crate::models::AppChannel;
use freya::prelude::*;
use freya::radio::*;

#[derive(PartialEq)]
pub struct Sidebar;

impl Component for Sidebar {
    fn render(&self) -> impl IntoElement {
        let mut radio = use_radio(AppChannel::Ui);
        let connections = radio.read().connections.clone();

        rect()
            .width(Size::px(260.))
            .height(Size::fill())
            .background((35, 35, 35))
            .direction(Direction::Vertical)
            .content(Content::Flex)
            .padding(Gaps::new_all(12.))
            .child(
                label()
                    .text("Connections")
                    .font_size(18.)
                    .color(Color::WHITE),
            )
            .child(
                rect()
                    .height(Size::px(1.))
                    .background((55, 55, 55))
                    .margin(Gaps::new(8., 0., 8., 0.)),
            )
            .child(
                rect()
                    .width(Size::fill())
                    .height(Size::flex(1.))
                    .direction(Direction::Vertical)
                    .children(connections.iter().enumerate().map(|(i, conn)| {
                        let is_selected = radio.read().selected_connection == Some(i);
                        rect()
                            .key(i)
                            .width(Size::fill())
                            .height(Size::px(40.))
                            .corner_radius(CornerRadius::new_all(6.))
                            .background(if is_selected {
                                (50, 50, 50)
                            } else {
                                (42, 42, 42)
                            })
                            .padding(Gaps::new(0., 8., 0., 8.))
                            .direction(Direction::Horizontal)
                            .main_align(Alignment::SpaceBetween)
                            .cross_align(Alignment::Center)
                            .child(label().text(conn.name.clone()).color(Color::WHITE))
                            .on_press({
                                move |_| {
                                    let conn = radio.read().connections[i].clone();
                                    spawn(async move {
                                        match connect(&conn).await {
                                            Ok(client) => {
                                                let mut state = radio.write();
                                                state.selected_connection = Some(i);
                                                state.client = Some(client.clone());
                                                state.error_message = None;

                                                match crate::db::fetch_schemas(&client).await {
                                                    Ok(schemas) => {
                                                        state.schemas = schemas;
                                                        state.selected_schema = None;
                                                        state.tables.clear();
                                                    }
                                                    Err(e) => {
                                                        state.error_message = Some(format!(
                                                            "Failed to fetch schemas: {}",
                                                            e
                                                        ));
                                                    }
                                                }
                                            }
                                            Err(e) => {
                                                radio.write().error_message =
                                                    Some(format!("Connection failed: {}", e));
                                            }
                                        }
                                    });
                                }
                            })
                            .child(
                                rect()
                                    .direction(Direction::Horizontal)
                                    .child(
                                        label()
                                            .text("Edit")
                                            .font_size(11.)
                                            .color((255, 140, 0))
                                            .on_mouse_up({
                                                let mut radio = radio;
                                                move |_| {
                                                    radio.write().editing_connection = Some(i);
                                                    radio.write().show_form = true;
                                                }
                                            }),
                                    )
                                    .child(
                                        label()
                                            .text("Del")
                                            .font_size(11.)
                                            .color((255, 140, 0))
                                            .margin(Gaps::new(0., 0., 0., 8.))
                                            .on_mouse_up({
                                                move |_| {
                                                    radio.write().show_delete_confirm = Some(i);
                                                }
                                            }),
                                    ),
                            )
                            .into_element()
                    })),
            )
            .child(
                label()
                    .text("+ Add")
                    .color(Color::WHITE)
                    .font_size(14.)
                    .on_mouse_up(move |_| {
                        radio.write().show_form = true;
                        radio.write().editing_connection = None;
                    }),
            )
    }
}
