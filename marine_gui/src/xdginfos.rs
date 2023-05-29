use once_cell::sync::Lazy;

use desktopparse::DesktopEnvironment;

pub static DESKTOPS: Lazy<Vec<DesktopEnvironment>> = Lazy::new(desktopparse::get_all_desktop);
