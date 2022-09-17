mod pfd;

pub mod gui {

    use crate::{gui::pfd};

    pub fn gui_init() {

        println!("Start init gui module");

        pfd::pfd_init();

        println!("End init gui module");

    }
}

pub use gui::gui_init;
