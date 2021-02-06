extern crate gtk;

use gtk::*;
use std::process;

fn main() {
    if gtk::init().is_err() {
        eprintln!("failed to initialize gtk application");
        process::exit(1);
    }

    // Initialize the UI's initial state
    let app = App::new();

    // Make all the widgets within the UI visible.
    app.window.show_all();

    // Start the GTK main event loop
    gtk::main();
}

pub struct App {
    pub window: Window,
    pub header: Header,
}

pub struct Header {
    pub container: HeaderBar,
}

impl App {
    fn new() -> App {
        let window = Window::new(WindowType::Toplevel);

        let header = Header::new();

        window.set_titlebar(Some(&header.container));
        window.set_title("First Gtk App");
        window.set_default_size(350, 200);
        Window::set_default_icon_name("Gtk App");

        //what to do whe the close button is closed
        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });

        App { window, header }
    }
}

impl Header {
    fn new() -> Header {
        let container = HeaderBar::new();

        container.set_title(Some("First Gtk App"));
        //enable window control within this header-bar
        container.set_show_close_button(true);

        Header { container }
    }
}
