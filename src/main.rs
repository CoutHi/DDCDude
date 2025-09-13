use gtk::ffi::{GtkAlign, GTK_ALIGN_CENTER};
use gtk::{prelude::*, ApplicationWindow, Text};
use std::fmt::Error;
use std::process::Command;
use std::io;
use gtk::{glib, Align, Application};

const APP_ID : &str = "com.couthi.DDCDude";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let input_field = Text::builder()
        .max_length(3)
        .hexpand_set(true)
        .vexpand_set(true)
        .placeholder_text("Brightness %")
        .build();

    input_field.set_halign(Align::Center);
    input_field.set_valign(Align::Center);
    input_field.set_alignment(0.5);
    input_field.connect_activate(entered);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("DDCDude")
        .default_width(512)
        .resizable(false)
        .default_height(128)
        .child(&input_field)
        .build();
    window.present();
}

fn get_monitors(percent: glib::GString) -> Result<u8, std::io::Error>{
    let cmd_get = Command::new("ddcutil").arg("det").output().expect("ddcutil Couldn't Run.");
    let buffer = cmd_get.stdout;
    let cmd_input = String::from_utf8(buffer).expect("Couldn't Convert ddcutil det To UTF-8.");

    let mons : Vec<usize> = cmd_input.match_indices("Display").map(|(i, _)|i).collect();
    println!("Monitors: {:#?}", mons);

    for element in mons {
        let mon = &cmd_input[element+8..element+9];
        let mut cmd_set = Command::new("ddcutil");
        cmd_set.args(["-d", mon.trim(), "setvcp", "10", percent.as_str()]);
        println!("Running: {:#?}", cmd_set);
        let res = cmd_set.output();
        match res {
            Ok(_) => continue,
            Err(e) => return Err(e)
        }
    }
    Ok(0)
}

fn entered(field: &Text){
    println!("Setting Brightness To: {}%", field.text());
    get_monitors(field.text()).expect("Failed");
}
