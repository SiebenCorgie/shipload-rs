extern crate gtk;


use gtk::Builder;
use gtk::prelude::*;


pub fn path_from_filechooser(filechooser_widget: &gtk::FileChooserButton) -> String {
	//Get location from widget
	let location = filechooser_widget.get_filename();
	//Final String
	let mut locationstr: String = "~".to_string();
	match location{
		Some(loc) => locationstr = match loc.to_str()
									{
										Some(string) => string.to_string(),
										None => "None".to_string(),
									},
		None => locationstr = "~".to_string(),
	}
	return locationstr;
}


pub fn text_from_entry(widget: &gtk::Entry) -> String{
	let argument = widget.get_text();
	let mut argument_string: String = "".to_string();
	match argument{
		Some(arg) => argument_string = arg,
		None => argument_string = "".to_string(),
	}
	argument_string
}
