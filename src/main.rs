
extern crate yaml_rust;
use std::env;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use std::thread;
use std::fs;
use flight_ctrls::FlightCtrlsProvider;
use yaml_rust::{YamlLoader, Yaml};
use crate::bus::{AdcDataMessage, APCmdMessage, APStateMessage, BusMessage};
use crate::{
    sensors::SensorsProvider, 
    flight_ctrl::flight_ctrls, 
    providers::providers::Provider, 
    avionics::{avionics::Avionics}};

mod avionics;
mod bus;
mod flight_ctrl;
mod gui;
mod providers;
mod sensors;


fn main() {

    let args: Vec<String> = env::args().collect();

    let conf_file_path: &String = &args[1];

    let conf_file: String = fs::read_to_string(conf_file_path).unwrap();
    let config_file_str: &str = conf_file.as_str();
    
    let configs: Vec<Yaml> = YamlLoader::load_from_str(config_file_str).unwrap();

    let config: &Yaml = &configs[0];

    // Debug support
    // println!("{:?}", config);

    let mut provider: Box::<dyn Provider> = providers::resolve_provider(&config);
    provider.init();

    let sensors: Box::<dyn SensorsProvider> = provider.get_sensors();
    let flcs: Box::<dyn FlightCtrlsProvider> = provider.get_flcs();

    //sensors.acquire();
    //flcs.send();
    //sensors.acquire();
    //flcs.send();
    //sensors.acquire();
    //sensors.acquire();
    
    // MPSC channel to send data from (ADC, AP) to GUI.
    let (tx_gui , rx_gui): (Sender<BusMessage>, Receiver<BusMessage>) = mpsc::channel();

    // MPSC channel to send data from (ADC, GUI) data to AP.
    let (tx_ap , rx_ap): (Sender<BusMessage>, Receiver<BusMessage>) = mpsc::channel();

    let avionics: Avionics = avionics::avionics_init(sensors, flcs);

    gui::gui_init();

    println!("Autopilot ready");
}
