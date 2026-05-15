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
        let mut radio = use_radio(AppChannel::Connections);
        let editing = radio.read().editing_connection;
        let initial = editing.and_then(|i| radio.read().connections.get(i).cloned());

        let (init_name, init_host, init_port, init_database, init_user, init_password) =
            match initial {
                Some(c) => (
                    c.name,
                    c.host,
                    c.port.to_string(),
                    c.database,
                    c.user,
                    c.password,
                ),
                None => (
                    String::new(),
                    "localhost".to_string(),
                    "5432".to_string(),
                    String::new(),
                    String::new(),
                    String::new(),
                ),
            };

        let mut name = use_state(move || init_name);
        let mut host = use_state(move || init_host);
        let mut port = use_state(move || init_port);
        let mut database = use_state(move || init_database);
        let mut user = use_state(move || init_user);
        let mut password = use_state(move || init_password);
        let connection_string = use_state(String::new);
        let mut error = use_state(|| None::<String>);

        let on_parse = move |_| match parse_postgres_url(&connection_string.read()) {
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
            Err(e) => error.set(Some(e)),
        };

        let on_save = move |_| {
            let config = ConnectionConfig {
                name: name.read().clone(),
                host: host.read().clone(),
                port: port.read().parse().unwrap_or(5432),
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
                    Err(e) => error.set(Some(format!("Validation failed: {e}"))),
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
                let mut state = radio.write();
                state.show_form = false;
                state.editing_connection = None;
            })
            .child(PopupTitle::new(title.to_string()))
            .child(
                PopupContent::new().child(
                    rect()
                        .spacing(12.)
                        .child(form_input("Name", name))
                        .child(connection_string_field(connection_string, on_parse))
                        .child(form_input("Host", host))
                        .child(form_input("Port", port))
                        .child(form_input("Database", database))
                        .child(form_input("User", user))
                        .child(form_input("Password", password))
                        .map(error.read().clone(), |el, err| {
                            let theme = use_theme();
                            let err_color = theme.read().colors.error;
                            el.child(label().text(err).color(err_color).font_size(12.))
                        }),
                ),
            )
            .child(
                PopupButtons::new()
                    .child(Button::new().on_press(on_save).filled().child("Save"))
                    .child(
                        Button::new()
                            .on_press(move |_| {
                                let mut state = radio.write();
                                state.show_form = false;
                                state.editing_connection = None;
                            })
                            .flat()
                            .child("Cancel"),
                    ),
            )
    }
}

fn form_input(field_label: &str, value: State<String>) -> Rect {
    rect()
        .spacing(4.)
        .child(field_caption(field_label))
        .child(Input::new(value).placeholder("").width(Size::fill()))
}

fn connection_string_field(
    connection_string: State<String>,
    on_parse: impl Into<EventHandler<Event<PressEventData>>>,
) -> Rect {
    rect()
        .spacing(4.)
        .child(field_caption("Connection String"))
        .child(
            rect()
                .horizontal()
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
        )
}

fn field_caption(text: &str) -> Label {
    let theme = use_theme();
    let color = theme.read().colors.text_secondary;
    label().text(text.to_string()).color(color).font_size(12.)
}
