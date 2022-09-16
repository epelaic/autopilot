mod pfd;

pub mod gui {

    use std::sync::mpsc::{Receiver, Sender};
    use std::thread;
    use std::time::Duration;
    use crate::{gui::pfd, bus::BusMessage};

    pub fn gui_init() {

        println!("Start init gui module");

        pfd::pfd_init();

        println!("End init gui module");

    }
}

pub use gui::gui_init;
