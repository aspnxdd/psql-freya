# psql-freya

A lightweight, native PostgreSQL GUI client built with [Freya](https://freyaui.dev) and Rust.

## Features

- **Connection Management** — Add, edit, and delete PostgreSQL connections with a persistent JSON config.
- **Connection String Parsing** — Paste a `postgresql://` URL to auto-fill host, port, database, user, and password.
- **Schema & Table Browser** — Explore schemas and tables in a two-level sidebar. Click a table to auto-run `SELECT *`.
- **SQL Query Editor** — Type queries in the main area and run them with a button or interaction.
- **Auto-Quoting** — Unquoted `schema.table` references in `FROM` and `JOIN` clauses are automatically double-quoted.
- **Result Grid** — Query results are rendered in a virtual-scrolling grid with dynamically sized columns.
- **Rich Type Support** — Displays arrays, JSON/JSONB, timestamps, UUIDs, numerics, bytea (hex), and more.
- **Dark UI** — Custom dark theme with an orange primary accent.

## UI Overview

The interface is split into three panes:

1. **Sidebar (left)** — List of saved connections. Select one to connect and load schemas.
2. **Schema / Table List (center)** — Browse schemas, then tables. Selecting a table runs a default query.
3. **Main Area (right)** — SQL input, error display, and results grid.

## Prerequisites

- [Rust](https://rustup.rs/) toolchain
- A running PostgreSQL server to connect to

## Build & Run

```bash
# Development
cargo run

# Release
cargo build --release
```

## Configuration

Connections are saved to `~/.psql-freya/config.json` and loaded automatically on startup.

## Project Structure

```
src/
  main.rs                 # Entry point — Tokio runtime + Freya launch
  app.rs                  # Root component — theme init, radio station, layout
  models.rs               # Shared state (AppState, ConnectionConfig, TableInfo, QueryResult)
  db.rs                   # Async PostgreSQL helpers (connect, fetch_schemas, fetch_tables, run_query)
  config.rs               # JSON config persistence
  connection_string.rs    # postgres:// URL parser
  value_parser.rs         # PostgreSQL type → display string mapping
  components/
    mod.rs                # Component exports
    sidebar.rs            # Connection list + connect action
    connection_form.rs    # Add / edit connection popup
    delete_confirm_dialog.rs  # Delete confirmation popup
    schema_table_list.rs  # Schema + table browser
    main_area.rs          # Query input, run button, results grid
```

## Dependencies

| Crate            | Purpose                                      |
|------------------|----------------------------------------------|
| `freya`          | Native GUI framework (reactive, Skia-based)  |
| `tokio-postgres` | Async PostgreSQL client                      |
| `serde` / `serde_json` | Config serialization                   |
| `chrono`         | Date/time parsing                            |
| `rust_decimal`   | `numeric` type support                       |
| `regex`          | SQL auto-quoting                             |
| `uuid`           | UUID display                                 |
| `url`            | Connection string parsing                    |
| `dirs`           | Home directory resolution                    |

## License

MIT
