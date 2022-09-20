
mod pfd;

extern crate egui;


pub mod gui {

    use std::{sync::{mpsc::{Sender, Receiver}, Arc, Mutex, MutexGuard}};
    use crate::{bus::{BusMessage, AdcDataMessage, APCmdPayload, APStateMessage}};

    const ALT_MAX_VALUE: f32 = 40_000f32;
    const ALT_MIN_VALUE: f32 = 0f32;
    const ALT_100_INCREMENT: f32 = 100f32;
    const ALT_500_INCREMENT: f32 = 500f32;

    pub struct GuiState {
        pub adc_state: AdcDataMessage,
        pub ap_state: APStateMessage,
    }


    impl GuiState {

        pub const fn new() -> Self {
            
            Self{ adc_state: AdcDataMessage::new(), ap_state: APStateMessage::new() }
        }
    }

    pub struct GuiApp {
        pub state: Arc<Mutex<GuiState>>,
        pub gui_tx_ap: Sender<BusMessage>,
    }

    impl GuiApp {
        
        fn increment_value(&self, old_value: &mut f32, step: f32, max: f32) {

            let mut new_value: f32 = *old_value + step;

            if new_value > max {
                new_value = max;
            }

            *old_value =  new_value;
        }

        fn decrement_value(&self, old_value: &mut f32, step: f32, min: f32) {

            let mut new_value: f32 = *old_value - step;

            if new_value < min {
                new_value = min;
            }

            *old_value =  new_value;
        }

        fn send_ap_cmd(&self, ap_cmd_payload: APCmdPayload) {

            let bus_message: BusMessage = BusMessage::APCmd(ap_cmd_payload);
            let _ = self.gui_tx_ap.send(bus_message);
        }
    }

    impl eframe::App for GuiApp {

        fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

            let mut state: MutexGuard<GuiState> = self.state.lock().unwrap();

            egui::CentralPanel::default().show(ctx, |ui| {
                
                ui.heading("Autopilot App");

                //ui.label(format!("AP alt: {}", state.ap_state.alt));

                ui.horizontal(|ui| {

                    if ui.button("<<").clicked() {
    
                        self.decrement_value(&mut state.ap_state.alt , ALT_500_INCREMENT, ALT_MIN_VALUE);
                        self.send_ap_cmd(APCmdPayload::SetAlt(state.ap_state.alt));
                    }

                    if ui.button("<").clicked() {
    
                        self.decrement_value(&mut state.ap_state.alt , ALT_100_INCREMENT, ALT_MIN_VALUE);
                        self.send_ap_cmd(APCmdPayload::SetAlt(state.ap_state.alt));
                    }

                    ui.label(format!("AP alt: {}ft", state.ap_state.alt));

                    if ui.button(">").clicked() {
    
                        self.increment_value(&mut state.ap_state.alt , ALT_100_INCREMENT, ALT_MAX_VALUE);
                        self.send_ap_cmd(APCmdPayload::SetAlt(state.ap_state.alt));
                    }

                    if ui.button(">>").clicked() {
    
                        self.increment_value(&mut state.ap_state.alt , ALT_500_INCREMENT, ALT_MAX_VALUE);
                        self.send_ap_cmd(APCmdPayload::SetAlt(state.ap_state.alt));
                    }

                });
            });
        }

    }

    pub struct Gui {
        pub state: Arc<Mutex<GuiState>>,
        pub rx_gui: Receiver<BusMessage>,
    }

    impl Gui {

        pub fn handle_bus_message(&mut self) {

            match self.rx_gui.try_recv() {
                Ok(bus_message) => {
                    match bus_message {
                        BusMessage::AdcData(adc_data) => self.handle_adc_data_message(adc_data),
                        BusMessage::APState(ap_state) => self.handle_ap_state_message(ap_state),
                        _ => (),
                    };           
                },
                Err(_) => ()//println!("[GUI] Message processing error")
            }
        }

        fn handle_adc_data_message(&mut self, adc_data: AdcDataMessage) {
            println!("[GUI][DATA] {:?}", adc_data);

            let mut state: MutexGuard<GuiState> = self.state.lock().unwrap();
            state.adc_state = adc_data;
        }

        fn handle_ap_state_message(&mut self, ap_cmd: APStateMessage) {
            println!("[GUI][APSTATE] {:?}", ap_cmd);

            let mut state: MutexGuard<GuiState> = self.state.lock().unwrap();
            state.ap_state = ap_cmd;
        }
    }

}

pub use gui::Gui;
pub use gui::GuiApp;
