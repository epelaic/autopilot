
pub mod adc;
pub mod autopilot;

pub mod avionics {

    use std::{sync::{mpsc::Sender, Arc}, sync::mpsc::Receiver, time::Duration, thread};

    use crate::{
        avionics::{adc::{self,Adc, AdcRegistry}, autopilot::{autopilot, autopilot::Autopilot}}, 
        sensors::SensorsProvider, flight_ctrl::FlightCtrlsProvider, bus::{BusMessage, AdcDataMessage}};

    pub fn avionics_init(
            sensors: Arc::<dyn SensorsProvider + Send + Sync>, 
            flcs: Arc::<dyn FlightCtrlsProvider + Send + Sync>) -> Avionics {

        println!("Start init avionics module");

        let adc: Arc::<Adc> = Arc::new(adc::adc_init(sensors));

        let autopilot: Autopilot = autopilot::autopilot_init(flcs);

        println!("End init avionics module");

        Avionics {adc: adc, autopilot}
    }

    pub struct Avionics {
        pub adc: Arc::<Adc>,
        pub autopilot: Autopilot,
    }
}

pub use avionics::avionics_init;

