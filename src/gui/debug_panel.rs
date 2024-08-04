
/**
 * Mode Control Panel (MCP)
 * Display graphically : 
 * - Speed/mach (IAS in knots)
 * - Altitude MSL/AGL (Feets)
 * - Bank angle (deg)
 * - Heading (deg)
 * - Vertical speed (feets/min)
 * - Angle Of Attack AoA (deg)
 * - Pitch (deg)
 * - G load (G)
 * - AP Altitude hold target value (feets)
 */
use std::sync::MutexGuard;
use egui::{Ui, Pos2};
use crate::gui::gui::GuiState;
use super::gui_utils;

pub struct DebugPanel {

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

impl DebugPanel {

    pub fn new(position: Pos2, width: f32, height: f32) -> DebugPanel {

        let box_min_x: f32 = position.x;
        let box_max_x: f32 = position.x + width;
        let box_min_y: f32 = position.y;
        let box_max_y: f32 = position.y + height;
        let x_middle_pos: f32 = gui_utils::get_middle_pos(box_min_x, width);
        let y_middle_pos: f32 = gui_utils::get_middle_pos(box_min_y, height);

        DebugPanel{
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

    pub fn view_update(&self, state: &mut MutexGuard<GuiState>, _ctx: &egui::Context, ui: &mut Ui) {

        egui::Frame::none()
            .fill(egui::Color32::LIGHT_GRAY)
            .show(ui, |ui| {

            ui.horizontal(|ui| {

                
                ui.set_height(self.height);
                ui.set_width(self.width);

                let ap_state_alt = state.ap_state.alt.round();
                let alt_msl = state.adc_state.alt_msl.round();
                let alt_agl = state.adc_state.alt_agl.round();
                let ias = state.adc_state.ias.round();
                let vs = state.adc_state.vs.round();
                let heading = state.adc_state.heading.round();
                let pitch = state.adc_state.pitch_angle.round();
                let roll = state.adc_state.roll_angle.round();
                let aoa = state.adc_state.aoa;
                let g_load = state.adc_state.g_load;
                
                ui.label("[DEBUG] ");
                ui.label(format!("AP ALT: {}ft", ap_state_alt));
                ui.label(format!("ALT MSL: {} ft", alt_msl));
                ui.label(format!("ALT AGL: {} ft", alt_agl));
                ui.label(format!("IAS: {} kts", ias));
                ui.label(format!("VS: {} ft/min", vs));
                ui.label(format!("HDG: {}", heading));
                ui.label(format!("Pitch: {}", pitch));
                ui.label(format!("Roll: {}", roll));
                ui.label(format!("AoA: {:.1}", aoa));
                ui.label(format!("G load: {:.1}", g_load));

            });
        });
    }
}

