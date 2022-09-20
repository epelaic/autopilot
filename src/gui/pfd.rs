/**
 * PFD for Primary Flight Display
 * Display graphically : 
 * - speed (IAS in knots)
 * - Altitude (Feets)
 * - Bank angle (deg)
 * - Pitch angle (deg)
 * - Vertical speed (feets/min)
 */
pub mod pfd {

    use std::sync::MutexGuard;

    use egui::Ui;

    use crate::gui::gui::GuiState;


    pub fn pfd_update(state: MutexGuard<GuiState>, ctx: &egui::Context, ui: &mut Ui) {

        ui.horizontal(|ui| {

            ui.label(format!("ALT: {}ft", state.adc_state.alt));
            ui.label(format!("IAS: {}kts", state.adc_state.ias));
            ui.label(format!("VS: {}ft/min", state.adc_state.vs));
            ui.label(format!("HDG: {}", state.adc_state.heading));
        });
    }
}

pub use pfd::pfd_update;