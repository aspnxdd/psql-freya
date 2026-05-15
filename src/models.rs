use crate::config::load_config;
use freya::radio::RadioChannel;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio_postgres::Client;

#[derive(Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectionConfig {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub user: String,
    pub password: String,
}

#[derive(Clone, PartialEq)]
pub struct TableInfo {
    pub schema: String,
    pub name: String,
}

#[derive(Clone, Default, PartialEq)]
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

pub struct AppState {
    pub connections: Vec<ConnectionConfig>,
    pub selected_connection: Option<usize>,
    pub show_form: bool,
    pub editing_connection: Option<usize>,
    pub show_delete_confirm: Option<usize>,

    pub client: Option<Arc<Client>>,
    pub schemas: Vec<String>,
    pub selected_schema: Option<String>,
    pub tables: Vec<TableInfo>,
    pub selected_table: Option<TableInfo>,

    pub query_text: String,
    pub query_results: Option<QueryResult>,
    pub error_message: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            connections: load_config(),
            selected_connection: None,
            show_form: false,
            editing_connection: None,
            show_delete_confirm: None,
            client: None,
            schemas: Vec::new(),
            selected_schema: None,
            tables: Vec::new(),
            selected_table: None,
            query_text: String::new(),
            query_results: None,
            error_message: None,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
pub enum AppChannel {
    Connections,
    DbMeta,
    Query,
}

impl RadioChannel<AppState> for AppChannel {}
