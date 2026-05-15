mod app;
mod components;
mod config;
mod db;
mod models;

use freya::prelude::*;

fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let _guard = rt.enter();

    launch(
        LaunchConfig::new().with_window(
            WindowConfig::new(app::app)
                .with_title("psql-freya")
                .with_size(1200., 800.),
        ),
    );
}
