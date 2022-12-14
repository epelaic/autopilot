
pub mod bus {
    use std::fmt;

    #[derive(Debug, Clone)]
    pub enum BusMessage {
        AdcData(AdcDataMessage),
        APState(APStateMessage),
        APCmd(APCmdPayload)
    }

    // ADC Messages
    
    #[derive(Debug, Clone)]
    pub struct AdcDataMessage {
        pub ias: f32,
        pub alt_msl: f32,
        pub alt_agl: f32,
        pub heading: f32,
        pub vs: f32,
        pub aoa: f32,
        pub mach: f32,
        pub g_load: f32,
        pub pitch_angle: f32,
        pub roll_angle: f32,
    }

    impl AdcDataMessage {
        
        pub const fn new() -> Self {
            Self {
                ias: 0f32,
                alt_msl: 0f32,
                alt_agl: 0f32,
                heading: 0f32,
                vs: 0f32,
                aoa: 0f32,
                mach: 0f32,
                g_load: 0f32,
                pitch_angle: 0f32,
                roll_angle: 0f32,
            }
        }
    }

    impl fmt::Display for AdcDataMessage {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "ias: {}, alt MSL: {}, alt AGL: {}, hdg: {}, vs: {}, aoa: {}, mach: {}, g_load: {}, pitch: {}, roll: {}", 
            self.ias, self.alt_msl, self.alt_agl, self.heading, self.vs, self.aoa, self.mach, self.g_load, self.pitch_angle, self.roll_angle)
        }
    }

    // AP Messages
    #[derive(Debug, Clone)]
    pub struct APStateMessage {

        pub engaged: bool,

        // Modes AP
        pub alt_hold_mode: bool,
        pub vs_mode: bool,
        pub heading_mode: bool,
        pub auto_throttle_mode: bool,

        // Modes values

        pub alt: f32,
        pub heading: f32,
        pub speed: f32,
        pub speed_unit: SpeedUnit,
        pub bank_angle: f32,
        pub vs: f32,
    }

    impl APStateMessage {

        pub const fn new() -> Self {

            Self{
                engaged:false, 
                alt_hold_mode: false,
                vs_mode: false,
                heading_mode: false,
                auto_throttle_mode: false,
                alt: 15_000f32,
                heading: 180f32,
                speed: 250f32,
                speed_unit: SpeedUnit::IAS,
                bank_angle: 10f32,
                vs: 0f32
            }
        }
    }

    #[derive(Debug, Clone)]
    pub enum  APCmdPayload {

        APEngage(bool),

        // Modes AP
        EnableAltHoldMode(bool),
        EnaleVSMode(bool),
        EnalbeHeadingHoldMode(bool),
        EnableAutoThrottleMode(bool),

        // Modes values

        SetHeading{ heading: u8, turn_side: APTurnSide },
        SetSpeed{ speed: f32, unit: SpeedUnit },
        SetVs(f32),
        SetAlt(f32),
        SetBankAngle(i8)

    }

    #[derive(Debug, Clone)]
    pub enum APTurnSide {
        Left,
        Right
    }

    #[derive(Debug, Clone)]
    pub enum SpeedUnit {
        IAS,
        MACH
    }

    // GUI Messages

}

pub use bus::BusMessage;
pub use bus::AdcDataMessage;
pub use bus::APStateMessage;
pub use bus::SpeedUnit;
pub use bus::APTurnSide;
pub use bus::APCmdPayload;