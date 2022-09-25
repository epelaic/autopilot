use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;

#[derive(FromPrimitive)]
pub enum XPLN11DataReadEnum {
    Frame = 0,
    Speeds = 3,
    MachVviGLoad = 4,
    JoystickYoke = 8,
    PitchRollHeadings = 17,
    AoA = 18,
    MagCompass = 19,
    Gnss = 20,
    ThrottleCmd = 25,
    ThrottleActual = 26,
    N1 = 41,
    N2 = 42,
    ClimbStats = 132
}

#[derive(FromPrimitive)]
pub enum FrameEnum {
    FAct = 0,
    Fsim = 1,
    Frame = 3,
    Cpu1 = 4,
    Cpu2 = 5,
    Gnrd = 6,
    Flit = 7
}

#[derive(FromPrimitive)]
pub enum SpeedsEnum {
    Kias = 0,
    Keas = 1,
    Ktas = 2,
    Ktgs = 3,
    VindMph = 5,
    VTrueMphas = 6
}

#[derive(FromPrimitive)]
pub enum MachVVIGloadEnum {
    Mach = 0,
    Vvi = 2,
    GloadNorm = 4,
    GloadAxial = 5,
    GloadSide = 6,
}

#[derive(FromPrimitive)]
pub enum JoystickYokeEnum {
    Elevator = 0,
    Ailerons = 1,
    Rudder = 2
}

#[derive(FromPrimitive)]
pub enum PitchRollHeadingsEnum {
    Pitch = 0,
    Roll = 1,
    Heading = 2,
    HeadingMag = 3
}

#[derive(FromPrimitive)]
pub enum AoAEnum {
    Alpha = 0,
    Beta = 1,
    Hpath = 2,
    Vpath = 3,
    Slip = 7
}

#[derive(FromPrimitive)]
pub enum MagCompassEnum {
    Mag = 0,
    Mavar = 1
}

#[derive(FromPrimitive)]
pub enum GnssEnum {
    LatitudeDeg = 0,
    LongitudeDeg = 1,

    // Mean Sea Level
    AltitudeFtMSL = 2,

    // Above Ground Level
    AltitudeFtAGL = 3,
    OnRnwy = 4,
    AltitudeInd = 5,
    LatitudeOrigin = 6,
    LongitudeOrigin = 7
}

#[derive(FromPrimitive)]
pub enum ThrottleCmdEnum {
    Thro1 = 0,
    Thro2 = 1
}

#[derive(FromPrimitive)]
pub enum ThrottleActualEnum {
    Thro1 = 0,
    Thro2 = 1
}

#[derive(FromPrimitive)]
pub enum N1Enum {
    N11 = 0,
    N12 = 1
}

#[derive(FromPrimitive)]
pub enum N2Enum {
    N21 = 0,
    N22 = 1
}

#[derive(FromPrimitive)]
pub enum ClimbStatsEnum {
    HSpd = 0,
    VSpd = 1,
    Mult = 3
}

