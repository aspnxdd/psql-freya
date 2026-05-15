# Agent Guide — psql-freya

This document helps agents and developers navigate and modify the `psql-freya` codebase effectively.

## Project Type

- **Language:** Rust
- **GUI Framework:** [Freya](https://freyaui.dev) — cross-platform, native, declarative, Skia-based.
- **App Type:** Single binary with a `main.rs` entry point.
- **Async Runtime:** Tokio (multi-thread). A Tokio runtime guard is entered in `main.rs` before launching Freya so `tokio-postgres` can run.

## Entry Point & Launch Flow

1. `main.rs` builds a Tokio runtime, enters it, and calls `freya::prelude::launch()`.
2. `app.rs` is the root Freya component. It:
   - Initializes a custom dark theme (`use_init_theme`).
   - Initializes the Freya Radio station (`use_init_radio_station::<AppState, AppChannel>`).
   - Loads persisted connections via `use_hook` + `load_config()`.
   - Lays out the three main panes and conditionally renders popups.

## State Management

We use **Freya Radio** for global, reactive state.

- **State:** `AppState` in `models.rs` holds connections, selected connection, `tokio-postgres` client, schemas, tables, query text/results, error message, and UI flags (`show_form`, `show_delete_confirm`).
- **Channels:** `AppChannel` enum with three variants:
  - `DbMeta` — schema/table metadata changes
  - `QueryResults` — query execution results
  - `Ui` — general UI state (popups, selections)

Components subscribe to specific channels with `use_radio(AppChannel::X)`. Only subscribers to a channel re-render when that channel is written to, enabling surgical updates.

## Theming

The app uses a custom dark theme defined in `app.rs`:

- `theme.colors.primary = Color::from_rgb(255, 140, 0)` (orange)
- `theme.colors.background = Color::from_rgb(22, 22, 22)`

Use Freya’s theme helpers (e.g., `.theme_background()`) where possible instead of hardcoding colors in new components.

## Component Conventions

- **Stateful UI:** Use struct components with `#[derive(PartialEq)]` and implement `Component`.
- **Stateless UI:** Use plain functions returning `impl IntoElement`.
- **Registration:** Add new modules to `src/components/mod.rs`.
- **Builder Pattern:** Chain element methods directly. Do not store elements in variables to mutate later.
- **Hooks:**
  - Only call hooks at the top level of `render`.
  - Never call hooks inside conditionals, loops, closures, or async blocks.
  - Capture hook values in `move` closures for event handlers.
- **Lists:** Always use `.key(id)` on elements inside dynamic iterators (`VirtualScrollView`, `.children(...)`).
- **Conditional Rendering:** Prefer a single `.maybe(condition, |el| ...)` wrapping multiple children rather than repeated `.maybe_child(...)` calls.

## Async & Database Layer

- All PostgreSQL I/O lives in `db.rs`.
- Freya’s `spawn()` (not `tokio::spawn`) must be used for async tasks that write to component state.
- `tokio::spawn` is only used internally in `db.rs` to drive the raw `tokio-postgres` connection future.
- Keep DB logic out of components; components should only call `db::` functions inside `spawn` blocks.

## Config & Persistence

- `config.rs` handles read/write of `~/.psql-freya/config.json`.
- `ConnectionConfig` is serialized with `serde`.
- Call `save_config` after any mutation to the connections list.

## Value Formatting

- `value_parser.rs` maps `tokio-postgres` `Type`s to human-readable strings.
- Supports scalars, arrays, JSON, timestamps, UUID, numeric, bytea (hex), and more.
- Extend `parse_scalar` or `parse_array` when adding support for new PostgreSQL types.

## Adding a New Feature — Checklist

1. **Model changes** — Add fields to `AppState` or new types in `models.rs` if needed.
2. **DB changes** — Add queries or helpers in `db.rs`.
3. **UI changes** — Create or modify components in `src/components/`.
4. **Config changes** — Update `ConnectionConfig` or persistence in `config.rs` if the data should survive restarts.
5. **Theme changes** — Use theme helpers or extend the custom theme in `app.rs`.

## Freya Reference

- **Local skill:** `skills/freya/SKILL.md` — Best practices, patterns, hooks, state management, theming, and feature guides.
- **Online docs:** https://freyaui.dev/llms.txt — Official feature list and links to API docs.

## Build Commands

```bash
cargo check       # Fast syntax/type check
cargo build       # Debug build
cargo run         # Run in development
cargo clippy      # Linting
cargo fmt         # Formatting
```
