
extern crate yaml_rust;
use std::env;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc};
use std::thread;
use std::fs;
use std::time::Duration;
use flight_ctrls::FlightCtrlsProvider;
use yaml_rust::{YamlLoader, Yaml};
use crate::avionics::adc::{AdcRegistry, Adc};
use crate::bus::{AdcDataMessage, APCmdMessage, APStateMessage, BusMessage};
use crate::{
    sensors::SensorsProvider, 
    flight_ctrl::flight_ctrls, 
    providers::providers::Provider, 
    avionics::{avionics::Avionics }};

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

    let sensors: Arc::<dyn SensorsProvider + Send + Sync> = provider.get_sensors();
    let flcs: Arc::<dyn FlightCtrlsProvider + Send + Sync> = provider.get_flcs();

    //sensors.acquire();
    //flcs.send();
    //sensors.acquire();
    //flcs.send();
    //sensors.acquire();
    //sensors.acquire();
    
    // MPSC channel to send data from (ADC, AP) to GUI.
    let (tx_gui , rx_gui): (Sender<BusMessage>, Receiver<BusMessage>) = mpsc::channel();
    let adc_tx_gui: Sender<BusMessage> = tx_gui.clone();

    // MPSC channel to send data from (ADC, GUI) data to AP.
    let (tx_ap , rx_ap): (Sender<BusMessage>, Receiver<BusMessage>) = mpsc::channel();
    let adc_tx_ap: Sender<BusMessage> = tx_ap.clone();
    let gui_tx_ap: Sender<BusMessage> = tx_ap.clone();

    let avionics: Avionics = avionics::avionics_init(
        sensors, 
        flcs);

    let adc: Arc<Adc> = avionics.adc;

    gui::gui_init();

    // Init Thread ADC
    let adc_handler = thread::spawn(move || -> ! {

        let d: Duration = Duration::from_millis(100);
        
        loop {
            
            let adc_registry: AdcRegistry = adc.get_frame();

            let adc_data:AdcDataMessage = adc_registry.to_adc_data();
            let bus_message = BusMessage::AdcData(adc_data);

            println!("[ADC] sending data...");
            adc_tx_gui.send(bus_message).unwrap();

            thread::sleep(d);
        }
    });

    // Init Thread AP

    // Init Thread GUI
    let gui_handler = thread::spawn(move || -> ! {

        let d: Duration = Duration::from_millis(1000);
        
        loop {

            match rx_gui.recv() {
                Ok(message) => {
                    match message {
                        BusMessage::AdcData(adc_data) => println!("[GUI][DATA] {:?}", adc_data),
                        BusMessage::APState(ap_state) => println!("[GUI][APSTATE] {:?}", ap_state),
                        BusMessage::APCmd(ap_cmd) => println!("[GUI][APCMD] {:?}", ap_cmd),
                    }
                },            
                Err(e) => ()
            }

            thread::sleep(d);
        }
    });

    println!("Autopilot ready");

    adc_handler.join().expect("Adc handler error");
    gui_handler.join().expect("Adc handler error");

}
