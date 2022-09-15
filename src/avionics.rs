
mod adc;
mod autopilot;

pub mod avionics {

    use crate::{
        avionics::{adc::{self,Adc}, autopilot::{autopilot, autopilot::Autopilot}}, 
        sensors::SensorsProvider, flight_ctrl::FlightCtrlsProvider};

    pub fn avionics_init(sensors: Box::<dyn SensorsProvider>, flcs: Box::<dyn FlightCtrlsProvider>) -> Avionics {

        println!("Start init avionics module");
        
        let adc: Adc = adc::adc_init(sensors);

        let autopilot: Autopilot = autopilot::autopilot_init(flcs);
        
        adc.ias();

        println!("End init avionics module");

        Avionics {adc, autopilot}
    }

    pub struct Avionics {
        pub adc: Adc,
        pub autopilot: Autopilot,
    }
}

pub use avionics::avionics_init;
