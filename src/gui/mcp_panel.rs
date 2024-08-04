
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
use egui::{Button, Color32, Pos2, RichText, Ui};
use crate::bus::APCmdPayload;
use crate::gui::gui::GuiState;
use crate::gui::constants::{ALT_100_STEP_VALUE, ALT_500_STEP_VALUE, ALT_MAX_VALUE, ALT_MIN_VALUE};
use crate::gui::common::{decrement_value, increment_value};

use super::common::APBusMessageSender;

pub struct ModeControlPanel {

    pub width: f32,
    pub height: f32
 }

impl ModeControlPanel {

    pub fn new(width: f32, height: f32) -> ModeControlPanel {

        ModeControlPanel{
            width,
            height
        }
    }

    pub fn view_update(&self, state: &mut MutexGuard<GuiState>, _ctx: &egui::Context, ui: &mut Ui, ap_msg_sender: &dyn APBusMessageSender) {

        egui::Frame::none()
            .fill(egui::Color32::LIGHT_BLUE)
            .show(ui, |ui| {

            ui.set_width(self.width);
            ui.set_height(self.height);

            egui::Frame::none()
            .fill(egui::Color32::GRAY)
            .show(ui, |ui| {
            
                ui.vertical(|ui| {

                    ui.set_width(150.0);
                    ui.set_height(self.height);
                    
                    ui.label(RichText::new("Altitude").color(Color32::WHITE));
                    ui.label(RichText::new(format!("{} ft", state.ap_state.alt)).color(Color32::GREEN));

                    ui.horizontal(|ui| {

                        if ui.button("<<").clicked() {

                            decrement_value(&mut state.ap_state.alt , crate::gui::constants::ALT_500_STEP_VALUE, ALT_MIN_VALUE);
                            ap_msg_sender.send_ap_cmd(APCmdPayload::SetAlt(state.ap_state.alt));
                        }

                        if ui.button("<").clicked() {

                            decrement_value(&mut state.ap_state.alt , ALT_100_STEP_VALUE, ALT_MIN_VALUE);
                            ap_msg_sender.send_ap_cmd(APCmdPayload::SetAlt(state.ap_state.alt));
                        }

                        if ui.button(">").clicked() {

                            increment_value(&mut state.ap_state.alt , ALT_100_STEP_VALUE, ALT_MAX_VALUE);
                            ap_msg_sender.send_ap_cmd(APCmdPayload::SetAlt(state.ap_state.alt));
                        }

                        if ui.button(">>").clicked() {

                            increment_value(&mut state.ap_state.alt , ALT_500_STEP_VALUE, ALT_MAX_VALUE);
                            ap_msg_sender.send_ap_cmd(APCmdPayload::SetAlt(state.ap_state.alt));
                        }

                    });

                    let mut alt_hold_color: Color32 = Color32::DARK_GRAY;
                    let mut alt_hold_action: bool = true;

                    if state.ap_state.alt_hold_mode {
                        alt_hold_color = Color32::LIGHT_GREEN;
                        alt_hold_action = false;
                    } else {
                        alt_hold_color = Color32::WHITE;
                    }
                    
                    let alt_hold_button = Button::new(RichText::new("ALT HLD").color(alt_hold_color))
                                                                    .fill(Color32::DARK_GRAY);
                    
                    if ui.add(alt_hold_button).clicked() {

                        ap_msg_sender.send_ap_cmd(APCmdPayload::EnableAltHoldMode(alt_hold_action));
                    }
                });
            });
        });
    }
}

