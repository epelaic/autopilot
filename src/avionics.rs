
mod adc;
mod autopilot;

pub mod avionics {

    use crate::avionics::{adc::{self, AdcRegistry}, autopilot};

    pub fn avionics_init() {

        println!("Start init avionics module");
        
        let adc_registry: AdcRegistry = adc::adc_init();
        autopilot::autopilot_init();
        
        adc_registry.ias();

        println!("End init avionics module");
    }
}

pub use avionics::avionics_init;
