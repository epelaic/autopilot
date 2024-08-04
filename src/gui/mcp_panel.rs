
/**
 * Mode Control Panel (MCP)
 * Display graphically : 
 * - AP Engage/disengage
 * - AP Altitude Hold Mode, target value and selectors
 * - Bank angle selector
 * - Heading selector (and turn direction holder)
 * - Vertical speed Mode, target value and selectors)
 * - Speed Mode (Knots/Mach) toogle, target value and selectors)
 */
use std::sync::MutexGuard;
use egui::{Pos2, Ui};
use crate::bus::APCmdPayload;
use crate::gui::gui::GuiState;
use crate::gui::constants::{ALT_100_STEP_VALUE, ALT_500_STEP_VALUE, ALT_MAX_VALUE, ALT_MIN_VALUE};
use crate::gui::common::{decrement_value, increment_value};

use super::common::APBusMessageSender;
use super::gui_utils;
pub struct ModeControlPanel {

    pub position: Pos2,
    pub width: f32,
    pub height: f32,

    box_min_x: f32,
    box_max_x: f32,
    box_min_y: f32,
    box_max_y: f32,
    x_middle_pos: f32,
    y_middle_pos: f32,
 }

impl ModeControlPanel {

    pub fn new(position: Pos2, width: f32, height: f32) -> ModeControlPanel {

        let box_min_x: f32 = position.x;
        let box_max_x: f32 = position.x + width;
        let box_min_y: f32 = position.y;
        let box_max_y: f32 = position.y + height;
        let x_middle_pos: f32 = gui_utils::get_middle_pos(box_min_x, width);
        let y_middle_pos: f32 = gui_utils::get_middle_pos(box_min_y, height);

        ModeControlPanel{
            position,
            width,
            height,
            box_min_x,
            box_max_x,
            box_min_y,
            box_max_y,
            x_middle_pos,
            y_middle_pos
        }
    }

    pub fn view_update(&self, state: &mut MutexGuard<GuiState>, _ctx: &egui::Context, ui: &mut Ui, ap_msg_sender: &dyn APBusMessageSender) {

        egui::Frame::none()
            .fill(egui::Color32::LIGHT_BLUE)
            .show(ui, |ui| {

            ui.set_width(self.width);
            ui.set_height(self.height);

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
        });
    }
}

