use crate::config::save_config;
use crate::models::AppChannel;
use freya::prelude::*;
use freya::radio::*;

#[derive(PartialEq)]
pub struct DeleteConfirmDialog;

impl Component for DeleteConfirmDialog {
    fn render(&self) -> impl IntoElement {
        let mut radio = use_radio(AppChannel::Connections);
        let theme = use_theme();
        let text_secondary = theme.read().colors.text_secondary;

        let target = radio
            .read()
            .show_delete_confirm
            .and_then(|i| radio.read().connections.get(i).map(|c| (i, c.name.clone())));

        let title = match &target {
            Some((_, name)) => format!("Delete '{name}'"),
            None => "Delete Connection".to_string(),
        };

        let on_delete = move |_| {
            let Some((i, _)) = target.clone() else {
                return;
            };
            let was_active = radio.read().selected_connection == Some(i);

            {
                let mut state = radio.write();
                state.connections.remove(i);
                save_config(&state.connections);
                state.show_delete_confirm = None;
                state.selected_connection = match state.selected_connection {
                    Some(s) if s == i => None,
                    Some(s) if s > i => Some(s - 1),
                    other => other,
                };
            }

            if was_active {
                let mut db = radio.write_channel(AppChannel::DbMeta);
                db.client = None;
                db.schemas.clear();
                db.selected_schema = None;
                db.tables.clear();
                db.selected_table = None;

                let mut q = radio.write_channel(AppChannel::Query);
                q.query_results = None;
                q.error_message = None;
            }
        };

        Popup::new()
            .show(true)
            .width(Size::px(360.))
            .on_close_request(move |_| {
                radio.write().show_delete_confirm = None;
            })
            .child(PopupTitle::new(title))
            .child(
                PopupContent::new().child(
                    label()
                        .text("Are you sure you want to delete this connection? This action cannot be undone.")
                        .color(text_secondary)
                        .font_size(14.),
                ),
            )
            .child(
                PopupButtons::new()
                    .child(Button::new().on_press(on_delete).filled().child("Delete"))
                    .child(
                        Button::new()
                            .on_press(move |_| {
                                radio.write().show_delete_confirm = None;
                            })
                            .flat()
                            .child("Cancel"),
                    ),
            )
    }
}
