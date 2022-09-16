pub mod sensors {

    pub trait  SensorsProvider {

        fn acquire(&self) -> SensorsValues;
    }

    pub struct SensorsValues {

        ias: f32,
        alt: f32,
        vs: f32,
        aoa: f32,
        mach: f32,
        g_load: f32
    }
    
    impl SensorsValues {
    
        pub const fn new() -> SensorsValues {
    
            return SensorsValues {
                ias: 0f32, 
                alt: 0f32,
                vs: 0f32,
                aoa: 0f32,
                mach: 0f32,
                g_load: 0f32,
            };
        }
    }
}

pub use sensors::SensorsProvider;
