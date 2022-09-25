#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod avionics;
mod bus;
mod flight_ctrl;
mod gui;
mod providers;
mod sensors;

extern crate yaml_rust;

use std::env;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self};
use std::fs;
use std::time::Duration;
use gui::gui::GuiState;
use yaml_rust::{YamlLoader, Yaml};

use crate::avionics::adc::adc::{Adc};
use crate::avionics::autopilot::autopilot::Autopilot;
use crate::bus::{BusMessage};
use crate::sensors::SensorsProvider; 
use crate::flight_ctrl::flight_ctrls::FlightCtrlsProvider; 
use crate::providers::providers::Provider;
use crate::gui::{Gui, GuiApp};

fn main() {

    let args: Vec<String> = env::args().collect();

    let conf_file_path: &String = &args[1];

    let conf_file: String = fs::read_to_string(conf_file_path).unwrap();
    let config_file_str: &str = conf_file.as_str();
    
    let configs: Vec<Yaml> = YamlLoader::load_from_str(config_file_str).unwrap();
    let config: &Yaml = &configs[0];

    let mut provider: Box::<dyn Provider> = providers::resolve_provider(&config);
    provider.init();

    let adc_frame_rate = config["adc"]["frame_rate"].as_i64().unwrap() as u64;
    println!("adc_frame_rate : {}", adc_frame_rate);

    let sensors: Arc::<dyn SensorsProvider + Send + Sync> = provider.get_sensors();
    let flcs: Arc::<dyn FlightCtrlsProvider + Send + Sync> = provider.get_flcs();
    
    // MPSC channel to send data from (ADC, AP) to GUI.
    let (tx_gui , rx_gui): (Sender<BusMessage>, Receiver<BusMessage>) = mpsc::channel();
    let adc_tx_gui: Sender<BusMessage> = tx_gui.clone();
    let ap_tx_gui: Sender<BusMessage> = tx_gui.clone();

    // MPSC channel to send data from (ADC, GUI) data to AP.
    let (tx_ap , rx_ap): (Sender<BusMessage>, Receiver<BusMessage>) = mpsc::channel();
    let adc_tx_ap: Sender<BusMessage> = tx_ap.clone();
    let gui_tx_ap: Sender<BusMessage> = tx_ap.clone();


    // ----- Building ADC
    let adc: Adc = Adc{
        sensors: sensors, 
        adc_tx_gui: adc_tx_gui, 
        adc_tx_ap: adc_tx_ap};

    // ----- Builing AP
    let mut autopilot: Autopilot = Autopilot::from(
        flcs, 
        rx_ap, 
        ap_tx_gui);
    
    // ----- Building GUI
    let gui_state_mutex: Mutex<GuiState> = Mutex::new(GuiState::new());
    let gui_state: Arc<Mutex<GuiState>> = Arc::new(gui_state_mutex);
    let gui_state2: Arc<Mutex<GuiState>>  = gui_state.clone();
    
    let gui_app: GuiApp = GuiApp::from(gui_state, gui_tx_ap);

    let mut gui: Gui = Gui{
        state: gui_state2,
        rx_gui: rx_gui,
    };

    let mut handles = vec![];

    // ----- Init Thread ADC -----
    let adc_handle = thread::spawn(move || -> ! {

        let d: Duration = Duration::from_millis(adc_frame_rate);
        
        loop {
            // Read sensors and convert to ADC format
            adc.read_sensors();

            thread::sleep(d);
        }
    });
    handles.push(adc_handle);

    // ----- Init Thread AP -----
    let ap_handle = thread::spawn(move || -> ! {
        
        loop {
            // Read ADC Data or Handle GUI AP commands
            autopilot.handle_bus_message();
        }
    });
    handles.push(ap_handle);

    // ----- Init Thread GUI -----
    let gui_handle = thread::spawn(move || -> ! {
        
        loop {
            // Read ADC Data or AP State
            gui.handle_bus_message();
        }
    });
    handles.push(gui_handle);

    // Init Gui APP
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Autopilot",
        options,
        Box::new(|_cc| Box::new(gui_app)),
    );

    println!("Autopilot ready");

    // join the handles in the vector
    //for i in handles {
    //    i.join().unwrap();
    //}

}
