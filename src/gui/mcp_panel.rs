
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
use crate::bus::{APCmdPayload, APTurnSide};
use crate::gui::gui::GuiState;
use crate::gui::constants::{ALT_100_STEP_VALUE, ALT_500_STEP_VALUE, ALT_MAX_VALUE, ALT_MIN_VALUE};
use crate::gui::common::{decrement_value, increment_value, get_next_ap_heading_value, APBusMessageSender, HeadingKnob};

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
            
            ui.horizontal(|ui| {
                ui.set_width(self.width);
                ui.set_height(self.height);
            
                
                //------ MCP SPEED WIDGET ------//

                //------ HEADING SEL WIDGET ------//
                egui::Frame::none()
                    .fill(egui::Color32::GRAY)
                    .show(ui, |ui| {
                    
                    ui.vertical(|ui| {

                        ui.set_width(150.0);
                        ui.set_height(self.height);
                        
                        ui.label(RichText::new("HEADING").color(Color32::WHITE));
                        ui.label(RichText::new(state.ap_state.heading.to_string()).color(Color32::GREEN));


                        ui.horizontal(|ui| {

                            let mut bank_angle_10_btn_color: Color32 = Color32::WHITE;

                            if state.ap_state.bank_angle == 10.0 {
                                bank_angle_10_btn_color = Color32::LIGHT_GREEN;
                            }
                        
                            let bank_angle_10_btn = Button::new(RichText::new("10").color(bank_angle_10_btn_color))
                                                                        .fill(Color32::DARK_GRAY);
                            if ui.add(bank_angle_10_btn).clicked() {
                                ap_msg_sender.send_ap_cmd(APCmdPayload::SetBankAngle(10));
                            }

                            let mut bank_angle_20_btn_color: Color32 = Color32::WHITE;

                            if state.ap_state.bank_angle == 20.0 {
                                bank_angle_20_btn_color = Color32::LIGHT_GREEN;
                            }
                        
                            let bank_angle_20_btn = Button::new(RichText::new("20").color(bank_angle_20_btn_color))
                                                                        .fill(Color32::DARK_GRAY);
                            if ui.add(bank_angle_20_btn).clicked() {
                                ap_msg_sender.send_ap_cmd(APCmdPayload::SetBankAngle(20));
                            }

                            let mut bank_angle_30_btn_color: Color32 = Color32::WHITE;

                            if state.ap_state.bank_angle == 30.0 {
                                bank_angle_30_btn_color = Color32::LIGHT_GREEN;
                            }
                        
                            let bank_angle_30_btn = Button::new(RichText::new("30").color(bank_angle_30_btn_color))
                                                                        .fill(Color32::DARK_GRAY);
                            if ui.add(bank_angle_30_btn).clicked() {
                                ap_msg_sender.send_ap_cmd(APCmdPayload::SetBankAngle(30));
                            }

                        }); // End horizontal

                        ui.horizontal(|ui| {
                            
                            let mut knob: HeadingKnob = HeadingKnob::None();
                            
                            if ui.button("<<").clicked() {
                                knob = HeadingKnob::Left(10);
                            }

                            if ui.button("<").clicked() {
                                knob = HeadingKnob::Left(1);
                            }

                            if ui.button(">").clicked() {
                                knob = HeadingKnob::Right(1);
                            }

                            if ui.button(">>").clicked() {
                                knob = HeadingKnob::Right(10);
                            }

                            if knob != HeadingKnob::None() {
                                let new_heading_value_result = get_next_ap_heading_value(state.adc_state.heading, state.ap_state.heading, knob);
                                
                                if new_heading_value_result.is_ok() {
                                    let new_heading_value = new_heading_value_result.ok().unwrap();
                                    ap_msg_sender.send_ap_cmd(APCmdPayload::SetHeading { heading: new_heading_value.0, turn_side: new_heading_value.1 });
                                } else if new_heading_value_result.is_err() {
                                    print!("Heading knob error : {}", new_heading_value_result.err().unwrap());
                                }
                            }

                        }); // End horizontal
                    }); // End vertical
                }); // End Frame

                //------ ALTITUDE HOLD WIDGET ------//
                egui::Frame::none()
                .fill(egui::Color32::GRAY)
                .show(ui, |ui| {
                
                    ui.vertical(|ui| {

                        ui.set_width(150.0);
                        ui.set_height(self.height);
                        
                        ui.label(RichText::new("ALTITUDE").color(Color32::WHITE));
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

                        }); // End horizontal

                        let mut alt_hold_color: Color32 = Color32::WHITE;
                        let mut alt_hold_action: bool = true;

                        if state.ap_state.alt_hold_mode {
                            alt_hold_color = Color32::LIGHT_GREEN;
                            alt_hold_action = false;
                        }
                        
                        let alt_hold_button = Button::new(RichText::new("ALT HLD").color(alt_hold_color))
                                                                        .fill(Color32::DARK_GRAY);
                        
                        if ui.add(alt_hold_button).clicked() {

                            ap_msg_sender.send_ap_cmd(APCmdPayload::EnableAltHoldMode(alt_hold_action));
                        }
                    }); // End vertical
                }); // End Frame

                //------ VERTICAL SPEED WIDGET ------//

                //------ AP ENGAGE WIDGET ------//
            });
        });
    }
}

