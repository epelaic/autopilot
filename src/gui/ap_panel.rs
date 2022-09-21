
/**
 * Autopilot Panel
 * Display graphically : 
 * - speed/mach (IAS in knots)
 * - Altitude (Feets)
 * - Bank angle (deg) (deg)
 * - Vertical speed (feets/min)
 */
use std::sync::MutexGuard;
use egui::Ui;
use crate::bus::APCmdPayload;
use crate::gui::gui::GuiState;
use crate::gui::constants::{ALT_100_STEP_VALUE, ALT_500_STEP_VALUE, ALT_MAX_VALUE, ALT_MIN_VALUE};
use crate::gui::common::{decrement_value, increment_value};

use super::common::APBusMessageSender;
pub struct AutopilotPanel {
 }

impl AutopilotPanel {

    pub fn view_update(&self, state: &mut MutexGuard<GuiState>, _ctx: &egui::Context, ui: &mut Ui, ap_msg_sender: &dyn APBusMessageSender) {

        ui.horizontal(|ui| {

            if ui.button("<<").clicked() {

                decrement_value(&mut state.ap_state.alt , crate::gui::constants::ALT_500_STEP_VALUE, ALT_MIN_VALUE);
                ap_msg_sender.send_ap_cmd(APCmdPayload::SetAlt(state.ap_state.alt));
            }

            if ui.button("<").clicked() {

                decrement_value(&mut state.ap_state.alt , ALT_100_STEP_VALUE, ALT_MIN_VALUE);
                ap_msg_sender.send_ap_cmd(APCmdPayload::SetAlt(state.ap_state.alt));
            }

            ui.label(format!("AP alt: {}ft", state.ap_state.alt));

            if ui.button(">").clicked() {

                increment_value(&mut state.ap_state.alt , ALT_100_STEP_VALUE, ALT_MAX_VALUE);
                ap_msg_sender.send_ap_cmd(APCmdPayload::SetAlt(state.ap_state.alt));
            }

            if ui.button(">>").clicked() {

                increment_value(&mut state.ap_state.alt , ALT_500_STEP_VALUE, ALT_MAX_VALUE);
                ap_msg_sender.send_ap_cmd(APCmdPayload::SetAlt(state.ap_state.alt));
            }

        });
    }
}

