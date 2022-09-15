
/**
 * ADC for Air Data Computer
 * Provide centralized calculated values from différents Air sensors like pitot probe,
 * static probe, AOA probe, accelerometers, etc...
 * - IAS (Knots)
 * - Altitude (Feets)
 * - Mach number (mach)
 * - AOA (deg)
 * - Vertical speed (feets/min)
 * - G Load factor (Gs)
 */

use crate::{ sensors::SensorsProvider };

pub struct  Adc {
    registry: AdcRegistry,
    sensors: Box::<dyn SensorsProvider>,
}

impl Adc {

    fn get_frame(&self) -> AdcRegistry {

        self.sensors.acquire();

        return self.registry.clone();
    }

    fn dump_registry_state_to_console(&self) {

        println!("AdcRegistry state : ias: {}, alt: {},vs: {}, aoa: {}, mach: {}, g_load : {}", 
            self.registry.ias, 
            self.registry.alt,
            self.registry.vs,
            self.registry.aoa,
            self.registry.mach,
            self.registry.g_load
        );
    }

    pub fn ias(&self) -> f32 {
        return self.registry.ias;
    }

    pub fn alt(&self) -> f32 {
        return self.registry.alt;
    }

    pub fn vs(&self) -> f32 {
        return self.registry.vs;
    }

    pub fn aoa(&self) -> f32 {
        return self.registry.aoa;
    }

    pub fn mach(&self) -> f32 {
        return self.registry.mach;
    }

    pub fn g_load(&self) -> f32 {
        return self.registry.g_load;
    }
}



#[derive(Clone)]
pub struct AdcRegistry {

    ias: f32,
    alt: f32,
    vs: f32,
    aoa: f32,
    mach: f32,
    g_load: f32
}

impl AdcRegistry {

    const fn new() -> AdcRegistry {

        return AdcRegistry {
            ias: 0f32, 
            alt: 0f32,
            vs: 0f32,
            aoa: 0f32,
            mach: 0f32,
            g_load: 0f32,
        };
    }
}

pub fn adc_init(sensors: Box::<dyn SensorsProvider>) -> Adc {
    
    println!("Start init adc module");

    let adc = Adc{registry: AdcRegistry::new(), sensors: sensors};

    adc.dump_registry_state_to_console();

    println!("End init adc module");

    return adc;
}

