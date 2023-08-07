use std::sync::MutexGuard;

use egui::epaint::{RectShape, PathShape};
use egui::{Pos2, Ui, Rect, Painter, Rounding, Color32, Stroke, Shape, Align};
use crate::gui::gui::GuiState;
use crate::gui::gui_utils;

pub struct SpeedIndicator {

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

impl SpeedIndicator {
    
    pub fn new(position: Pos2, width: f32, height: f32) -> SpeedIndicator {

        let box_min_x: f32 = position.x;
        let box_max_x: f32 = position.x + width;
        let box_min_y: f32 = position.y;
        let box_max_y: f32 = position.y + height;
        let x_middle_pos: f32 = gui_utils::get_middle_pos(box_min_x, width);
        let y_middle_pos: f32 = gui_utils::get_middle_pos(box_min_y, height);

        SpeedIndicator {
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

        let ias: f32 = state.adc_state.ias;

        self.draw_ias_speed(ias, ctx, cliped_painter);
    }

    fn draw_ias_speed(&self, ias: f32, ctx: &egui::Context, cliped_painter: Painter) {

        // IAS Rect background
        let ias_bg_path_points: Vec<Pos2> = vec![
            Pos2{x: self.x_middle_pos - 35.0, y: self.y_middle_pos - 15.0}, // top left
            Pos2{x: self.x_middle_pos + 5.0, y: self.y_middle_pos - 15.0}, // top right
            Pos2{x: self.x_middle_pos + 5.0, y: self.y_middle_pos - 5.0}, // cut top right
            Pos2{x: self.x_middle_pos + 10.0, y: self.y_middle_pos}, // Edge right
            Pos2{x: self.x_middle_pos + 5.0, y: self.y_middle_pos + 5.0}, // cut bottom right
            Pos2{x: self.x_middle_pos + 5.0, y: self.y_middle_pos + 15.0}, // bottom right
            Pos2{x: self.x_middle_pos - 35.0, y: self.y_middle_pos + 15.0} // bottom left
            ];
        let ias_bg_path = PathShape{points: ias_bg_path_points, closed: true, 
            fill: Color32::BLACK, stroke: Stroke{width: 1.5, color: Color32::YELLOW}};
        cliped_painter.add(ias_bg_path);

        // IAS Text
        let font_size: f32 = 20.0;
        let pos: Pos2 = Pos2 { x: self.x_middle_pos + 2.5, y: self.y_middle_pos - font_size/2.0 };
        gui_utils::draw_text_label(&cliped_painter, ctx, ias.to_string(), 
                                    font_size, Color32::WHITE, Stroke::NONE, 
                                    pos, Align::RIGHT, None);

    }
}