use std::sync::MutexGuard;

use egui::{Pos2, Ui, Rect, Painter, epaint::RectShape, Rounding, Color32, Stroke, Shape, TextureId};
use egui::epaint::PathShape;

use super::{gui_utils, gui::GuiState};


pub struct HeadingIndicator {

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

impl HeadingIndicator {

    pub fn new(position: Pos2, width: f32, height: f32) -> HeadingIndicator {

        let box_min_x: f32 = position.x;
        let box_max_x: f32 = position.x + width;
        let box_min_y: f32 = position.y;
        let box_max_y: f32 = position.y + height;
        let x_middle_pos: f32 = gui_utils::get_middle_pos(box_min_x, width);
        let y_middle_pos: f32 = gui_utils::get_middle_pos(box_min_y, height);

        HeadingIndicator {
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

    pub fn view_update(&self, _state: &mut MutexGuard<GuiState>, _ctx: &egui::Context, ui: &mut Ui) {

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
            fill_texture_id: TextureId::Managed(0),
            uv: Rect::ZERO
        };

        // Call painter to draw objects
        ui.painter().add(Shape::Rect(box_rect));

        // heading disc
        let heading_disc_center_pos: Pos2 = Pos2{x: self.x_middle_pos, y: self.box_min_y + (self.width / 2.0) + 15.0};
        let heading_disc_shape: Shape = Shape::circle_filled(heading_disc_center_pos, self.width / 2.0, Color32::GRAY);
        cliped_painter.add(heading_disc_shape);

        // 0° reverse triangle reference
        let mut trg: Vec<Pos2> = Vec::new();
        trg.push(Pos2{x: self.x_middle_pos - 15.0, y: self.box_min_y});
        trg.push(Pos2{x: self.x_middle_pos + 15.0, y: self.box_min_y});
        trg.push(Pos2{x: self.x_middle_pos, y: self.box_min_y + 15.0});

        let trg_shape: PathShape = PathShape { points: trg, closed: true, fill: Color32::BLACK, stroke: Stroke { width: 2.0, color: Color32::WHITE }};
        cliped_painter.add(trg_shape);

        // Vertical white middle line
        let vertical_middle_line_pos: [Pos2; 2] = [
            Pos2{x: self.x_middle_pos, y: self.box_min_y + 15.0},
            Pos2{x: self.x_middle_pos, y: self.box_max_y}
        ];
        let vertical_middle_line_shape: Shape = Shape::line_segment(vertical_middle_line_pos, Stroke { width: 2.0, color: Color32::WHITE } );
        cliped_painter.add(vertical_middle_line_shape);

    }
}