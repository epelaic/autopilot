
extern crate yaml_rust;
use yaml_rust::{YamlLoader, Yaml};
use crate::sensors::SensorsProvider;

mod avionics;
mod gui;
mod providers;
mod sensors;


fn main() {

    let config_file_str: &str =
    "
    provider: 
        name: xpln11
        host: 127.0.0.1
        port: 49003
    ";
       
    let configs = YamlLoader::load_from_str(config_file_str).unwrap();

    let config: &Yaml = &configs[0];

    // Debug support
    println!("{:?}", config);

    assert_eq!(config["provider"]["name"].as_str().unwrap(), "xpln11");

    let mut sensors_provider: Box<dyn SensorsProvider> = sensors::sensors_init(&config);
    sensors_provider.init();

    avionics::avionics_init();
    gui::gui_init();

    println!("Autopilot ready");
}
