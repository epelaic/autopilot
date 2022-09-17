
pub mod bus {
    use std::fmt;

    #[derive(Debug, Clone)]
    pub enum BusMessage {
        AdcData(AdcDataMessage),
        APState(APStateMessage),
        APCmd(APCmdMessage)
    }

    // ADC Messages
    
    #[derive(Debug, Clone)]
    pub struct AdcDataMessage {
        pub ias: f32,
        pub alt: f32,
        pub vs: f32,
        pub aoa: f32,
        pub mach: f32,
        pub g_load: f32,
        pub pitch_angle: f32,
        pub roll_angle: f32,
    }

    impl fmt::Display for AdcDataMessage {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "ias: {}, alt: {}, vs: {}, aoa: {}, mach: {}, g_load: {}, pitch: {}, roll: {}", 
            self.ias, self.alt, self.vs, self.aoa, self.mach, self.g_load, self.pitch_angle, self.roll_angle)
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

    #[derive(Debug, Clone)]
    pub struct APCmdMessage {
        payload: APCmd
    }

    #[derive(Debug, Clone)]
    pub enum  APCmd {

        APEngage(bool),

        // Modes AP
        EnableAltHoldMode(bool),
        EnaleVSMode(bool),
        EnalbeHeadingHoldMode(bool),
        EnableAutoThrottleMode(bool),

        // Modes values

        SetHeading{ heading: u8, turn_side: APTurnSide },
        SetSpeed{ speed: f32, unit: SpeedUnit },
        SetVs(u32),
        SetAlt(u32),
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
pub use bus::APCmdMessage;
pub use bus::SpeedUnit;
pub use bus::APTurnSide;