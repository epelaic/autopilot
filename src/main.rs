
extern crate yaml_rust;
use flight_ctrls::FlightCtrlsProvider;
use yaml_rust::{YamlLoader, Yaml};
use crate::{sensors::SensorsProvider, flight_ctrl::flight_ctrls, providers::providers::Provider, avionics::{avionics::Avionics}};

mod avionics;
mod flight_ctrl;
mod gui;
mod providers;
mod sensors;


fn main() {

    let config_file_str: &str =
    "
    provider: 
        name: xpln11
        host: 127.0.0.1
        read_port: 49003
        write_port: 49000
    ";
       
    let configs = YamlLoader::load_from_str(config_file_str).unwrap();

    let config: &Yaml = &configs[0];

    // Debug support
    // println!("{:?}", config);

    assert_eq!(config["provider"]["name"].as_str().unwrap(), "xpln11");

    let mut provider: Box::<dyn Provider> = providers::resolve_provider(&config);
    provider.init();

    let sensors: Box::<dyn SensorsProvider> = provider.get_sensors();
    let flcs: Box::<dyn FlightCtrlsProvider> = provider.get_flcs();

    sensors.acquire();
    flcs.send();
    sensors.acquire();
    flcs.send();
    sensors.acquire();
    sensors.acquire();
    

    let avionics: Avionics = avionics::avionics_init(sensors, flcs);

    gui::gui_init();

    println!("Autopilot ready");
}
