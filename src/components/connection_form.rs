use crate::config::save_config;
use crate::connection_string::{parse_postgres_url, random_connection_name};
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

        let mut name = use_state(|| {
            editing
                .and_then(|i| radio.read().connections.get(i).cloned())
                .map(|c| c.name)
                .unwrap_or_default()
        });
        let mut host = use_state(|| {
            editing
                .and_then(|i| radio.read().connections.get(i).cloned())
                .map(|c| c.host)
                .unwrap_or_else(|| "localhost".to_string())
        });
        let mut port = use_state(|| {
            editing
                .and_then(|i| radio.read().connections.get(i).cloned())
                .map(|c| c.port.to_string())
                .unwrap_or_else(|| "5432".to_string())
        });
        let mut database = use_state(|| {
            editing
                .and_then(|i| radio.read().connections.get(i).cloned())
                .map(|c| c.database)
                .unwrap_or_default()
        });
        let mut user = use_state(|| {
            editing
                .and_then(|i| radio.read().connections.get(i).cloned())
                .map(|c| c.user)
                .unwrap_or_default()
        });
        let mut password = use_state(|| {
            editing
                .and_then(|i| radio.read().connections.get(i).cloned())
                .map(|c| c.password)
                .unwrap_or_default()
        });
        let connection_string = use_state(|| String::new());
        let mut error = use_state(|| None::<String>);

        let on_parse = move |_| {
            match parse_postgres_url(&connection_string.read()) {
                Ok(parsed) => {
                    if name.read().trim().is_empty() {
                        name.set(random_connection_name());
                    }
                    host.set(parsed.host);
                    port.set(parsed.port.to_string());
                    database.set(parsed.database);
                    user.set(parsed.user);
                    password.set(parsed.password);
                    error.set(None);
                }
                Err(e) => {
                    error.set(Some(e));
                }
            }
        };

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

        let title = if editing.is_some() {
            "Edit Connection"
        } else {
            "Add Connection"
        };

        Popup::new()
            .show(true)
            .width(Size::px(400.))
            .on_close_request(move |_| {
                radio.write().show_form = false;
                radio.write().editing_connection = None;
            })
            .child(PopupTitle::new(title.to_string()))
            .child(
                PopupContent::new().child(
                    rect()
                        .direction(Direction::Vertical)
                        .spacing(12.)
                        .child(form_input("Name".to_string(), name))
                        .child(
                            rect()
                                .direction(Direction::Vertical)
                                .spacing(4.)
                                .child(
                                    label()
                                        .text("Connection String")
                                        .color((180, 180, 180))
                                        .font_size(12.),
                                )
                                .child(
                                    rect()
                                        .direction(Direction::Horizontal)
                                        .spacing(8.)
                                        .height(Size::px(32.))
                                        .content(Content::Flex)
                                        .child(
                                            Input::new(connection_string)
                                                .placeholder("postgresql://user:pass@host:port/db")
                                                .width(Size::flex(1.))
                                                .expanded(),
                                        )
                                        .child(
                                            Button::new()
                                                .height(Size::fill())
                                                .on_press(on_parse)
                                                .filled()
                                                .child("Parse"),
                                        ),
                                ),
                        )
                        .child(form_input("Host".to_string(), host))
                        .child(form_input("Port".to_string(), port))
                        .child(form_input("Database".to_string(), database))
                        .child(form_input("User".to_string(), user))
                        .child(form_input("Password".to_string(), password))
                        .map(error.read().clone(), |el, err| {
                            el.child(label().text(err).color((255, 80, 80)).font_size(12.))
                        }),
                ),
            )
            .child(
                PopupButtons::new()
                    .child(Button::new().on_press(on_save).filled().child("Save"))
                    .child(
                        Button::new()
                            .on_press(move |_| {
                                radio.write().show_form = false;
                                radio.write().editing_connection = None;
                            })
                            .flat()
                            .child("Cancel"),
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
