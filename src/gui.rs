
mod ap_panel;
mod attitude_indicator;
mod common;
mod constants;
mod pfd;

extern crate egui;


pub mod gui {

    use std::{sync::{mpsc::{Sender, Receiver, TryRecvError}, Arc, Mutex, MutexGuard}, time::Duration, thread, io::Empty};

    use crate::bus::{BusMessage, AdcDataMessage, APCmdPayload, APStateMessage};
    use crate::gui::common::APBusMessageSender;
    use super::{pfd::PrimaryFligthDisplay, ap_panel::AutopilotPanel};

    fn setup_custom_fonts(ctx: &egui::Context) {
        // Start with the default fonts (we will be adding to them rather than replacing them).
        let mut fonts = egui::FontDefinitions::default();
    
        // Install my own font (maybe supporting non-latin characters).
        // .ttf and .otf files supported.
        fonts.font_data.insert(
            "FreeMono".to_owned(),
            egui::FontData::from_static(include_bytes!(
                "../fonts/FreeMono.ttf"
            )),
        );
    
        // Put my font first (highest priority) for proportional text:
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "FreeMono".to_owned());
    
        // Put my font as last fallback for monospace:
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .push("FreeMono".to_owned());
    
        // Tell egui to use these fonts:
        ctx.set_fonts(fonts);
    }

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
        ap_panel: AutopilotPanel,
        pfd: PrimaryFligthDisplay,
    }

    impl GuiApp {

        pub fn new(cc: &eframe::CreationContext<'_>, state: Arc<Mutex<GuiState>>, gui_tx_ap: Sender<BusMessage>) -> Self {
            
            setup_custom_fonts(&cc.egui_ctx);
            
            Self { 
                state: state, 
                gui_tx_ap: gui_tx_ap, 
                ap_panel: AutopilotPanel{}, 
                pfd: PrimaryFligthDisplay {}
            }
        }

    }

    impl APBusMessageSender for GuiApp {

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

                self.ap_panel.view_update(&mut state, ctx, ui, self);
                self.pfd.view_update(&mut state, ctx, ui);
            });

            ctx.request_repaint();
        }
    }

    pub struct Gui {
        pub state: Arc<Mutex<GuiState>>,
        pub rx_gui: Receiver<BusMessage>,
    }

    impl Gui {

        pub fn handle_bus_message(&mut self) {

            let d: Duration = Duration::from_millis(50);

            match self.rx_gui.try_recv() {
                Ok(bus_message) => {
                    match bus_message {
                        BusMessage::AdcData(adc_data) => self.handle_adc_data_message(adc_data),
                        BusMessage::APState(ap_state) => self.handle_ap_state_message(ap_state),
                        _ => (),
                    };           
                },
                Err(e) => {
                    match e {
                        TryRecvError::Empty => (),
                        TryRecvError::Disconnected => println!("[GUI] Message processing error : {:?}", e)
                    }   
                }
            }

            //thread::sleep(d);
        }

        fn handle_adc_data_message(&mut self, adc_data: AdcDataMessage) {
            //println!("[GUI][DATA] {:?}", adc_data);

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
