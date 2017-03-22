//Test File gtk_test

extern crate gtk;

//Custom mods
mod system_io;
mod gtk_converter;
pub mod m_config;
//Os interaction
use std::process::Command;
use std::process::ChildStdout;

use std::io;
use std::io::prelude::*;

use gtk::Builder;
use gtk::prelude::*;


// make moving clones into closures more convenient
//shameless copied from the examples
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

fn execute_command(location: &String, command: &String, arguments: &String){
    Command::new("xterm")
    .arg("-hold")
    .arg("-e")
    .arg("cd ".to_string() + location + " && " + command + " " + arguments)
    .spawn()
    .expect("Failed to run command");
}

fn convert_to_str(x: &str) -> &str{
    x
}

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let glade_src = include_str!("shipload.glade");
    let builder = Builder::new();
    builder.add_from_string(glade_src).unwrap();
//**********************************************
//Crucial

    let configuration = m_config::create_config();
//Main
    //Get Window
    let window: gtk::Window = builder.get_object("window").unwrap();
    //Close Button
    let close_button: gtk::Button = builder.get_object("B_Close").unwrap();
    //Set Header bar information
    let header: gtk::HeaderBar = builder.get_object("Header").unwrap();

    let pref_window: gtk::Window = builder.get_object("W_Preferences").unwrap();

    let pref_button: gtk::Button = builder.get_object("B_Preferences").unwrap();

    let pref_close: gtk::Button = builder.get_object("Pref_Close").unwrap();

    let pref_save: gtk::Button = builder.get_object("Pref_Save").unwrap();

//Cargo
    let cargo_build: gtk::Button = builder.get_object("B_Cargo_Build").unwrap();

    let cargo_build_folder: gtk::FileChooserButton = builder.get_object("Cargo_Build_FolderChooser").unwrap();

    let cargo_build_arguments: gtk::Entry = builder.get_object("Cargo_Build_ExtraOptions_Entry").unwrap();

    let cargo_run_run: gtk::Button = builder.get_object("B_Cargo_Run").unwrap();

    let cargo_run_arguments: gtk::Entry = builder.get_object("Cargo_Run_ExtraOptions_Entry").unwrap();
//RustUp

//Crates.io
    let text_buffer: gtk::TextBuffer = builder.get_object("CratesTextBuffer").unwrap();

    let search_button: gtk::Button = builder.get_object("CratesSearch").unwrap();

    let search_entry: gtk::Entry = builder.get_object("CratesSearch_Entry").unwrap();

    let level_bar: gtk::LevelBar = builder.get_object("SearchLevel").unwrap();

//**********************************************
//Main
    header.set_title("Teddy");
    header.set_subtitle("Rolf");

    //Close event
    close_button.connect_clicked(move |_| {
        println!("Closing normal!");

        gtk::main_quit();
        Inhibit(false);

    });
    //Window Close event
    window.connect_delete_event(|_,_| {
        gtk::main_quit();
        Inhibit(false)
    });

    //Preferences show event
    pref_button.connect_clicked(clone!(pref_window => move |_| {
        pref_window.show_all();
    }));
    //Hide, without save
    pref_close.connect_clicked(clone!(pref_window => move |_| {
        pref_window.hide();
    }));

    //Hide, with save
    pref_save.connect_clicked(clone!(pref_window => move |_| {
        pref_window.hide();
    }));

//Cargo
    cargo_build.connect_clicked(clone!(cargo_build_folder, cargo_build_arguments => move |_|{

        let argument_string: String = gtk_converter::text_from_entry(&cargo_build_arguments);

        let locationstr: String = gtk_converter::path_from_filechooser(&cargo_build_folder);
        execute_command(&locationstr, &"cargo build".to_string(), &argument_string.to_string());
    }));

    cargo_run_run.connect_clicked(clone!(cargo_run_arguments, cargo_build_folder => move |_|{

        let argument_string: String = gtk_converter::text_from_entry(&cargo_run_arguments);

        let locationstr: String = gtk_converter::path_from_filechooser(&cargo_build_folder);
        execute_command(&locationstr, &"cargo run".to_string(), &argument_string.to_string());

    }));
//RustUp

//Crates.io
    search_button.connect_clicked(clone!(text_buffer, search_entry => move |_| {


        let entry: String = gtk_converter::text_from_entry(&search_entry);

        while level_bar.get_value() != 0.2 {
            level_bar.set_value(0.2);
        }

        println!("Outside: {}", entry);
        level_bar.set_value(0.5);

        let output = Command::new("cargo").arg("search")
                                            .arg(entry)
                                            .arg("--limit")
                                            .arg("40")
                                            .output()
                                            .expect("Failed to ls");

        let out: String = String::from_utf8(output.stdout).expect("Not UTF-8");


        level_bar.set_value(0.75);

        let last: &str = convert_to_str(&out);
        text_buffer.set_text(last);

        level_bar.set_value(1.0);
    }));

    window.show_all();
    gtk::main();
}
