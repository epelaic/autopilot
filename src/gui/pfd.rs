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
    altitude_indicator::AltitudeIndicator, gui_utils::RectWrapper, heading_indicator::HeadingIndicator, speed_indicator::SpeedIndicator
};

pub struct PrimaryFligthDisplay { 

    pub width: f32,
    pub height: f32,
}

impl PrimaryFligthDisplay {

    pub fn new(width: f32, height: f32) -> PrimaryFligthDisplay {

        PrimaryFligthDisplay{
            width,
            height
        }
    }

    pub fn view_update(&self, state: &mut MutexGuard<GuiState>, ctx: &egui::Context, ui: &mut Ui) {


        egui::Frame::none()
            .fill(egui::Color32::GRAY)
            .show(ui, |ui| {

            ui.vertical(|ui| {
                
                ui.set_height(self.height);
                ui.set_width(self.width);

                let position = Pos2{x: ui.max_rect().left_top().x, y: ui.max_rect().left_top().y};

                let pfd_rect_wraper: RectWrapper = RectWrapper::new(position, self.width, self.height);

                // Primary rect (external boundaries)
                let clip_rect: Rect = Rect{
                    min: Pos2{x: pfd_rect_wraper.box_min_x, y: pfd_rect_wraper.box_min_y }, 
                    max: Pos2{x: pfd_rect_wraper.box_max_x, y: pfd_rect_wraper.box_max_y}
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
                    Pos2{x: pfd_rect_wraper.box_min_x + 15.0, y: pfd_rect_wraper.box_min_y + 60.0},
                    75.0,
                    400.0);

                let attitude_indicator: AttitudeIndicator = AttitudeIndicator::new(
                    Pos2{x: pfd_rect_wraper.box_min_x + 95.0, y: pfd_rect_wraper.box_min_y + 116.0},
                    300.0,
                    300.0);

                let altitude_indicator: AltitudeIndicator = AltitudeIndicator::new(
                    Pos2{x: pfd_rect_wraper.box_max_x - 100.0, y: pfd_rect_wraper.box_min_y + 60.0},
                    75.0,
                    400.0);
                
                let heading_indicator: HeadingIndicator = HeadingIndicator::new(
                    Pos2{x: pfd_rect_wraper.box_min_x + 95.0, y: pfd_rect_wraper.box_min_y + 500.0},
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


