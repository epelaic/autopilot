pub mod sensors {

    pub trait  SensorsProvider {

        fn acquire(&self) -> SensorsValues;
    }

    pub struct SensorsValues {

        pub ias: f32,
        pub alt_msl: f32,
        pub alt_agl: f32,
        pub heading: f32,
        pub vs: f32,
        pub aoa: f32,
        pub mach: f32,
        pub g_load: f32,
        pub pitch: f32,
        pub roll: f32,
        pub yaw: f32
    }
    
    impl SensorsValues {
    
        pub const fn new() -> SensorsValues {
    
            return SensorsValues {
                ias: 0f32, 
                alt_msl: 0f32,
                alt_agl: 0f32,
                heading: 0f32,
                vs: 0f32,
                aoa: 0f32,
                mach: 0f32,
                g_load: 0f32,
                pitch: 0f32,
                roll: 0f32,
                yaw: 0f32
            };
        }

        pub const fn from(
                    ias: f32, 
                    alt_msl: f32,
                    alt_agl: f32,
                    heading: f32,
                    vs: f32,
                    aoa: f32,
                    mach: f32,
                    g_load: f32,
                    pitch: f32,
                    roll: f32,
                    yaw: f32) -> SensorsValues {
    
            return SensorsValues {
                ias: ias, 
                alt_msl: alt_msl,
                alt_agl: alt_agl,
                heading: heading,
                vs: vs,
                aoa: aoa,
                mach: mach,
                g_load: g_load,
                pitch: pitch,
                roll: roll,
                yaw: yaw
            };
        }
    }
}

pub use sensors::SensorsProvider;
