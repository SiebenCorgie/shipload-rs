//Test File gtk_test

extern crate gtk;

use gtk::Builder;
use gtk::prelude::*;


fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let glade_src = include_str!("shipload.glade");
    let builder = Builder::new();
    builder.add_from_string(glade_src).unwrap();

    let window: gtk::Window = builder.get_object("window").unwrap();
    //window.set_auto_startup_notification(true);
    let button: gtk::Button = builder.get_object("B_Close").unwrap();
    let label: gtk::Label = builder.get_object("L_Crates").unwrap();

    let header: gtk::HeaderBar = builder.get_object("Header").unwrap();
    header.set_title("Teddy");
    header.set_subtitle("Rolf");

    button.connect_clicked(move |_| {
        println!("Closing normal!");

        gtk::main_quit();
        Inhibit(false);

    });

    window.connect_delete_event(|_,_| {
        gtk::main_quit();
        Inhibit(false)
    });
    window.show_all();
    gtk::main();
}
