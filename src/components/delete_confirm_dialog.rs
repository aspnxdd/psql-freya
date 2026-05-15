use crate::config::save_config;
use crate::models::AppChannel;
use freya::prelude::*;
use freya::radio::*;

#[derive(PartialEq)]
pub struct DeleteConfirmDialog;

impl Component for DeleteConfirmDialog {
    fn render(&self) -> impl IntoElement {
        let mut radio = use_radio(AppChannel::Ui);
        let index = radio.read().show_delete_confirm;

        let (title, can_delete) = if let Some(i) = index {
            let conn_name = radio.read().connections.get(i).map(|c| c.name.clone());
            if let Some(name) = conn_name {
                (
                    format!("Delete '{}'", name),
                    Some((i, name)),
                )
            } else {
                ("Delete Connection".to_string(), None)
            }
        } else {
            ("Delete Connection".to_string(), None)
        };

        let on_delete = move |_| {
            if let Some((i, _)) = can_delete {
                let mut state = radio.write();
                state.connections.remove(i);

                if state.selected_connection == Some(i) {
                    state.selected_connection = None;
                    state.client = None;
                    state.schemas.clear();
                    state.tables.clear();
                    state.query_results = None;
                    state.error_message = None;
                } else if let Some(selected) = state.selected_connection {
                    if selected > i {
                        state.selected_connection = Some(selected - 1);
                    }
                }

                save_config(&state.connections);
                state.show_delete_confirm = None;
            }
        };

        let on_cancel = move |_| {
            radio.write().show_delete_confirm = None;
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
                    rect()
                        .direction(Direction::Vertical)
                        .spacing(8.)
                        .child(
                            label()
                                .text("Are you sure you want to delete this connection? This action cannot be undone.")
                                .color((180, 180, 180))
                                .font_size(14.),
                        ),
                ),
            )
            .child(
                PopupButtons::new()
                    .child(Button::new().on_press(on_delete).filled().child("Delete"))
                    .child(Button::new().on_press(on_cancel).flat().child("Cancel")),
            )
    }
}
