
pub mod adc {

    use std::sync::Arc;

    use crate::sensors::sensors::SensorsValues;
    
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

    use crate::{ sensors::SensorsProvider, bus::{AdcDataMessage} };

    pub struct  Adc {
        pub sensors: Arc::<dyn SensorsProvider + Send + Sync>
    }

    impl Adc {

        pub fn get_frame(&self) -> AdcRegistry {

            let s_values: SensorsValues = self.sensors.acquire();
            self.apply_sensors_values(s_values)
        }

        fn apply_sensors_values(&self, s_values: SensorsValues) -> AdcRegistry {

            AdcRegistry{
                ias: s_values.ias,
                alt: s_values.alt,
                vs: s_values.vs,
                aoa: s_values.aoa,
                mach: s_values.mach,
                g_load: s_values.g_load    
            }
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

        pub fn to_adc_data(&self) -> AdcDataMessage {

            return AdcDataMessage{
                ias: self.ias, 
                alt: self.alt, 
                vs: self.vs, 
                aoa: self.aoa, 
                mach: self.mach, 
                g_load: self.g_load, 
                pitch_angle: 0f32, 
                roll_angle: 0f32
            };
        }
    }

}

