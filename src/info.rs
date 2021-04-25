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

pub fn gtk_error<T: AsRef<str>>(title: T, window: Option<&gtk::ApplicationWindow>) {
    let message_dialog = gtk::Dialog::new_with_buttons(Some(title.as_ref()), window, gtk::DialogFlags::MODAL, &[("Close", gtk::ResponseType::Close)]);
        
    message_dialog.connect_response(move |d, _| {
        d.hide();
    });

    message_dialog.show();
    std::process::exit(1);
}
