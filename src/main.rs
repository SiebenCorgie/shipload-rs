

extern crate gtk;

use gtk::prelude::*;

use gtk::{
    self, AboutDialog, AppChooserDialog, Builder, Button, Dialog, Entry, FileChooserAction,
    FileChooserDialog, FontChooserDialog, Scale, SpinButton, RecentChooserDialog, ResponseType,
    Spinner, Window
};



fn main() {

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let glade_src = include_str!("TestGlade.glade");
    let builder = Builder::new();
    builder.add_from_string(glade_src);

    let window: gtk::Window = builder.get_object("MainWindow");

    window.connect_delete_event(|_,_| {
        gtk::main_quit();
        Inhibit(false)

    window.show_all();
    gtk::main();
    
    println!("Hello, world!");
}
