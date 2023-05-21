use gtk::prelude::*;
use gtk::{glib, Application, Button, TextBuffer, Window};

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> glib::ExitCode {
   // Create a new application
   let app = Application::builder().application_id(APP_ID).build();

   // Connect to "activate" signal of `app`
   app.connect_activate(build_ui);

   // Run the application
   app.run()
}

fn build_ui(app: &Application) {
   // Create a window and set the title
   let ui = include_str!("../mmviz.ui");
   let builder = gtk::Builder::from_string(ui);

   let window: Window = builder.object("window1").unwrap();
   let btn: Button = builder.object("btn1").unwrap();
   let text: gtk::TextView = builder.object("text1").unwrap();
   btn.connect_clicked(move |_| {
      let cnt = "aa";
      text.buffer().set_text(&cnt.to_string());
   });
   window.set_application(Some(app));
   // Present window
   window.present();
}
