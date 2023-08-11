use std::ops::Range;
use std::sync::MutexGuard;

use egui::epaint::{RectShape, PathShape};
use egui::{Pos2, Ui, Rect, Painter, Rounding, Color32, Stroke, Shape, Align};
use crate::gui::gui::GuiState;
use crate::gui::gui_utils;

pub struct AltitudeIndicator {

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

impl AltitudeIndicator {

    pub fn new(position: Pos2, width: f32, height: f32) -> AltitudeIndicator {

        let box_min_x: f32 = position.x;
        let box_max_x: f32 = position.x + width;
        let box_min_y: f32 = position.y;
        let box_max_y: f32 = position.y + height;
        let x_middle_pos: f32 = gui_utils::get_middle_pos(box_min_x, width);
        let y_middle_pos: f32 = gui_utils::get_middle_pos(box_min_y, height);

        AltitudeIndicator {
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

        // Primary rect (external boundaries)
        let clip_rect: Rect = Rect{
            min: Pos2{x: self.box_min_x, y: self.box_min_y }, 
            max: Pos2{x: self.box_max_x, y: self.box_max_y}
        };

        let cliped_painter: Painter = ui.painter().with_clip_rect(clip_rect);

        let box_rect: RectShape = RectShape { 
            rect: clip_rect, 
            rounding: Rounding::none(), 
            fill: Color32::GRAY, 
            stroke: Stroke { width: 2.0, color: Color32::GRAY } 
        };

        // Call painter to draw objects
        ui.painter().add(Shape::Rect(box_rect));

        let alt_msl: f32 = state.adc_state.alt_msl;

        self.draw_alt_msl_scale(alt_msl, ctx, &cliped_painter);
        self.draw_alt_msl(alt_msl, ctx, &cliped_painter);
    }

    fn draw_alt_msl_scale(&self, alt_msl: f32, ctx: &egui::Context, cliped_painter: &Painter) {

        const TOTAL_ALT_VISIBLE: f32 = 750.0;
        let alt_msl_i: i32 = alt_msl as i32;
        let max_visible_alt: i32 = alt_msl_i + 400;
        let mut min_visible_alt: i32 = alt_msl_i - 400;

        min_visible_alt = ((min_visible_alt as f32 / 100.0) as i32 * 100) as i32;

        let visible_alt_label_scale: Vec<i32> = AltitudeIndicator::generate_alt_scale(min_visible_alt, max_visible_alt, 100);
    

        let visible_alt_msl_scale: Vec<i32> = visible_alt_label_scale.iter()
                    .filter(|v| v >= &&min_visible_alt && v <= &&max_visible_alt)
                    .map(|v| *v)
                    .collect();
        
        let font_size: f32 = 15.0;

        for a_scale in visible_alt_msl_scale {

            let text_label: String = a_scale.to_string();
            let a_scale_offset_from_alt: f32 = (alt_msl_i - a_scale) as f32;
            let a_scale_y_offset_from_alt: f32 = (a_scale_offset_from_alt * self.height) / TOTAL_ALT_VISIBLE;
            let a_scale_y_pos: f32 = a_scale_y_offset_from_alt + self.y_middle_pos;

            // Altitude scale label
            if (a_scale as i32 / 100) % 2 == 0 {
            
                let pos: Pos2 = Pos2 { x: self.x_middle_pos + 20.0, y: a_scale_y_pos - font_size/2.0 };
                gui_utils::draw_text_label(&cliped_painter, ctx, text_label, 
                                    font_size, Color32::WHITE, Stroke::NONE, 
                                    pos, Align::RIGHT, None);
            }

            // Altitude scale line
            let a_scale_line_pos: [Pos2; 2] = [
                Pos2{x: self.box_min_x + 0.0, y: a_scale_y_pos},
                Pos2{x: self.box_min_x + 10.0, y:a_scale_y_pos}
            ];

            cliped_painter.add(Shape::LineSegment { points: a_scale_line_pos, stroke: Stroke { width: 1.5, color: Color32::WHITE } });
        }

    }

    fn draw_alt_msl(&self, alt_msl: f32, ctx: &egui::Context, cliped_painter: &Painter) {

        // IAS Rect background
        let ias_bg_path_points: Vec<Pos2> = vec![
            Pos2{x: self.x_middle_pos - 30.0, y: self.y_middle_pos - 15.0}, // top left
            Pos2{x: self.x_middle_pos + 38.0, y: self.y_middle_pos - 15.0}, // top right
            Pos2{x: self.x_middle_pos + 38.0, y: self.y_middle_pos + 15.0}, // bottom right
            Pos2{x: self.x_middle_pos - 30.0, y: self.y_middle_pos + 15.0}, // bottom left
            Pos2{x: self.x_middle_pos - 30.0, y: self.y_middle_pos + 5.0}, // cut bottom left
            Pos2{x: self.x_middle_pos - 40.0, y: self.y_middle_pos}, // Edge left
            Pos2{x: self.x_middle_pos - 30.0, y: self.y_middle_pos - 5.0} // cut top left
            ];
        let ias_bg_path = PathShape{points: ias_bg_path_points, closed: true, 
            fill: Color32::BLACK, stroke: Stroke{width: 1.5, color: Color32::YELLOW}};
        cliped_painter.add(ias_bg_path);

        // IAS Text
        let text_label: String = (alt_msl as i32).to_string();
        let font_size: f32 = 20.0;
        let pos: Pos2 = Pos2 { x: self.x_middle_pos + 30.0, y: self.y_middle_pos - font_size/2.0 };
        gui_utils::draw_text_label(&cliped_painter, ctx, text_label, 
                                    font_size, Color32::WHITE, Stroke::NONE, 
                                    pos, Align::RIGHT, None);

    }

    fn generate_alt_scale(start: i32, end: i32, increment: i32) -> Vec<i32> {

        let mut results: Vec<i32> = Vec::new();
        let mut current_value: i32 = start;

        while current_value < end {
            
            results.push(current_value);
            current_value += increment;
        }

        results
    }
}