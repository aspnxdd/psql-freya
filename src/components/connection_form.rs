use crate::config::save_config;
use crate::db::connect;
use crate::models::{AppChannel, ConnectionConfig};
use freya::prelude::*;
use freya::radio::*;

#[derive(PartialEq)]
pub struct ConnectionForm;

impl Component for ConnectionForm {
    fn render(&self) -> impl IntoElement {
        let mut radio = use_radio(AppChannel::Ui);
        let editing = radio.read().editing_connection;

        let name = use_state(|| {
            editing
                .and_then(|i| radio.read().connections.get(i).cloned())
                .map(|c| c.name)
                .unwrap_or_default()
        });
        let host = use_state(|| {
            editing
                .and_then(|i| radio.read().connections.get(i).cloned())
                .map(|c| c.host)
                .unwrap_or_else(|| "localhost".to_string())
        });
        let port = use_state(|| {
            editing
                .and_then(|i| radio.read().connections.get(i).cloned())
                .map(|c| c.port.to_string())
                .unwrap_or_else(|| "5432".to_string())
        });
        let database = use_state(|| {
            editing
                .and_then(|i| radio.read().connections.get(i).cloned())
                .map(|c| c.database)
                .unwrap_or_default()
        });
        let user = use_state(|| {
            editing
                .and_then(|i| radio.read().connections.get(i).cloned())
                .map(|c| c.user)
                .unwrap_or_default()
        });
        let password = use_state(|| {
            editing
                .and_then(|i| radio.read().connections.get(i).cloned())
                .map(|c| c.password)
                .unwrap_or_default()
        });
        let mut error = use_state(|| None::<String>);

        let on_save = move |_| {
            let port_num: u16 = port.read().parse().unwrap_or(5432);
            let config = ConnectionConfig {
                name: name.read().clone(),
                host: host.read().clone(),
                port: port_num,
                database: database.read().clone(),
                user: user.read().clone(),
                password: password.read().clone(),
            };

            spawn(async move {
                match connect(&config).await {
                    Ok(_) => {
                        let mut state = radio.write();
                        if let Some(i) = editing {
                            state.connections[i] = config;
                        } else {
                            state.connections.push(config);
                        }
                        save_config(&state.connections);
                        state.show_form = false;
                        state.editing_connection = None;
                    }
                    Err(e) => {
                        error.set(Some(format!("Validation failed: {}", e)));
                    }
                }
            });
        };

        let on_cancel = move |_| {
            radio.write().show_form = false;
            radio.write().editing_connection = None;
        };

        rect()
            .width(Size::fill())
            .height(Size::fill())
            .background((0, 0, 0, 150))
            .center()
            .child(
                rect()
                    .width(Size::px(400.))
                    .background((45, 45, 55))
                    .corner_radius(CornerRadius::new_all(12.))
                    .padding(Gaps::new_all(24.))
                    .direction(Direction::Vertical)
                    .spacing(12.)
                    .child(
                        label()
                            .text(if editing.is_some() {
                                "Edit Connection"
                            } else {
                                "Add Connection"
                            })
                            .font_size(18.)
                            .color(Color::WHITE),
                    )
                    .child(form_input("Name".to_string(), name))
                    .child(form_input("Host".to_string(), host))
                    .child(form_input("Port".to_string(), port))
                    .child(form_input("Database".to_string(), database))
                    .child(form_input("User".to_string(), user))
                    .child(form_input("Password".to_string(), password))
                    .map(error.read().clone(), |el, err| {
                        el.child(label().text(err).color((255, 80, 80)).font_size(12.))
                    })
                    .child(
                        rect()
                            .direction(Direction::Horizontal)
                            .spacing(12.)
                            .child(Button::new().on_press(on_save).filled().child("Save"))
                            .child(Button::new().on_press(on_cancel).flat().child("Cancel")),
                    ),
            )
    }
}

fn form_input(label_text: String, value: State<String>) -> impl IntoElement {
    rect()
        .direction(Direction::Vertical)
        .spacing(4.)
        .child(
            label()
                .text(label_text)
                .color((180, 180, 180))
                .font_size(12.),
        )
        .child(Input::new(value).placeholder("").width(Size::fill()))
}
