use crate::components::{
    connection_form::ConnectionForm, delete_confirm_dialog::DeleteConfirmDialog,
    main_area::MainArea, schema_table_list::SchemaTableList, sidebar::Sidebar,
};
use crate::config::load_config;
use crate::models::{AppChannel, AppState};
use freya::prelude::*;
use freya::radio::*;

pub fn app() -> impl IntoElement {
    let mut theme = dark_theme();
    theme.colors.primary = Color::from_rgb(255, 140, 0);
    theme.colors.background = Color::from_rgb(22, 22, 22);
    use_init_theme(|| theme);
    use_init_radio_station::<AppState, AppChannel>(AppState::default);

    let mut radio = use_radio(AppChannel::Ui);

    use_hook(move || {
        let connections = load_config();
        radio.write().connections = connections;
    });

    let show_form = radio.read().show_form;
    let show_delete_confirm = radio.read().show_delete_confirm.is_some();

    rect()
        .width(Size::fill())
        .height(Size::fill())
        .direction(Direction::Horizontal)
        .theme_background()
        .child(Sidebar {})
        .child(SchemaTableList {})
        .child(MainArea {})
        .maybe(show_form, |el| el.child(ConnectionForm {}))
        .maybe(show_delete_confirm, |el| el.child(DeleteConfirmDialog {}))
}
