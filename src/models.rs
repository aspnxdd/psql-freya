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

#[derive(Clone, Default)]
pub struct AppState {
    pub connections: Vec<ConnectionConfig>,
    pub selected_connection: Option<usize>,
    pub client: Option<Arc<Client>>,
    pub schemas: Vec<String>,
    pub selected_schema: Option<String>,
    pub tables: Vec<TableInfo>,
    pub query_text: String,
    pub query_results: Option<QueryResult>,
    pub error_message: Option<String>,
    pub show_form: bool,
    pub editing_connection: Option<usize>,
}

#[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
pub enum AppChannel {
    DbMeta,
    QueryResults,
    Ui,
}

impl freya::radio::RadioChannel<AppState> for AppChannel {}
