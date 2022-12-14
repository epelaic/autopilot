
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

            match self.sensors.acquire() {
                Ok(s_values) => {

                    let adc_registry: AdcRegistry = self.apply_sensors_values(s_values);

                    let adc_data:AdcDataMessage = adc_registry.to_adc_data();
                    let gui_bus_message: BusMessage = BusMessage::AdcData(adc_data);
                    let ap_bus_message: BusMessage = gui_bus_message.clone();
        
                    //println!("[ADC] sending data...");
                    self.adc_tx_gui.send(gui_bus_message).unwrap();
                    self.adc_tx_ap.send(ap_bus_message).unwrap();

                },
                Err(e) => println!("Acquire error : {:?}", e)
            }

        }

        fn apply_sensors_values(&self, s_values: SensorsValues) -> AdcRegistry {

            AdcRegistry{
                ias: s_values.ias,
                alt_msl: s_values.alt_msl,
                alt_agl: s_values.alt_agl,
                heading: s_values.heading,
                vs: s_values.vs,
                aoa: s_values.aoa,
                mach: s_values.mach,
                g_load: s_values.g_load,
                pitch: s_values.pitch,
                roll: s_values.roll
            }
        }

    }

    #[derive(Clone)]
    pub struct AdcRegistry {

        ias: f32,
        alt_msl: f32,
        alt_agl: f32,
        heading: f32,
        vs: f32,
        aoa: f32,
        mach: f32,
        g_load: f32,
        pitch: f32,
        roll: f32
    }

    impl AdcRegistry {

        const fn new() -> AdcRegistry {

            return AdcRegistry {
                ias: 0f32, 
                alt_msl: 0f32,
                alt_agl: 0f32,
                heading: 0f32,
                vs: 0f32,
                aoa: 0f32,
                mach: 0f32,
                g_load: 0f32,
                pitch: 0f32,
                roll: 0f32
            };
        }

        pub fn to_adc_data(&self) -> AdcDataMessage {

            return AdcDataMessage{
                ias: self.ias, 
                alt_msl: self.alt_msl,
                alt_agl: self.alt_agl, 
                heading: self.heading,
                vs: self.vs, 
                aoa: self.aoa, 
                mach: self.mach, 
                g_load: self.g_load, 
                pitch_angle: self.pitch, 
                roll_angle: self.roll
            };
        }
    }

}

