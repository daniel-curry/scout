use std::sync::Arc;
use gtk::IconLookupFlags;
use gdk::gdk_pixbuf::Pixbuf;
use gio::AppInfo;
use gio::prelude::AppInfoExt;
use gtk::prelude::{IconThemeExt, ImageExt};
use crate::config::Config;

fn get_icon(app: &AppInfo, cfg: &Config) -> Option<Pixbuf> {
    let icon = app.icon()?;
    let icon_theme = gtk::IconTheme::default()?;
    let icon_info = icon_theme.lookup_by_gicon(
        &icon,
        cfg.icon_size,
        IconLookupFlags::FORCE_SIZE,
    )?;

    icon_info.load_icon().ok()
}

pub fn create_app_icon_widget(app: &AppInfo, cfg: Arc<Config>) -> gtk::Image {
    if let Some(pixbuf) = get_icon(app, &cfg) {
        gtk::Image::from_pixbuf(Some(&pixbuf))
    } else {
        let image = gtk::Image::from_icon_name(
            Some("application-x-executable"),
            gtk::IconSize::Invalid,
        );
        image.set_pixel_size(cfg.icon_size);
        image
    }
}

pub fn create_generic_icon_widget(icon_name: &str, cfg: Arc<Config>) -> gtk::Image {
    let image = gtk::Image::from_icon_name(
        Some(icon_name),
        gtk::IconSize::Invalid,
    );
    image.set_pixel_size(cfg.icon_size);
    image
}
