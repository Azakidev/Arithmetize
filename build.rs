use adw::gio::compile_resources;

fn main() {
    compile_resources(
        "content",
        "content/app.gresource.xml",
        "gtk-aritmetize.gresource",
    );
}