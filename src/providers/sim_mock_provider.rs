
use yaml_rust::Yaml;
use crate::sensors::SensorsProvider;
use crate::flight_ctrl::FlightCtrlsProvider;
use crate::sensors::sensors::SensorsValues;
use super::providers::Provider;

const NAME: &str = "SIMMOCK";

pub struct SimMockProvider {

}

impl Provider for SimMockProvider {

    fn name(&self) -> &str {

        return NAME;
    }


    fn init(&mut self) {
        println!("SimMock Provider",);
    }

    fn shutdown(&self) {
        println!("SimMock Provider shutdown");
    }

    fn get_sensors(&self) -> Box::<dyn SensorsProvider> {

        Box::new(SimMockSensorsProvider{})
    }

    fn get_flcs(&self) -> Box::<dyn FlightCtrlsProvider> {

        Box::new(SimMockFlightCtrlsProvider{})
    }
}

struct SimMockSensorsProvider {

}

impl SensorsProvider for SimMockSensorsProvider {

    fn acquire(&self) -> SensorsValues {
        println!("SimMock Provider acquire");
        SensorsValues::from(
            250f32,
            10.000f32,
            0.0f32,
            5.0f32,
            0.5f32,
            1.0f32)
    }
}

struct  SimMockFlightCtrlsProvider {

}

impl FlightCtrlsProvider for SimMockFlightCtrlsProvider {

    fn send(&self) {
        println!("SimMock Provider send");
    }
}

pub fn simmock_provider_init(config: &Yaml) -> Box<dyn Provider> {

    println!("Start init simmock_provider module");

    let provider = Box::new(SimMockProvider{});

    println!("End init simmock_provider module");

    return provider;
}