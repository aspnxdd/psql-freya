use crate::components::{
    connection_form::ConnectionForm, delete_confirm_dialog::DeleteConfirmDialog,
    main_area::MainArea, schema_table_list::SchemaTableList, sidebar::Sidebar,
};
use crate::models::{AppChannel, AppState};
use crate::theme::brand_theme;
use freya::prelude::*;
use freya::radio::*;

pub fn app() -> impl IntoElement {
    use_init_theme(brand_theme);
    use_init_radio_station::<AppState, AppChannel>(AppState::new);

    let radio = use_radio(AppChannel::Connections);
    let show_form = radio.read().show_form;
    let show_delete_confirm = radio.read().show_delete_confirm.is_some();

    rect()
        .width(Size::fill())
        .height(Size::fill())
        .horizontal()
        .theme_background()
        .child(Sidebar {})
        .child(SchemaTableList {})
        .child(MainArea {})
        .maybe(show_form, |el| el.child(ConnectionForm {}))
        .maybe(show_delete_confirm, |el| el.child(DeleteConfirmDialog {}))
}
