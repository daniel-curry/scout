mod ui;
mod search;
mod launcher;
mod app;
mod config;
mod entry;
mod icon;

use std::sync::Arc;
use config::{Config, load_or_create};

fn main() -> std::io::Result<()> {
    let cfg: Arc<Config> = Arc::new(load_or_create()?);
    app::run(cfg);
    Ok(())
}
