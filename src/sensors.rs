pub mod sensors {

    pub trait  SensorsProvider {

        fn acquire(&self) -> SensorsValues;
    }

    pub struct SensorsValues {

        ias: i16,
        alt: i16,
        vs: i16,
        aoa: i16,
        mach: i16,
        g_load: i16
    }
    
    impl SensorsValues {
    
        pub const fn new() -> SensorsValues {
    
            return SensorsValues {
                ias: 0, 
                alt: 0,
                vs: 0,
                aoa: 0,
                mach: 0,
                g_load: 0,
            };
        }
    }
}

pub use sensors::SensorsProvider;
