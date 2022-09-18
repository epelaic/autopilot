mod pfd;

pub mod gui {

    use std::sync::{mpsc::{Sender, Receiver}};
    use crate::bus::{BusMessage, AdcDataMessage, APCmdMessage};

    pub struct Gui {

        pub rx_gui: Receiver<BusMessage>,
        pub gui_tx_ap: Sender<BusMessage>,
    }

    impl Gui {


        pub fn handle_bus_message(&mut self) {

            match self.rx_gui.recv() {
                Ok(bus_message) => {
                    match bus_message {
                        BusMessage::AdcData(adc_data) => self.handle_adc_data_message(adc_data),
                        BusMessage::APCmd(ap_cmd) => self.handle_ap_state_message(ap_cmd),
                        _ => (),
                    };           
                },
                Err(_) => println!("[GUI] Message processing error")
            }
        }

        fn handle_adc_data_message(&mut self, adc_data: AdcDataMessage) {
            println!("[GUI][DATA] {:?}", adc_data);
        }

        fn handle_ap_state_message(&mut self, ap_cmd: APCmdMessage) {
            println!("[GUI][DATA] {:?}", ap_cmd);
        }
    }
}

pub use gui::Gui;
