mod xpln11_provider;


pub mod sensors {

    extern crate yaml_rust;
    use core::fmt;
    use yaml_rust::Yaml;
    use crate::sensors::xpln11_provider;

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

    pub trait  SensorsProvider {

        fn init(&self);
        fn acquire(&self);
        fn shutdown(&self);
    }

    pub fn sensors_init(config: &Yaml) -> Box<dyn SensorsProvider> {

        println!("Start init sensors module");

        let provider: Box<dyn SensorsProvider>;

        let provider_name = config["provider"]["name"].as_str().unwrap();

        if SensorsProviderEnum::XPLN11.to_string().eq(provider_name) {
            println!("Loading X-Plane 11 sensors provider");
            provider = xpln11_provider::xpl11_provider_init(&config) as Box<dyn SensorsProvider>;
        } else {
            panic!("Unknown sensor provider: {provider_name}");
        }

        println!("End init sensors module");

        return provider;
    }
}

pub use sensors::sensors_init;
pub use sensors::SensorsProvider;
