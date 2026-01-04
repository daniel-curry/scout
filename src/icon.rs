use gtk::IconLookupFlags;
use gdk::gdk_pixbuf::Pixbuf;
use gio::AppInfo;
use gio::prelude::AppInfoExt;
use gtk::prelude::{IconThemeExt, ImageExt};

use crate::config::ICON_SIZE;

fn get_icon(app: &AppInfo) -> Option<Pixbuf> {
    let icon = app.icon()?;
    let icon_theme = gtk::IconTheme::default()?;
    let icon_info = icon_theme.lookup_by_gicon(
        &icon,
        ICON_SIZE,
        IconLookupFlags::FORCE_SIZE,
    )?;

    icon_info.load_icon().ok()
}

pub fn create_app_icon_widget(app: &AppInfo) -> gtk::Image {
    if let Some(pixbuf) = get_icon(app) {
        gtk::Image::from_pixbuf(Some(&pixbuf))
    } else {
        let image = gtk::Image::from_icon_name(
            Some("application-x-executable"),
            gtk::IconSize::Invalid,
        );
        image.set_pixel_size(ICON_SIZE);
        image
    }
}

pub fn create_generic_icon_widget(icon_name: &str) -> gtk::Image {
    let image = gtk::Image::from_icon_name(
        Some(icon_name),
        gtk::IconSize::Invalid,
    );
    image.set_pixel_size(ICON_SIZE);
    image
}
