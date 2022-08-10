pub mod sensors {

    extern crate yaml_rust;
    use yaml_rust::Yaml;
    use crate::providers::{self, providers::Provider};

    pub trait  SensorsProvider : Provider {

        fn acquire(&self);
    }

    pub fn sensors_init(config: &Yaml) -> Box<dyn SensorsProvider> {

        println!("Start init sensors module");

        let provider: Box<dyn SensorsProvider> = providers::resolve_sensor_provider(&config);
        println!("Sensors provider : {}", provider.name());
        return provider;
    }
}

pub use sensors::sensors_init;
pub use sensors::SensorsProvider;
