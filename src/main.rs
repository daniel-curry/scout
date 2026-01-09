mod ui;
mod search;
mod launcher;
mod app;
mod config;
mod entry;
mod icon;

use std::rc::Rc;
use config::{Config, load_or_create};

fn main() -> std::io::Result<()> {
    let cfg: Rc<Config> = Rc::new(load_or_create()?);
    app::run(cfg);
    Ok(())
}
