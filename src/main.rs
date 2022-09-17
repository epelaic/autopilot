
mod avionics;
mod bus;
mod flight_ctrl;
mod gui;
mod providers;
mod sensors;

extern crate yaml_rust;
use std::env;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc};
use std::thread;
use std::fs;
use std::time::Duration;
use flight_ctrls::FlightCtrlsProvider;
use yaml_rust::{YamlLoader, Yaml};
use crate::avionics::adc::adc::{ AdcRegistry, Adc};
use crate::avionics::autopilot::autopilot::Autopilot;
use crate::bus::{AdcDataMessage, APStateMessage, BusMessage, SpeedUnit};
use crate::{
    sensors::SensorsProvider, 
    flight_ctrl::flight_ctrls, 
    providers::providers::Provider, 
};

fn main() {

    let args: Vec<String> = env::args().collect();

    let conf_file_path: &String = &args[1];

    let conf_file: String = fs::read_to_string(conf_file_path).unwrap();
    let config_file_str: &str = conf_file.as_str();
    
    let configs: Vec<Yaml> = YamlLoader::load_from_str(config_file_str).unwrap();
    let config: &Yaml = &configs[0];

    let mut provider: Box::<dyn Provider> = providers::resolve_provider(&config);
    provider.init();

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


    let adc: Adc = Adc{sensors: sensors};
    let mut autopilot: Autopilot = Autopilot{engaged: false, flcs: flcs, ap_tx_gui: ap_tx_gui};

    // Init Thread ADC
    let adc_handler = thread::spawn(move || -> ! {

        let d: Duration = Duration::from_millis(100);
        
        loop {
            
            let adc_registry: AdcRegistry = adc.get_frame();

            let adc_data:AdcDataMessage = adc_registry.to_adc_data();
            let gui_bus_message = BusMessage::AdcData(adc_data);
            let ap_bus_message: BusMessage = gui_bus_message.clone();

            println!("[ADC] sending data...");
            adc_tx_gui.send(gui_bus_message).unwrap();
            adc_tx_ap.send(ap_bus_message).unwrap();

            thread::sleep(d);
        }
    });

    // Init Thread AP
    let ap_handler = thread::spawn(move || -> ! {


        let d: Duration = Duration::from_millis(100);
        
        loop {

            // Read Sensor Data or Handle GUI AP commands
            match rx_ap.recv() {
                Ok(message) => {
                    match message {
                        BusMessage::AdcData(adc_data) => autopilot.handle_adc_data_message(adc_data),
                        BusMessage::APCmd(ap_cmd) => autopilot.handle_ap_cmd_message(ap_cmd),
                        _ => (),
                    }
                },            
                Err(_) => println!("[AP] Message processing error")
            }

            thread::sleep(d);
        }
    });

    // Init Thread GUI
    let gui_handler = thread::spawn(move || -> ! {

        gui::gui_init();

        let d: Duration = Duration::from_millis(1000);
        
        loop {

            match rx_gui.recv() {
                Ok(message) => {
                    match message {
                        BusMessage::AdcData(adc_data) => println!("[GUI][DATA] {:?}", adc_data),
                        BusMessage::APState(ap_state) => println!("[GUI][APSTATE] {:?}", ap_state),
                        _ => (),
                    }
                },            
                Err(_) => println!("[GUI] Message processing error")
            }

            thread::sleep(d);
        }
    });

    println!("Autopilot ready");

    adc_handler.join().expect("Adc handler error");
    ap_handler.join().expect("AP handler error");
    gui_handler.join().expect("GUI handler error");

}
