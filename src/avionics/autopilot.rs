
pub mod autopilot {

    use std::sync::{Arc, mpsc::{Sender, Receiver}};
    use crate::{bus::{AdcDataMessage, APCmdMessage, BusMessage, APStateMessage, SpeedUnit}};

    use crate::{flight_ctrl::FlightCtrlsProvider};

    pub struct Autopilot {

        pub engaged: bool,
        pub flcs: Arc::<dyn FlightCtrlsProvider + Send + Sync>,
        pub rx_ap: Receiver<BusMessage>,
        pub ap_tx_gui: Sender<BusMessage>,
    }

    impl Autopilot {

        pub fn handle_bus_message(&mut self) {

            match self.rx_ap.recv() {
                Ok(bus_message) => {
                    match bus_message {
                        BusMessage::AdcData(adc_data) => self.handle_adc_data_message(adc_data),
                        BusMessage::APCmd(ap_cmd) => self.handle_ap_cmd_message(ap_cmd),
                        _ => (),
                    };
                },
                Err(_) => println!("[AP] Message processing error")
            }
        }

        fn handle_adc_data_message(&mut self, adc_data: AdcDataMessage) {
            println!("[AP][DATA] {:?}", adc_data);
        }

        fn handle_ap_cmd_message(&mut self, ap_cmd: APCmdMessage) {
            println!("[AP][APCMD] {:?}", ap_cmd);

            let ap_state: APStateMessage = APStateMessage{
                engaged:false, 
                alt_hold_mode: false,
                vs_mode: false,
                heading_mode: false,
                auto_throttle_mode: false,
                alt: 15_000f32,
                heading: 180f32,
                speed: 250f32,
                speed_unit: SpeedUnit::IAS,
                bank_angle: 10f32,
                vs: 0f32
            };

            self.ap_tx_gui.send(BusMessage::APState(ap_state)).unwrap();
        }
    }

}

pub use autopilot::Autopilot;
