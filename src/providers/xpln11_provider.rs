use yaml_rust::Yaml;

use crate::sensors::SensorsProvider;
use super::providers::Provider;

const NAME: &str = "XPLN11";

pub struct XPLN11SensorsProvider {

    host: String,
    port: i64
}

impl XPLN11SensorsProvider {

    fn connect(&self) {}

    fn get_data(&self) {}

    fn close(&self) {}

}

impl Provider for XPLN11SensorsProvider {

    fn name(&self) -> &str {

        return NAME;
    }

    fn init(&self) {
        println!("XPLN11 Provider config : host: {}, port: {}", self.host, self.port);
        self.connect();
    }

    fn shutdown(&self) {
        println!("XPLN11 SensorsProvider shutdown");
        self.close();
    }
}

impl SensorsProvider for XPLN11SensorsProvider {

    fn acquire(&self) {
        println!("XPLN11 SensorsProvider acquire");
        self.get_data();
    }
}

pub fn xpl11_provider_init(config: &Yaml) -> Box<dyn SensorsProvider> {

    println!("Start init xpl11_provider module");

    let host = config["provider"]["host"].as_str().unwrap().to_string();
    let port = config["provider"]["port"].as_i64().unwrap();

    let provider = Box::new(XPLN11SensorsProvider{host, port});

    println!("End init xpl11_provider module");

    return provider;
}

