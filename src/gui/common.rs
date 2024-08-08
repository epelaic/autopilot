
use std::fmt;

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
/// param current_ap_turn_side: Current AP Turing side
/// param old_value: Current AP Heading selection
/// param knob: Heading selector turn side + amount of headin to add or remove
pub fn get_next_ap_heading_value(current_heading: f32, current_ap_turn_side: APTurnSide, old_value: f32, knob: HeadingKnob) -> Result<(f32, APTurnSide), HeadingValueError> {

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

    // Check if we need to correct the resulting turn side to avoid flip the aircraft 
    // turn when juste reducing the heading target value.
    let mut resulting_turn_side: APTurnSide = current_ap_turn_side;


    if !is_target_heading_between_current_and_old_value(new_value, current_heading, old_value, current_ap_turn_side) {
        
        if desired_turn_side != current_ap_turn_side {
            match current_ap_turn_side {
                APTurnSide::Right => resulting_turn_side = APTurnSide::Left,
                APTurnSide::Left => resulting_turn_side = APTurnSide::Right
            }
        }
    }

    return Ok((new_value, resulting_turn_side));
}

/// Check if the new target heading value is contains inclusive between current heading and the old targeted AP heading
/// according to the current AP turn side.
/// Exemple 1 : current hdg 270°, old target 90°, current turning side by right (clockwise), Reduce target heading to 80°.
///     80° is between 270° and 90° for a right turn (clockwise), so wee can maintain right turn side.
/// Exemple 2 : current hdg 270°, old target 275°, current turning side by right (clockwise), reduce target heading to 265°,
///     265° is not between 270° and 275° for a right turn (clockwise), so wee need to change the turn side to left.
fn is_target_heading_between_current_and_old_value(target_heading:f32, current_heading:f32, old_value: f32, current_ap_turn_side: APTurnSide) -> bool {

    let target_heading_360 = target_heading + MAX_HEADING_VALUE;
    let current_heading_360 = current_heading + MAX_HEADING_VALUE;
    let old_value_360 = old_value + MAX_HEADING_VALUE;

    if current_ap_turn_side == APTurnSide::Right {
        // Case clockwise (turn right)
        
        if target_heading_360 >= current_heading && target_heading_360 <= old_value_360 {
            return true;
        }

    } else {
        // Case counter clockwise (turn left)
        if target_heading_360 <= current_heading_360 && target_heading_360 <= old_value_360 {
            return true;
        }
    }

    return false;
}

/**
 * Test min heading knob value
 * Expected : No heading change, turn right
 */
#[test]
fn test_get_next_ap_heading_value_chdg_0_ctd_right_aphdg_0_knob_right_0() {

    let res1 = get_next_ap_heading_value(0.0, APTurnSide::Right, 0.0, HeadingKnob::Right(0));

    assert_eq!(res1, Ok((0.0, APTurnSide::Right)));
}

/**
 * Test min heading knob value
 * Expected : No heading change, turn left
 */
#[test]
fn test_get_next_ap_heading_value_chdg_0_cts_right_aphdg_0_knob_left_0() {

    let res1 = get_next_ap_heading_value(0.0, APTurnSide::Right, 0.0, HeadingKnob::Left(0));

    assert_eq!(res1, Ok((0.0, APTurnSide::Right)));
}

/**
 * Test outbound heading knob value > 180
 * Expected : Error
 */
#[test]
fn test_get_next_ap_heading_value_chdg_0_cts_right_aphdg_0_knob_right_361() {

    let res1 = get_next_ap_heading_value(0.0, APTurnSide::Right, 0.0, HeadingKnob::Right(361));

    assert_eq!(res1, Err(HeadingValueError{value: 361}));
}

/**
 * Basic turn right
 * Expected : value 1.0, Turn right
 */
#[test]
fn test_get_next_ap_heading_value_chdg_0_cts_right_aphdg_0_knob_right_1() {

    let res1 = get_next_ap_heading_value(0.0, APTurnSide::Right, 0.0, HeadingKnob::Right(1));

    assert_eq!(res1, Ok((1.0, APTurnSide::Right)));
}

/**
 * Basic turn right, start at current heading and ap heading each at 359°, must handle 360° passing value.
 * Expected : value 0.0°, Turn right
 */
#[test]
fn test_get_next_ap_heading_value_chdg_359_cts_right_aphdg_359_knob_right_1() {

    let res1 = get_next_ap_heading_value(359.0, APTurnSide::Right, 359.0, HeadingKnob::Right(1));

    assert_eq!(res1, Ok((0.0, APTurnSide::Right)));
}

/**
 * Basic turn right 180°, start at current heading and ap heading each at 270°.
 * Expected : value 90.0°, Turn right
 */
#[test]
fn test_get_next_ap_heading_value_chdg_270_cts_right_aphdg_270_knob_right_180() {

    let res1 = get_next_ap_heading_value(270.0, APTurnSide::Right, 270.0, HeadingKnob::Right(180));

    assert_eq!(res1, Ok((90.0, APTurnSide::Right)));
}

/**
 * Maintain turn right while reducing heading target gap from current heading when turn heading selector knob to left.
 * Current heading is 270°, last AP heading selected 90°, input selector knob to reduce from 10° (targeted to 80°)
 * Expected : value 80.0°, Turn right
 */
#[test]
fn test_get_next_ap_heading_value_chdg_270_cts_right_aphdg_90_knob_left_10() {

    let res1 = get_next_ap_heading_value(270.0, APTurnSide::Right, 90.0, HeadingKnob::Left(10));

    assert_eq!(res1, Ok((80.0, APTurnSide::Right)));
}

/**
 * Basic turn left 180°, start at current heading and ap heading each at 90°.
 * Expected : value 270.0°, Turn left
 */
#[test]
fn test_get_next_ap_heading_value_chdg_90_cts_left_aphdg_90_knob_right_180() {

    let res1 = get_next_ap_heading_value(90.0, APTurnSide::Left, 90.0, HeadingKnob::Left(180));

    assert_eq!(res1, Ok((270.0, APTurnSide::Left)));
}

#[test]
fn test_is_target_heading_between_current_and_old_value_case_80_270_90_right_true() {

    let result: bool = is_target_heading_between_current_and_old_value(80.0, 270.0, 90.0, APTurnSide::Right);

    assert_eq!(result, true);
}

#[test]
fn test_is_target_heading_between_current_and_old_value_case_0_0_0_right_true() {

    let result: bool = is_target_heading_between_current_and_old_value(80.0, 270.0, 90.0, APTurnSide::Right);

    assert_eq!(result, true);
}

#[test]
fn test_is_target_heading_between_current_and_old_value_case_350_0_10_right_false() {

    let result: bool = is_target_heading_between_current_and_old_value(350.0, 0.0, 10.0, APTurnSide::Right);

    assert_eq!(result, false);
}

#[test]
fn test_is_target_heading_between_current_and_old_value_case_0_90_270_left_true() {

    let result: bool = is_target_heading_between_current_and_old_value(0.0, 90.0, 270.0, APTurnSide::Left);

    assert_eq!(result, true);
}

#[test]
fn test_is_target_heading_between_current_and_old_value_case_0_90_270_right_true() {

    let result: bool = is_target_heading_between_current_and_old_value(80.0, 270.0, 90.0, APTurnSide::Right);

    assert_eq!(result, true);
}
