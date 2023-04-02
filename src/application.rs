use gettextrs::gettext;
use log::{debug, info};

use gtk::subclass::prelude::*;
use gtk::{gio, glib};
use he::prelude::*;
use he::subclass::prelude::*;

use crate::config::{APP_ID, APP_PATH, NAME_SUFFIX, VERSION};
use crate::window::ApplicationWindow;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct Application {}

    #[glib::object_subclass]
    impl ObjectSubclass for Application {
        const NAME: &'static str = "Application";
        type Type = super::Application;
        type ParentType = he::Application;
    }

    impl ObjectImpl for Application {}
    impl ApplicationImpl for Application {
        fn activate(&self) {
            debug!("HeApplication<Application>::activate");
            self.parent_activate();
            let app = self.obj();

            app.active_window()
                .unwrap_or(ApplicationWindow::new(&app).upcast())
                .present();
        }

        fn startup(&self) {
            debug!("HeApplication<Application>::startup");
            self.parent_startup();
            let app = self.obj();

            // Set icons for shell
            gtk::Window::set_default_icon_name(APP_ID);

            app.setup_actions();
            app.setup_accels();
        }
    }

    impl GtkApplicationImpl for Application {}
    impl HeApplicationImpl for Application {}
}

glib::wrapper! {
    pub struct Application(ObjectSubclass<imp::Application>)
        @extends gio::Application, gtk::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl Application {
    fn setup_actions(&self) {
        // Quit
        let action_quit = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| {
                // This is needed to trigger the delete event and saving the window state
                app.active_window()
                    .expect("Expected an Active Window")
                    .close();
                app.quit();
            })
            .build();

        // About
        let action_about = gio::ActionEntry::builder("about")
            .activate(|app: &Self, _, _| {
                app.show_about_dialog();
            })
            .build();
        self.add_action_entries([action_quit, action_about]);
    }

    // Sets up keyboard shortcuts
    fn setup_accels(&self) {
        self.set_accels_for_action("app.quit", &["<Control>q"]);
        self.set_accels_for_action("window.close", &["<Control>w"]);
    }

    fn show_about_dialog(&self) {
        he::AboutWindow::new(
            &self.active_window().expect("Expected an Active Window"),
            &(gettext("Rust Template") + NAME_SUFFIX),
            APP_ID,
            VERSION,
            APP_ID,
            Some("https://weblate.fyralabs.com/addons/tauOS/rust-template/"),
            Some("https://github.com/tau-OS/rust-template/issues"),
            Some("https://github.com/tau-OS/rust-template"),
            &vec!["Fyra Labs", &gettext("translator-credits")],
            &vec!["Fyra Labs"],
            2023,
            he::AboutWindowLicenses::Gplv3,
            he::Colors::Purple,
        )
        .present();
    }

    pub fn run(&self) -> glib::ExitCode {
        info!("Rust Template ({})", APP_ID);
        info!("Version: {} {}", VERSION, NAME_SUFFIX);

        ApplicationExtManual::run(self)
    }
}

impl Default for Application {
    fn default() -> Self {
        glib::Object::builder()
            .property("application-id", APP_ID)
            .property("resource-base-path", APP_PATH)
            .build()
    }
}
