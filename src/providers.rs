pub(crate) mod xpln11_provider;

pub mod providers {

    extern crate yaml_rust;
    use core::fmt;
    use yaml_rust::Yaml;
    use crate::flight_ctrl::FlightCtrlsProvider;
    use crate::sensors::SensorsProvider;
    use crate::providers::xpln11_provider;

    pub trait Provider : SensorsProvider + FlightCtrlsProvider{

        fn name(&self) -> &str;
        fn init(&mut self);
        fn shutdown(&self);
    }

    enum SensorsProviderEnum {
        XPLN11,
    }

    impl fmt::Display for SensorsProviderEnum {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                SensorsProviderEnum::XPLN11 => write!(f, "xpln11"),
            }
        }
    }

    enum FligthCtrlsProviderEnum {
        XPLN11,
    }

    impl fmt::Display for FligthCtrlsProviderEnum {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                FligthCtrlsProviderEnum::XPLN11 => write!(f, "xpln11"),
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
        } else {
            panic!("Unknown provider: {provider_name}");
        }

        println!("End init module");

        return provider;
    }

}

pub use providers::resolve_provider;
