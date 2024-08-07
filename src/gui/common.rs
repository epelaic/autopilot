
use std::{error::Error, fmt};

use num_traits::ToPrimitive;

use crate::bus::{APCmdPayload, APTurnSide};

pub trait APBusMessageSender {
    
    fn send_ap_cmd(&self, ap_cmd_payload: APCmdPayload);
}

//TODO refact to return a new value
pub fn increment_value(old_value: &mut f32, step: f32, max: f32) {

    let mut new_value: f32 = *old_value + step;

    if new_value > max {
        new_value = max;
    }

    *old_value =  new_value;
}

//TODO refact to return a new value
pub fn decrement_value(old_value: &mut f32, step: f32, min: f32) {

    let mut new_value: f32 = *old_value - step;

    if new_value < min {
        new_value = min;
    }

    *old_value =  new_value;
}

const MAX_HEADING_SELECTOR_VALUE: u32 = 180;
const MAX_HEADING_VALUE: f32 = 360.0;
const MIN_HEADING_VALUE: f32 = 0.0;
const HALF_HEADING: f32 = 180.0;

#[derive(Debug, Clone, PartialEq)]
pub enum HeadingKnob {
    Right(u32),
    Left(u32),
    None()
}

#[derive(Debug, Clone, PartialEq)]
pub struct HeadingValueError {
    pub value: u32
}

impl fmt::Display for HeadingValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid heading parameter value: {}, min: 0, max: 180", self.value)
    }
}

/// 
/// param current_heading: Current Aircraft Heading (not used for now)
/// param old_value: Current AP Heading selection
/// param knob: Heading selector turn side + amount of headin to add or remove
pub fn get_next_ap_heading_value(_current_heading: f32, old_value: f32, knob: HeadingKnob) -> Result<(f32, APTurnSide), HeadingValueError> {

    let mut desired_turn_side: APTurnSide = APTurnSide::Right;
    let mut new_value: f32 = old_value;
    let mut step: f32 = 0.0;

    match knob {
        HeadingKnob::Right(value) => { step = value.to_f32().unwrap(); new_value = new_value + step; desired_turn_side = APTurnSide::Right},
        HeadingKnob::Left(value) => { step = value.to_f32().unwrap(); new_value = new_value - step; desired_turn_side = APTurnSide::Left },
        HeadingKnob::None() => ()
    }

    if step > MAX_HEADING_SELECTOR_VALUE.to_f32().unwrap() {
        return Err(HeadingValueError{ value: step.to_u32().unwrap()});
    }

    if new_value >= MAX_HEADING_VALUE {
        new_value = new_value - MAX_HEADING_VALUE;
    } else if new_value < MIN_HEADING_VALUE {
        new_value = MAX_HEADING_VALUE + new_value;
    }

    return Ok((new_value, desired_turn_side));
}

/**
 * Test min heading knob value
 * Expected : No heading change, turn right
 */
#[test]
fn test_get_next_ap_heading_value_chdg_0_aphdg_0_knob_right_0() {

    let res1 = get_next_ap_heading_value(0.0, 0.0, HeadingKnob::Right(0));

    assert_eq!(res1, Ok((0.0, APTurnSide::Right)));
}

/**
 * Test min heading knob value
 * Expected : No heading change, turn left
 */
#[test]
fn test_get_next_ap_heading_value_chdg_0_aphdg_0_knob_left_0() {

    let res1 = get_next_ap_heading_value(0.0, 0.0, HeadingKnob::Left(0));

    assert_eq!(res1, Ok((0.0, APTurnSide::Left)));
}

/**
 * Test outbound heading knob value > 180
 * Expected : Error
 */
#[test]
fn test_get_next_ap_heading_value_chdg_0_aphdg_0_knob_right_361() {

    let res1 = get_next_ap_heading_value(0.0, 0.0, HeadingKnob::Right(361));

    assert_eq!(res1, Err(HeadingValueError{value: 361}));
}

/**
 * Basic turn right
 * Expected : value 1.0, Turn right
 */
#[test]
fn test_get_next_ap_heading_value_chdg_0_aphdg_0_knob_right_1() {

    let res1 = get_next_ap_heading_value(0.0, 0.0, HeadingKnob::Right(1));

    assert_eq!(res1, Ok((1.0, APTurnSide::Right)));
}

/**
 * Basic turn right, start at current heading and ap heading each at 359°, must handle 360° passing value.
 * Expected : value 0.0°, Turn right
 */
#[test]
fn test_get_next_ap_heading_value_chdg_359_aphdg_359_knob_right_1() {

    let res1 = get_next_ap_heading_value(359.0, 359.0, HeadingKnob::Right(1));

    assert_eq!(res1, Ok((0.0, APTurnSide::Right)));
}

/**
 * Basic turn right 180°, start at current heading and ap heading each at 270°.
 * Expected : value 90.0°, Turn right
 */
#[test]
fn test_get_next_ap_heading_value_chdg_270_aphdg_270_knob_right_180() {

    let res1 = get_next_ap_heading_value(270.0, 270.0, HeadingKnob::Right(180));

    assert_eq!(res1, Ok((90.0, APTurnSide::Right)));
}

/**
 * Basic turn left 180°, start at current heading and ap heading each at 90°.
 * Expected : value 270.0°, Turn left
 */
#[test]
fn test_get_next_ap_heading_value_chdg_90_aphdg_90_knob_right_180() {

    let res1 = get_next_ap_heading_value(90.0, 90.0, HeadingKnob::Left(180));

    assert_eq!(res1, Ok((270.0, APTurnSide::Left)));
}
