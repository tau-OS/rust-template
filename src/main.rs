mod application;
#[rustfmt::skip]
mod config;
mod window;

use gettextrs::{gettext, LocaleCategory};
use gtk::gio::ffi::{g_resources_register, GResource};
use gtk::glib;

use self::application::Application;
use self::config::{GETTEXT_PACKAGE, LOCALEDIR};

#[link(name = "resources")]
extern "C" {
    fn resources_get_resource() -> *mut GResource;
}

pub fn handle_resources() {
    unsafe { g_resources_register(resources_get_resource()) }
}

fn main() -> glib::ExitCode {
    // Initialize logger
    pretty_env_logger::init();

    // Prepare i18n
    gettextrs::setlocale(LocaleCategory::LcAll, "");
    gettextrs::bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    gettextrs::textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

    glib::set_application_name(&gettext("Rust Template"));

    handle_resources();

    let app = Application::default();
    app.run()
}
