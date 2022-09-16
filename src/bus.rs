
pub mod bus {
    use std::fmt;


    pub enum BusMessage {
        AdcData(AdcDataMessage),
        APState(APStateMessage),
        APCmd(APCmdMessage)
    }

    // ADC Messages
    
    #[derive(Debug)]
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
    #[derive(Debug)]
    pub struct APStateMessage {

        engaged: bool,

        // Modes AP
        alt_hold_mode: bool,
        vs_mode: bool,
        heading_mode: bool,
        auto_throttle_mode: bool,

        // Modes values

        alt: f32,
        heading: f32,
        speed: f32,
        speed_unit: SpeedUnit,
        bank_angle: f32,
        vs: f32,
    }

    #[derive(Debug)]
    pub struct APCmdMessage {
        payload: APCCmd
    }

    #[derive(Debug)]
    pub enum  APCCmd {

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

    #[derive(Debug)]
    pub enum APTurnSide {
        Left,
        Right
    }

    #[derive(Debug)]
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