
use crate::bus::APCmdPayload;

pub trait APBusMessageSender {
    
    fn send_ap_cmd(&self, ap_cmd_payload: APCmdPayload);
}

pub fn increment_value(old_value: &mut f32, step: f32, max: f32) {

    let mut new_value: f32 = *old_value + step;

    if new_value > max {
        new_value = max;
    }

    *old_value =  new_value;
}

pub fn decrement_value(old_value: &mut f32, step: f32, min: f32) {

    let mut new_value: f32 = *old_value - step;

    if new_value < min {
        new_value = min;
    }

    *old_value =  new_value;
}
