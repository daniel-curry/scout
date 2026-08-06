use crate::config::Config;
use gdk::gdk_pixbuf::Pixbuf;
use gio::{AppInfo, prelude::AppInfoExt};
use gtk::{IconLookupFlags, IconTheme, Image, prelude::IconThemeExt};

fn get_icon(app: &AppInfo, cfg: &Config) -> Option<Pixbuf> {
    IconTheme::default()?
        .lookup_by_gicon(&app.icon()?, cfg.icon_size, IconLookupFlags::FORCE_SIZE)?
        .load_icon()
        .ok()
}

pub fn create_app_icon_widget(app: &AppInfo, cfg: Config) -> Image {
    match get_icon(app, &cfg) {
        Some(pixbuf) => Image::from_pixbuf(Some(&pixbuf)),
        None => Image::builder()
            .icon_name("application-x-executable")
            .pixel_size(cfg.icon_size)
            .build(),
    }
}

pub fn create_generic_icon_widget(icon_name: &str, cfg: Config) -> Image {
    Image::builder()
        .icon_name(icon_name)
        .pixel_size(cfg.icon_size)
        .build()
}
