use std::process::Command;
use gtk::prelude::*;

pub fn rustc_version() -> Result<String, ()> {
    match Command::new("rustc")
        .arg("--version")
        .output() {
        Ok(data) => {
            match std::str::from_utf8(&data.stdout) {
                Ok(stdout) => return Ok(stdout.to_string()),
                Err(_) => return Err(()),
            };
        },
        Err(_) => return Err(()),
    };
}

pub fn gtk_error(main: &str, bottom: &str, window: Option<&gtk::ApplicationWindow>) {
    match window {
        Some(window) => {
            let message_dialog = gtk::MessageDialogBuilder::new()
                .transient_for(window)
                .title(main)
                .text(bottom)
                .resizable(false)
                .use_markup(true)
                .buttons(gtk::ButtonsType::Ok)
                .focusable(true)
                .build();
        
            message_dialog.connect_response(move |d: &gtk::MessageDialog, _: gtk::ResponseType| {
                d.hide();
            });
        
            message_dialog.show();
        },
        None =>{
            let message_dialog = gtk::MessageDialogBuilder::new()
                .title(main)
                .text(bottom)
                .resizable(false)
                .use_markup(true)
                .buttons(gtk::ButtonsType::Ok)
                .focusable(true)
                .build();

            message_dialog.connect_response(move |d: &gtk::MessageDialog, _: gtk::ResponseType| {
                d.hide();
            });

            message_dialog.show()
        },
    };
}
