use std::sync::MutexGuard;

use egui::Align;
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

    const ANGLES_TO_DRAW: [f32; 72] = [
        0.0, 5.0, 10.0, 15.0, 20.0, 25.0, 30.0, 35.0, 40.0, 45.0, 50.0, 55.0, 60.0, 65.0, 70.0, 75.0, 80.0, 85.0, 90.0, 95.0,
        100.0, 105.0, 110.0, 115.0, 120.0, 125.0, 130.0, 135.0, 140.0, 145.0, 150.0, 155.0, 160.0, 165.0, 170.0, 175.0, 180.0, 185.0,
        190.0, 195.0, 200.0, 205.0, 210.0, 215.0, 220.0, 225.0, 230.0, 235.0, 240.0, 245.0, 250.0, 255.0, 260.0, 265.0, 270.0, 275.0,
        280.0, 285.0, 290.0, 295.0, 300.0, 305.0, 310.0, 315.0, 320.0, 325.0, 330.0, 335.0, 340.0, 345.0, 350.0, 355.0
    ];

    const ANGLES_LABEL_SMALL: [f32; 24] = [
        10.0, 20.0, 40.0, 50.0, 70.0, 80.0, 
        100.0, 110.0, 130.0, 140.0, 160.0, 170.0, 190.0, 
        200.0, 220.0, 230.0, 250.0, 260.0, 280.0, 290.0, 310.0, 320.0, 340.0, 350.0
    ];

    const ANGLES_LABEL_BIG: [f32; 11] = [
        0.0, 30.0, 60.0, 90.0, 120.0, 150.0, 180.0, 210.0, 240.0, 270.0, 330.0
    ];

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

    pub fn view_update(&self, state: &mut MutexGuard<GuiState>, ctx: &egui::Context, ui: &mut Ui) {

        let heading_angle: f32 = state.adc_state.heading;

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

        self.draw_heading_angles(ui, ctx, heading_angle, heading_disc_center_pos, cliped_painter);

    }

    fn draw_heading_angles(&self, _ui: &mut Ui, ctx: &egui::Context, heading_angle: f32, rotation_axis: Pos2, cliped_painter: Painter) {

        // Draw reference heading angles
        for hdg in HeadingIndicator::ANGLES_TO_DRAW.iter() {
            
            let heading_offset: f32 = (heading_angle - *hdg) * -1.0;

            let min_y: f32 = self.box_min_y + 15.0;
            let mut max_y: f32 = self.box_min_y + 20.0;
            let mut print_label: bool = false;
            let mut font_size: f32 = 10.0;

            if HeadingIndicator::ANGLES_LABEL_BIG.contains(hdg) {
                max_y = self.box_min_y + 30.0;
                print_label = true;
                font_size = 20.0;

            } else if HeadingIndicator::ANGLES_LABEL_SMALL.contains(hdg) {
                max_y = self.box_min_y + 30.0;
                print_label = true;
            }

            let hdg_ref_in_radians: f32 = rust_math::trigonometry::deg2rad(heading_offset) * -1.0;

            let hdg_line_pos: &mut [Pos2; 2] = &mut [Pos2{x: self.x_middle_pos, y: min_y}, Pos2{x: self.x_middle_pos, y: max_y}];

            gui_utils::rotate_line(rotation_axis, hdg_ref_in_radians, hdg_line_pos);

            cliped_painter.add(Shape::line(hdg_line_pos.to_vec(), Stroke { width: 2.0, color: Color32::WHITE }));

            if print_label {
                // We convet to only two digit max label (ex: 180° -> 18°).
                let text_label: String = ((hdg / 10.0) as i32).to_string();

                let mut hdg_label_pos: Pos2 = Pos2{x: self.x_middle_pos, y: self.box_min_y + 30.0};

                let (xc1, yc1) = gui_utils::rotate_pos2(rotation_axis, hdg_ref_in_radians, hdg_label_pos);
                hdg_label_pos = Pos2{x: xc1, y: yc1};

                gui_utils::draw_text_label(&cliped_painter, ctx, text_label, 
                    font_size, Color32::WHITE, Stroke::NONE, 
                    hdg_label_pos, Align::Center, Some(hdg_ref_in_radians));
            }
        }
    }

}