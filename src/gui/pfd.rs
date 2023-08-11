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

use egui::{Ui, Pos2, epaint::RectShape, Painter, Rect, Rounding, Color32, Stroke, Shape};

use crate::gui::attitude_indicator::AttitudeIndicator;
use crate::gui::gui::GuiState;

use super::{speed_indicator::SpeedIndicator, gui_utils, altitude_indicator::{self, AltitudeIndicator}};

pub struct PrimaryFligthDisplay { 

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

impl PrimaryFligthDisplay {

    pub fn new(position: Pos2, width: f32, height: f32) -> PrimaryFligthDisplay {

        let box_min_x: f32 = position.x;
        let box_max_x: f32 = position.x + width;
        let box_min_y: f32 = position.y;
        let box_max_y: f32 = position.y + height;
        let x_middle_pos: f32 = gui_utils::get_middle_pos(box_min_x, width);
        let y_middle_pos: f32 = gui_utils::get_middle_pos(box_min_y, height);

        PrimaryFligthDisplay{
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

            // Primary rect (external boundaries)
            let clip_rect: Rect = Rect{
                min: Pos2{x: self.box_min_x, y: self.box_min_y }, 
                max: Pos2{x: self.box_max_x, y: self.box_max_y}
            };
            
            let cliped_painter: Painter = ui.painter().with_clip_rect(clip_rect);

            let box_rect: RectShape = RectShape { 
                rect: clip_rect, 
                rounding: Rounding::none(), 
                fill: Color32::BLACK, 
                stroke: Stroke { width: 2.0, color: Color32::BLACK } 
            };

            // Call painter to draw objects
            cliped_painter.add(Shape::Rect(box_rect));

            let speed_indicator: SpeedIndicator = SpeedIndicator::new(
                Pos2{x: self.box_min_x + 15.0, y: self.box_min_y + 60.0},
                75.0,
                400.0);

            let attitude_indicator: AttitudeIndicator = AttitudeIndicator::new(
                Pos2{x: self.box_min_x + 95.0, y: self.box_min_y + 116.0},
                300.0,
                300.0);

            let altitude_indicator: AltitudeIndicator = AltitudeIndicator::new(
                Pos2{x: self.box_max_x - 100.0, y: self.box_min_y + 60.0},
                75.0,
                400.0);

            speed_indicator.view_update(state, ctx, ui);
            attitude_indicator.view_update(state, ctx, ui);
            altitude_indicator.view_update(state, ctx, ui);
        });
    }
}


