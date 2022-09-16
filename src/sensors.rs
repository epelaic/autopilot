pub mod sensors {

    pub trait  SensorsProvider {

        fn acquire(&self) -> SensorsValues;
    }

    pub struct SensorsValues {

        pub ias: f32,
        pub alt: f32,
        pub vs: f32,
        pub aoa: f32,
        pub mach: f32,
        pub g_load: f32
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

        pub const fn from(
                    ias: f32, 
                    alt: f32,
                    vs: f32,
                    aoa: f32,
                    mach: f32,
                    g_load: f32) -> SensorsValues {
    
            return SensorsValues {
                ias: ias, 
                alt: alt,
                vs: vs,
                aoa: aoa,
                mach: mach,
                g_load: g_load,
            };
        }
    }
}

pub use sensors::SensorsProvider;
