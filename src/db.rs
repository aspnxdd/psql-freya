use crate::models::{ConnectionConfig, QueryResult, TableInfo};
use crate::value_parser::parse_value;
use std::sync::Arc;
use tokio_postgres::{Client, NoTls};

pub async fn connect(config: &ConnectionConfig) -> Result<Arc<Client>, String> {
    let conn_str = format!(
        "host={} port={} dbname={} user={} password={}",
        config.host, config.port, config.database, config.user, config.password
    );
    let (client, connection) = tokio_postgres::connect(&conn_str, NoTls)
        .await
        .map_err(|e| e.to_string())?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    Ok(Arc::new(client))
}

pub async fn fetch_schemas(client: &Client) -> Result<Vec<String>, String> {
    let rows = client
        .query(
            "SELECT schema_name FROM information_schema.schemata ORDER BY schema_name",
            &[],
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(rows.iter().map(|r| r.get::<_, String>(0)).collect())
}

pub async fn fetch_tables(client: &Client, schema: &str) -> Result<Vec<TableInfo>, String> {
    let rows = client
        .query(
            "SELECT table_schema, table_name FROM information_schema.tables WHERE table_schema = $1 ORDER BY table_name",
            &[&schema],
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(rows
        .iter()
        .map(|r| TableInfo {
            schema: r.get(0),
            name: r.get(1),
        })
        .collect())
}

pub async fn run_query(client: &Client, sql: &str) -> Result<QueryResult, String> {
    println!("Running query: {}", sql);
    let limited_sql = if sql.to_lowercase().contains("limit") {
        sql.to_string()
    } else {
        format!("{} LIMIT 1000", sql)
    }; 
     // select * from "public"."accounts"

    let stmt = client
        .prepare(&limited_sql)
        .await
        .map_err(|e| e.to_string())?;

    let columns: Vec<String> = stmt
        .columns()
        .iter()
        .map(|c| c.name().to_string())
        .collect();

    let rows = client.query(&stmt, &[]).await.map_err(|e| e.to_string())?;

    let mut data = Vec::new();
    for row in rows {
        let mut row_data = Vec::new();
        for (i, col) in stmt.columns().iter().enumerate() {
            let value = parse_value(&row, i, col.type_());
            row_data.push(value);
        }
        data.push(row_data);
    }

    Ok(QueryResult {
        columns,
        rows: data,
    })
}
