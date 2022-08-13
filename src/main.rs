mod main_window;
use main_window::MainWindow;
use gtk4::{self, traits::WidgetExt, prelude::*, gio::{resources_register_include}};
use adw::Application;

mod logic;

fn build_ui(application: &Application) {

    let window = MainWindow::new(application);
    window.show();

}

pub fn main() {

    resources_register_include!("gtk-aritmetize.gresource").expect("Failed to register resources");
    
    let application = Application::builder()
        .application_id("com.azakiradev.arithmetize")
        .build();

    application.connect_activate(build_ui);
    application.run();  
    
}
