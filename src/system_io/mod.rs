//Os interaction
use std::process::Command;
use std::process::ChildStdout;

pub fn execute_command(location: &String, command: &String, arguments: &String){
    Command::new("xterm")
    .arg("-hold")
    .arg("-e")
    .arg("cd ".to_string() + location + " && " + command + " " + arguments)
    .spawn()
    .expect("Failed to run command");
}
