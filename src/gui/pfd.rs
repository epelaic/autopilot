/**
 * PFD for Primary Flight Display
 * Display graphically : 
 * - speed (IAS in knots)
 * - Altitude (Feets)
 * - Bank angle (deg)
 * - Pitch angle (deg)
 * - Vertical speed (feets/min)
 */


use std::sync::MutexGuard;

use egui::Ui;

use crate::gui::gui::GuiState;

pub struct PrimaryFligthDisplay { }

impl PrimaryFligthDisplay {

    pub fn view_update(&self, state: &mut MutexGuard<GuiState>, _ctx: &egui::Context, ui: &mut Ui) {

        ui.horizontal(|ui| {

            ui.label(format!("ALT MSL: {}ft", state.adc_state.alt_msl));
            ui.label(format!("IAS: {}kts", state.adc_state.ias));
            ui.label(format!("VS: {}ft/min", state.adc_state.vs));
            ui.label(format!("HDG: {}", state.adc_state.heading));
            ui.label(format!("Pitch: {}", state.adc_state.pitch_angle));
            ui.label(format!("Roll: {}", state.adc_state.roll_angle));
        });
    }
}


