
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

const REGISTRY: AdcRegistry = AdcRegistry::new();


pub struct AdcRegistry {

    ias: i16,
    alt: i16,
    vs: i16,
    aoa: i16,
    mach: i16,
    g_load: i16
}

impl AdcRegistry {

    const fn new() -> AdcRegistry {

        return AdcRegistry {
            ias: 0, 
            alt: 0,
            vs: 0,
            aoa: 0,
            mach: 0,
            g_load: 0,
        };
    }

    pub fn ias(&self) -> i16{
        return self.ias;
    }

    pub fn alt(&self) -> i16 {
        return self.alt;
    }

    pub fn vs(&self) -> i16 {
        return self.vs;
    }

    pub fn aoa(&self) -> i16 {
        return self.aoa;
    }

    pub fn mach(&self) -> i16 {
        return self.mach;
    }

    pub fn g_load(&self) -> i16 {
        return self.g_load;
    }

}

pub fn adc_init() -> AdcRegistry {
    
    println!("Start init adc module");

    //let ias = REGISTRY.ias();
    //let alt: i16 = REGISTRY.alt();

    dump_registry_state_to_console();

    println!("End init adc module");

    return REGISTRY;
}

fn dump_registry_state_to_console() {

    println!("AdcRegistry state : ias: {}, alt: {},vs: {}, aoa: {}, mach: {}, g_load : {}", 
        REGISTRY.ias(), 
        REGISTRY.alt(),
        REGISTRY.vs(),
        REGISTRY.aoa(),
        REGISTRY.mach(),
        REGISTRY.g_load()
    );
}
