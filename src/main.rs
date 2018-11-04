extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("About I <3 OS");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    window.connect_delete_event(move |win, _| {
        win.destroy();
        Inhibit(false)
    });

    //let button = gtk::Button::new_with_label("Click me!");
    let vertical_layout = gtk::Box::new(gtk::Orientation::Vertical,3);
    // Title
    let love_os_title = gtk::Label::new(None);
    love_os_title.set_markup(r#"<span size="large">I Love OS </span>"#);
    vertical_layout.add(&love_os_title);
    // Version
    let love_os_version = gtk::Label::new("Version 0.0.0000000000000001");
    vertical_layout.add(&love_os_version);

    window.add(&vertical_layout);

    window.show_all();
}

fn main() {
    let application = gtk::Application::new("org.iloveos.about", gio::ApplicationFlags::empty()).expect("Initialization failed...");

    application.connect_startup(|app| {
        build_ui(app);
    });
    //application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());
}
