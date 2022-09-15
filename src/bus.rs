
pub mod bus {

    pub enum BusMessage {
        AdcData(AdcDataMessage),
        APState(APStateMessage),
        APCmd(APCmdMessage)
    }

    // ADC Messages
    pub struct AdcDataMessage {
        ias: f32,
        alt: f32,
        vs: f32,
        aoa: f32,
        mach: f32,
        g_load: f32,
        pitch_angle: f32,
        rool_angle: f32,
    }

    // AP Messages
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

    pub struct APCmdMessage {
        payload: APCCmd
    }

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

    pub enum APTurnSide {
        Left,
        Right
    }

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