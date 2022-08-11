use gio::prelude::*;
use gtk::prelude::*;
use gtk::{ApplicationWindow,Builder,GtkWindowExt};
//use glib;

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("test.glade");
    let builder = Builder::from_string(glade_src);
    
    
    let window: ApplicationWindow = builder.get_object("main_window").expect("Couldn't get the window");


    window.set_application(Some(application));
    window.set_title("Arithmetize");

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(true)
    });

    window.show_all();
}

fn main() {

    let application = gtk::Application::new(Some("com.azakidev.aritmetize"), Default::default(),)
    .expect("Initialization failed.");

    application.connect_activate(build_ui);

    application.run(&args().collect::<Vec<_>>());  
    
}
