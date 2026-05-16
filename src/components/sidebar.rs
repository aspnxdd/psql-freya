use crate::db::{connect, fetch_schemas};
use crate::models::{AppChannel, AppState};
use crate::theme::{divider, select};
use freya::icons::lucide;
use freya::prelude::*;
use freya::radio::*;

#[derive(PartialEq)]
pub struct Sidebar;

impl Component for Sidebar {
    fn render(&self) -> impl IntoElement {
        let mut radio = use_radio(AppChannel::Connections);
        let colors = use_theme().read().colors.clone();

        let conns = radio.read().connections.clone();
        let selected = radio.read().selected_connection;

        rect()
            .width(Size::px(260.))
            .height(Size::fill())
            .background(colors.surface_primary)
            .content(Content::Flex)
            .padding(Gaps::new_all(12.))
            .child(label().text("Connections").font_size(18.).theme_color())
            .child(divider())
            .child(
                rect()
                    .width(Size::fill())
                    .height(Size::flex(1.))
                    .spacing(4.)
                    .children(conns.iter().enumerate().map(|(i, conn)| {
                        connection_item(i, &conn.name, selected == Some(i), &colors, radio)
                            .into_element()
                    })),
            )
            .child(
                Button::new()
                    .on_press(move |_| {
                        let mut state = radio.write();
                        state.show_form = true;
                        state.editing_connection = None;
                    })
                    .filled()
                    .expanded()
                    .corner_radius(CornerRadius::new_all(99.))
                    .child(
                        svg(lucide::plus())
                            .theme_color()
                            .width(Size::px(16.))
                            .height(Size::px(16.)),
                    ),
            )
    }
}

fn connection_item(
    index: usize,
    name: &str,
    is_selected: bool,
    colors: &ColorsSheet,
    mut radio: Radio<AppState, AppChannel>,
) -> SideBarItem {
    let on_select = move |_| {
        let conn = radio.read().connections[index].clone();
        spawn(async move {
            match connect(&conn).await {
                Ok(client) => {
                    radio.write().selected_connection = Some(index);
                    radio.write_channel(AppChannel::Query).error_message = None;
                    match fetch_schemas(&client).await {
                        Ok(schemas) => {
                            let mut state = radio.write_channel(AppChannel::DbMeta);
                            state.client = Some(client);
                            state.schemas = schemas;
                            state.selected_schema = None;
                            state.tables.clear();
                            state.selected_table = None;
                        }
                        Err(e) => {
                            radio.write_channel(AppChannel::DbMeta).client = Some(client);
                            radio.write_channel(AppChannel::Query).error_message =
                                Some(format!("Failed to fetch schemas: {e}"));
                        }
                    }
                }
                Err(e) => {
                    radio.write_channel(AppChannel::Query).error_message =
                        Some(format!("Connection failed: {e}"));
                }
            }
        });
    };

    let on_edit = move |e: Event<PressEventData>| {
        e.stop_propagation();
        let mut state = radio.write();
        state.editing_connection = Some(index);
        state.show_form = true;
    };

    let on_delete = move |e: Event<PressEventData>| {
        e.stop_propagation();
        radio.write().show_delete_confirm = Some(index);
    };

    select(
        SideBarItem::new().key(index).on_press(on_select).child(
            rect()
                .horizontal()
                .width(Size::fill())
                .main_align(Alignment::SpaceBetween)
                .cross_align(Alignment::Center)
                .child(label().text(name.to_string()))
                .child(
                    rect()
                        .horizontal()
                        .spacing(8.)
                        .cross_align(Alignment::Center)
                        .child(icon_button(lucide::pencil(), colors.primary, on_edit))
                        .child(icon_button(lucide::trash_2(), colors.primary, on_delete)),
                ),
        ),
        is_selected,
    )
}

fn icon_button(
    bytes: impl Into<SvgBytes>,
    color: Color,
    on_press: impl Into<EventHandler<Event<PressEventData>>>,
) -> Button {
    Button::new()
        .flat()
        .compact()
        .corner_radius(CornerRadius::new_all(99.))
        .padding(Gaps::new_all(4.))
        .on_press(on_press)
        .child(
            svg(bytes)
                .color(color)
                .width(Size::px(14.))
                .height(Size::px(14.)),
        )
}
