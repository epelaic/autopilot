
pub mod adc {

    use std::sync::Arc;
    use std::sync::mpsc::Sender;

    use crate::bus::BusMessage;
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
        pub sensors: Arc::<dyn SensorsProvider + Send + Sync>,
        pub adc_tx_gui: Sender<BusMessage>,
        pub adc_tx_ap: Sender<BusMessage>
    }

    impl Adc {

        pub fn read_sensors(&self) {

            let adc_registry: AdcRegistry = self.get_frame();

            let adc_data:AdcDataMessage = adc_registry.to_adc_data();
            let gui_bus_message: BusMessage = BusMessage::AdcData(adc_data);
            let ap_bus_message: BusMessage = gui_bus_message.clone();

            println!("[ADC] sending data...");
            self.adc_tx_gui.send(gui_bus_message).unwrap();
            self.adc_tx_ap.send(ap_bus_message).unwrap();

        }

        pub fn get_frame(&self) -> AdcRegistry {

            let s_values: SensorsValues = self.sensors.acquire();
            self.apply_sensors_values(s_values)
        }

        fn apply_sensors_values(&self, s_values: SensorsValues) -> AdcRegistry {

            AdcRegistry{
                ias: s_values.ias,
                alt: s_values.alt,
                heading: s_values.heading,
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
        heading: f32,
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
                heading: 0f32,
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
                heading: self.heading,
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

