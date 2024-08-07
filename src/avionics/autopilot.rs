
pub mod autopilot {

    use std::sync::{Arc, mpsc::{Sender, Receiver, TryRecvError}};
    use crate::bus::{APCmdPayload, APStateMessage, APTurnSide, AdcDataMessage, BusMessage};
    use crate::flight_ctrl::FlightCtrlsProvider;

    pub struct Autopilot {

        pub ap_state: APStateMessage,
        pub flcs: Arc::<dyn FlightCtrlsProvider + Send + Sync>,
        pub rx_ap: Receiver<BusMessage>,
        pub ap_tx_gui: Sender<BusMessage>,
    }

    impl Autopilot {

        pub const fn from(
            flcs: Arc::<dyn FlightCtrlsProvider + Send + Sync>,
            rx_ap: Receiver<BusMessage>,
            ap_tx_gui: Sender<BusMessage>) -> Self {

            Self { 
                ap_state: APStateMessage::new(), 
                flcs: flcs, 
                rx_ap: rx_ap, 
                ap_tx_gui: ap_tx_gui }
        }

        pub fn handle_bus_message(&mut self) {

            match self.rx_ap.try_recv() {
                Ok(bus_message) => {
                    match bus_message {
                        BusMessage::AdcData(adc_data) => self.handle_adc_data_message(adc_data),
                        BusMessage::APCmd(ap_cmd) => self.handle_ap_cmd_message(ap_cmd),
                        _ => (),
                    };
                },
                Err(e) => {
                    match e {
                        TryRecvError::Empty => (),
                        TryRecvError::Disconnected => println!("[AP] Message processing error : {:?}", e)
                    }   
                }
            }
        }

        fn handle_adc_data_message(&mut self, _adc_data: AdcDataMessage) {
            //println!("[AP][DATA] {:?}", adc_data);
        }

        fn handle_ap_cmd_message(&mut self, ap_cmd: APCmdPayload) {

            println!("[AP][APCMD] {:?}", ap_cmd);

            match ap_cmd {
                APCmdPayload::SetAlt(alt) => self.set_ap_alt(alt),
                APCmdPayload::EnableAltHoldMode(enable) => self.ap_state.alt_hold_mode = enable,
                APCmdPayload::SetHeading{heading, turn_side} => self.set_ap_heading(heading as f32, turn_side),
                APCmdPayload::SetBankAngle(angle) => self.set_ap_bank_angle(angle),
                _ => ()
            }

            let ap_state: APStateMessage = self.ap_state.clone();

            self.ap_tx_gui.send(BusMessage::APState(ap_state)).unwrap();
        }

        fn set_ap_alt(&mut self, alt: f32) {

            self.ap_state.alt = alt;

            self.notify_observers();
        }

        fn set_ap_heading(&mut self, heading: f32, _turn_side: APTurnSide) {

            self.ap_state.heading = heading;

            self.notify_observers();
        }

        fn set_ap_bank_angle(&mut self, angle: i8) {

            self.ap_state.bank_angle = angle as f32;

            self.notify_observers();
        }

        // Notify GUI
        fn notify_observers(&self) {

            let ap_state: APStateMessage = self.ap_state.clone();
            self.ap_tx_gui.send(BusMessage::APState(ap_state)).unwrap();
        }

    }

}
