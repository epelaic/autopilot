pub(crate) mod xpln11_provider;
pub(crate) mod sim_mock_provider;

pub mod providers {

    extern crate yaml_rust;
    use core::fmt;
    use std::sync::Arc;
    use yaml_rust::Yaml;
    use crate::flight_ctrl::FlightCtrlsProvider;
    use crate::sensors::SensorsProvider;
    use crate::providers::xpln11_provider;
    use crate::providers::sim_mock_provider;

    pub trait Provider {

        fn name(&self) -> &str;
        fn init(&mut self);
        fn shutdown(&self);
        fn get_sensors(&self) -> Arc::<dyn SensorsProvider + Send + Sync>;
        fn get_flcs(&self) -> Arc::<dyn FlightCtrlsProvider + Send + Sync>;
    }

    enum SensorsProviderEnum {
        XPLN11,
        SIMMOCK,
    }

    impl fmt::Display for SensorsProviderEnum {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                SensorsProviderEnum::XPLN11 => write!(f, "xpln11"),
                SensorsProviderEnum::SIMMOCK => write!(f, "simmock"),
            }
        }
    }

    pub fn resolve_provider(config: &Yaml) -> Box<dyn Provider> {

        println!("Resolving provider");

        let provider: Box<dyn Provider>;

        let provider_name = config["provider"]["name"].as_str().unwrap();

        if SensorsProviderEnum::XPLN11.to_string().eq(provider_name) {
            println!("Loading X-Plane 11 sensors provider");
            provider = xpln11_provider::xpl11_provider_init(&config) as Box<dyn Provider>;
        } else if SensorsProviderEnum::SIMMOCK.to_string().eq(provider_name) {
            println!("Loading SimMock sensors provider");
            provider = sim_mock_provider::simmock_provider_init(&config) as Box<dyn Provider>;
        } else {
            panic!("Unknown provider: {provider_name}");
        }

        println!("End init module");

        return provider;
    }

}

pub use providers::resolve_provider;
