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

use egui::{Ui, Pos2};

use crate::gui::attitude_indicator::AttitudeIndicator;
use crate::gui::gui::GuiState;

pub struct PrimaryFligthDisplay { }

impl PrimaryFligthDisplay {

    pub fn view_update(&self, state: &mut MutexGuard<GuiState>, ctx: &egui::Context, ui: &mut Ui) {

        ui.vertical(|ui| {

            let alt_msl = state.adc_state.alt_msl.round();
            let alt_agl = state.adc_state.alt_agl.round();
            let ias = state.adc_state.ias.round();
            let vs = state.adc_state.vs.round();
            let heading = state.adc_state.heading.round();
            let pitch = state.adc_state.pitch_angle.round();
            let roll = state.adc_state.roll_angle.round();
            let aoa = state.adc_state.aoa;
            let g_load = state.adc_state.g_load;

            ui.label(format!("ALT MSL: {} ft", alt_msl));
            ui.label(format!("ALT AGL: {} ft", alt_agl));
            ui.label(format!("IAS: {} kts", ias));
            ui.label(format!("VS: {} ft/min", vs));
            ui.label(format!("HDG: {}", heading));
            ui.label(format!("Pitch: {}", pitch));
            ui.label(format!("Roll: {}", roll));
            ui.label(format!("AoA: {:.1}", aoa));
            ui.label(format!("G load: {:.1}", g_load));

            let attitude_indicator: AttitudeIndicator = AttitudeIndicator::new(
                Pos2{x: 250.0, y: 250.0},
                300.0,
                300.0);

            attitude_indicator.view_update(state, ctx, ui);
        });
    }
}


