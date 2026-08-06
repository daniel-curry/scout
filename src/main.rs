mod app;
mod config;
mod entry;
mod icon;
mod launcher;
mod search;
mod ui;

use std::rc::Rc;

use config::{Config, load_or_create};

fn main() -> std::io::Result<()> {
    let cfg: Config = load_or_create()?;
    app::run(Rc::new(cfg));
    Ok(())
}
