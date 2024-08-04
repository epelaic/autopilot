/**
 * PFD for Primary Flight Display
 * Display graphically : 
 * - Speed (IAS in knots)
 * - Altitude (feets)
 * - Bank angle (deg)
 * - Pitch angle (deg)
 * - Vertical speed (feets/min)
 * - Heading (deg)
 */


use std::sync::MutexGuard;

use egui::{Ui, Pos2, epaint::RectShape, Painter, Rect, Rounding, Color32, Stroke, Shape, TextureId};

use crate::gui::attitude_indicator::AttitudeIndicator;
use crate::gui::gui::GuiState;

use super::{
    gui_utils,
    speed_indicator::SpeedIndicator, 
    altitude_indicator::AltitudeIndicator,
    heading_indicator::HeadingIndicator
};

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

        egui::Frame::none()
            .fill(egui::Color32::GRAY)
            .show(ui, |ui| {

            ui.vertical(|ui| {
                
                ui.set_height(self.height);
                ui.set_width(self.width);

                // Primary rect (external boundaries)
                let clip_rect: Rect = Rect{
                    min: Pos2{x: self.box_min_x, y: self.box_min_y }, 
                    max: Pos2{x: self.box_max_x, y: self.box_max_y}
                };
                
                let cliped_painter: Painter = ui.painter().with_clip_rect(clip_rect);

                let box_rect: RectShape = RectShape { 
                    rect: clip_rect, 
                    rounding: Rounding::ZERO, 
                    fill: Color32::BLACK, 
                    stroke: Stroke { width: 2.0, color: Color32::BLACK },
                    blur_width: 0f32,
                    fill_texture_id: TextureId::Managed(0),
                    uv: Rect::ZERO
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
                
                let heading_indicator: HeadingIndicator = HeadingIndicator::new(
                    Pos2{x: self.box_min_x + 95.0, y: self.box_min_y + 500.0},
                    300.0,
                    100.0);

                speed_indicator.view_update(state, ctx, ui);
                attitude_indicator.view_update(state, ctx, ui);
                altitude_indicator.view_update(state, ctx, ui);
                heading_indicator.view_update(state, ctx, ui);
            });
        });
    }
}


