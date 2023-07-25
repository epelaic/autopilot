

use std::sync::{MutexGuard, Arc};

use egui::{Align2, Painter, Ui, Pos2, Color32, Stroke, Shape, Rounding, FontId,
    epaint::RectShape, epaint::Rect, epaint::TextShape, epaint::text::Fonts, epaint::text::FontFamily, epaint::{text::FontDefinitions, CubicBezierShape}, FontData, text::LayoutJob, Galley};
use eframe::{emath::align::Align, epaint::PathShape};

use crate::gui::gui::GuiState;

pub struct AttitudeIndicator { 

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

impl AttitudeIndicator {

    const ANGLES_TO_DRAW: [f32; 44] = [-90.0, -85.0, -80.0, -75.0, -70.0, -65.0, -60.0, -55.0, -50.0, -45.0, -40.0,
        -35.0, -30.0, -25.0, -20.0, -17.5, -15.0, -12.5, -10.0, -7.5, -5.0,-2.5, 
        2.5, 5.0, 7.5, 10.0, 12.5, 15.0, 17.5, 20.0, 25.0, 30.0, 35.0,
        40.0, 45.0, 50.0, 55.0, 60.0, 65.0, 70.0, 75.0, 80.0, 85.0, 90.0];
    
    const  ANGLES_DOT_5: [f32; 8] = [-17.5, -12.5, -7.5, -2.5, 2.5, 7.5, 12.5, 17.5];

    const ANGLES_HALF: [f32; 18] = [-85.0, -75.0, -65.0, -55.0, -45.0, -35.0, -25.0, -15.0, -5.0, 5.0, 15.0, 25.0, 35.0, 45.0, 55.0, 65.0, 75.0, 85.0];

    pub fn new(position: Pos2, width: f32, height: f32) -> AttitudeIndicator {

        let box_min_x: f32 = position.x;
        let box_max_x: f32 = position.x + width;
        let box_min_y: f32 = position.y;
        let box_max_y: f32 = position.y + height;
        let x_middle_pos: f32 = AttitudeIndicator::get_middle_pos(box_min_x, width);
        let y_middle_pos: f32 = AttitudeIndicator::get_middle_pos(box_min_y, height);

        AttitudeIndicator{
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

        let roll_angle: f32 = state.adc_state.roll_angle;
        let pitch_angle: f32 = state.adc_state.pitch_angle;

        // Primary rect (external boundaries)
        let clip_rect: Rect = Rect{
            min: Pos2{x: self.box_min_x, y: self.box_min_y }, 
            max: Pos2{x: self.box_max_x, y: self.box_max_y}
        };

        let cliped_painter: Painter = ui.painter().with_clip_rect(clip_rect);

        let box_rect: RectShape = RectShape { 
            rect: clip_rect, 
            rounding: Rounding::none(), 
            fill: Color32::BLUE, 
            stroke: Stroke { width: 2.0, color: Color32::BROWN } 
        };

        // Call painter to draw objects
        ui.painter().add(Shape::Rect(box_rect));

        self.draw_aircraft_attitude(ui, ctx, roll_angle, pitch_angle, cliped_painter);
        self.draw_aircraft_wings_pos(ui);

    }

    fn draw_aircraft_wings_pos(&self, ui: &mut Ui) {

        // Static left wing (référence)
        let wing_left_rect: RectShape = RectShape { 
            rect: Rect{
                min: Pos2{x: self.box_min_x + 20.0, y: self.y_middle_pos -5.0 }, 
                max: Pos2{x: self.box_min_x + 100.0, y: self.y_middle_pos +5.0}
            }, 
            rounding: Rounding::none(), 
            fill: Color32::BLACK, 
            stroke: Stroke { width: 1.0, color: Color32::WHITE } 
        };

        // Static rigth wing (référence)
        let wing_right_rect: RectShape = RectShape { 
            rect: Rect{
                min: Pos2{x: self.box_max_x - 100.0, y: self.y_middle_pos -5.0 }, 
                max: Pos2{x: self.box_max_x - 20.0, y: self.y_middle_pos +5.0}
            }, 
            rounding: Rounding::none(), 
            fill: Color32::BLACK, 
            stroke: Stroke { width: 1.0, color: Color32::WHITE } 
        };  

        // Static centered mini rect
        let mini_centered_rect: RectShape = RectShape { 
            rect: Rect{
                min: Pos2{x: self.x_middle_pos -5.0, y: self.y_middle_pos -5.0 }, 
                max: Pos2{x: self.x_middle_pos +5.0, y: self.y_middle_pos +5.0}
            }, 
            rounding: Rounding::none(), 
            fill: Color32::TRANSPARENT, 
            stroke: Stroke { width: 2.0, color: Color32::WHITE } 
        };

        ui.painter().add(Shape::Rect(wing_left_rect));  
        ui.painter().add(Shape::Rect(mini_centered_rect)); 
        ui.painter().add(Shape::Rect(wing_right_rect)); 

    }

    fn draw_aircraft_attitude(&self, ui: &mut Ui, ctx: &egui::Context, roll_angle: f32, pitch_angle: f32, cliped_painter: Painter) {

        let roll_angle_in_radians: f32 = rust_math::trigonometry::deg2rad(roll_angle);
        let view_visible_angles: f32 = 60.0;

        // Calc y offet pitch
        let pitch_line_y_offset: f32 = pitch_angle * self.height / view_visible_angles;
        let pitch_line_y_pos: f32 = pitch_line_y_offset + self.y_middle_pos;

        let rotation_axis: Pos2 = Pos2 { x: self.x_middle_pos, y: self.y_middle_pos };

        // Draw ground attitude
        let ground_rect_vec: Vec<Pos2> = AttitudeIndicator::build_path_shape_rect(self.x_middle_pos - 250.0, pitch_line_y_pos, 500.0, self.height + 150.0 );
        let ground_rect_vec_mut: &mut Vec<Pos2> = &mut ground_rect_vec.to_owned();
        AttitudeIndicator::rotate_vec_pos2(rotation_axis, roll_angle_in_radians, ground_rect_vec_mut);
        let ground_rect: PathShape = PathShape{points: ground_rect_vec_mut.to_vec(), closed: true, fill: Color32::BROWN, stroke: Stroke::NONE };

        cliped_painter.add(ground_rect);

        // Draw horizon line attitude
        let attitude_line_pos: &mut [Pos2; 2] = &mut [Pos2{x: self.box_min_x * -1.5, y: pitch_line_y_pos}, Pos2{x: self.box_max_x * 1.5, y: pitch_line_y_pos}];
        
        //println!("before rotate : {:?}", attitude_line_pos);
        
        AttitudeIndicator::rotate_line(rotation_axis, roll_angle_in_radians, attitude_line_pos);
        
        //println!("after rotate : {:?}", attitude_line_pos);
        
        let attitude_line_shape: Shape = Shape::line_segment(*attitude_line_pos, Stroke { width: 1.0, color: Color32::WHITE } );
    
        cliped_painter.add(attitude_line_shape);

        // Draw reference pitch angles with attitude
        for agl in AttitudeIndicator::ANGLES_TO_DRAW.iter() {

            let rectified_agl: f32 = agl * -1.0;
            let agl_pitch_line_y_offset: f32 = rectified_agl * self.height / view_visible_angles;
            let agl_pitch_line_y_pos: f32 = agl_pitch_line_y_offset + pitch_line_y_pos;

            let mut min_x: f32 = 50.0;
            let mut max_x: f32 = 50.0;
            let mut draw_angle_label: bool = true;

            if AttitudeIndicator::ANGLES_DOT_5.contains(agl) {
                min_x = 10.0;
                max_x = 10.0;
                draw_angle_label = false;

            } else if AttitudeIndicator::ANGLES_HALF.contains(agl) {
                min_x = 20.0;
                max_x = 20.0;
                draw_angle_label = false;
            }

            let agl_attitude_line_pos: &mut [Pos2; 2] = &mut [Pos2{x: self.x_middle_pos - min_x, y: agl_pitch_line_y_pos}, Pos2{x: self.x_middle_pos + max_x, y: agl_pitch_line_y_pos}];
            
            AttitudeIndicator::rotate_line(rotation_axis, roll_angle_in_radians, agl_attitude_line_pos);
            
            let agl_attitude_line_shape: Shape = Shape::line_segment(*agl_attitude_line_pos, Stroke { width: 1.0, color: Color32::WHITE } );
        
            cliped_painter.add(agl_attitude_line_shape);

            if draw_angle_label {

                let text_label = agl.to_string();
                let y_anchor_pos: f32 = agl_pitch_line_y_pos - 5.0;

                // Draw left
                let left_x_anchor_pos: f32 = self.x_middle_pos - min_x - 20.0;
                self.draw_attitude_ref_angle_label(ui,&cliped_painter, ctx, text_label.clone(), y_anchor_pos, left_x_anchor_pos, Align::RIGHT, roll_angle_in_radians, rotation_axis);

                // Draw right
                let right_x_anchor_pos: f32 = self.x_middle_pos + max_x + 30.0;
                self.draw_attitude_ref_angle_label(ui,&cliped_painter, ctx, text_label.clone(), y_anchor_pos, right_x_anchor_pos, Align::RIGHT, roll_angle_in_radians, rotation_axis);
            }
        }

        self.draw_bank_angle_ref(ui, rotation_axis, roll_angle_in_radians);

        // Draw axis rotation circle ref (debug purposes)
        //cliped_painter.add(Shape::circle_filled(rotation_axis, 10.0, Color32::GREEN));

    }

    fn draw_attitude_ref_angle_label(
        &self, ui: &mut Ui, cliped_painter: &Painter, ctx: &egui::Context, 
        text_label: String, agl_pitch_line_y_pos: f32, x_anchor_pos: f32, 
        anchor: Align, roll_angle_in_radians: f32, rotation_axis: Pos2) {

        
        let font_id: FontId = FontId::new(10.0, FontFamily::Monospace);
        let mut layout_job: LayoutJob = LayoutJob::simple_singleline(text_label, font_id, Color32::WHITE);
        layout_job.halign = anchor;

        let galley = ctx.fonts(|f| {

            f.layout_job(layout_job)
        });
        
        let mut pos: Pos2 = Pos2{x: x_anchor_pos, y: agl_pitch_line_y_pos};

        let (xc1, yc1) = AttitudeIndicator::rotate_pos2(rotation_axis, roll_angle_in_radians, pos);
        pos = Pos2{x: xc1, y: yc1};

        let text_shape: TextShape = TextShape { pos, galley, underline: Stroke::NONE, override_text_color: None, angle: roll_angle_in_radians * -1.0 };
        
        cliped_painter.add(text_shape);
        
        
    }

    fn draw_bank_angle_ref(&self, ui: &mut Ui, rotation_axis: Pos2, roll_angle_in_radians: f32) {

        // Blue background
        let base_point: Pos2 = Pos2{x: self.x_middle_pos, y: self.box_min_y + 15.0};
        let mut bgr: Vec<Pos2> = Vec::new();
        bgr.push(base_point.to_owned());

        let agl_p: Vec<f32> = [10.0, 20.0, 30.0, 45.0, 60.0].to_vec();

        for a in agl_p.iter() {
            let base_p: &mut Pos2 = &mut base_point.to_owned();
            let angle_in_radians: f32 = rust_math::trigonometry::deg2rad(*a);
            let r: (f32, f32) = AttitudeIndicator::rotate_pos2(rotation_axis, angle_in_radians, *base_p);
            bgr.push(Pos2{x: r.0, y: r.1});
        }

        // 60° left border
        bgr.push(Pos2{x: self.box_min_x, y: self.box_min_y + 60.0 });
        // Top left corner
        bgr.push(Pos2{x: self.box_min_x, y: self.box_min_y });
        // Top rigth corner
        bgr.push(Pos2{x: self.box_max_x, y: self.box_min_y });
        // 60° right border
        bgr.push(Pos2{x: self.box_max_x, y: self.box_min_y + 60.0 });

        let agl_n: Vec<f32> = [-60.0, -45.0, -30.0, -20.0, -10.0].to_vec();

        for a in agl_n.iter() {
            let base_p: &mut Pos2 = &mut base_point.to_owned();
            let angle_in_radians: f32 = rust_math::trigonometry::deg2rad(*a);
            let r: (f32, f32) = AttitudeIndicator::rotate_pos2(rotation_axis, angle_in_radians, *base_p);
            bgr.push(Pos2{x: r.0, y: r.1});
        }

        let bgr_shape: PathShape = PathShape { points: bgr, closed: true, fill: Color32::BLUE, stroke: Stroke{ width: 2.0, color: Color32::BLUE } };
        ui.painter().add(bgr_shape);

        // 0° reverse triangle reference
        let mut trg: Vec<Pos2> = Vec::new();
        trg.push(Pos2{x: self.x_middle_pos - 15.0, y: self.box_min_y});
        trg.push(Pos2{x: self.x_middle_pos + 15.0, y: self.box_min_y});
        trg.push(Pos2{x: self.x_middle_pos, y: self.box_min_y + 15.0});

        let trg_shape: PathShape = PathShape { points: trg, closed: true, fill: Color32::WHITE, stroke: Stroke::NONE };
        ui.painter().add(trg_shape);

        // Bank angle triangle attitude
        let mut bank: Vec<Pos2> = Vec::new();
        bank.push(Pos2{x: self.x_middle_pos - 15.0, y: self.box_min_y + 30.0});
        bank.push(Pos2{x: self.x_middle_pos + 15.0, y: self.box_min_y + 30.0});
        bank.push(Pos2{x: self.x_middle_pos, y: self.box_min_y + 15.0});

        AttitudeIndicator::rotate_vec_pos2(rotation_axis, roll_angle_in_radians, &mut bank);

        let bank_shape: PathShape = PathShape { points: bank, closed: true, fill: Color32::YELLOW, stroke: Stroke::NONE };
        ui.painter().add(bank_shape);

        // +10° line
        let line_small: [Pos2; 2] = [Pos2{x: self.x_middle_pos, y: self.box_min_y + 7.5}, Pos2{x: self.x_middle_pos, y: self.box_min_y + 15.0}];
        let line_big: [Pos2; 2] = [Pos2{x: self.x_middle_pos, y: self.box_min_y}, Pos2{x: self.x_middle_pos, y: self.box_min_y + 15.0}];

        self.draw_rotated_line(ui, rotation_axis, &mut line_small.to_owned(), 10.0);
        self.draw_rotated_line(ui, rotation_axis, &mut line_small.to_owned(), -10.0);
        self.draw_rotated_line(ui, rotation_axis, &mut line_small.to_owned(), 20.0);
        self.draw_rotated_line(ui, rotation_axis, &mut line_small.to_owned(), -20.0);
        self.draw_rotated_line(ui, rotation_axis, &mut line_big.to_owned(), 30.0);
        self.draw_rotated_line(ui, rotation_axis, &mut line_big.to_owned(), -30.0);
        self.draw_rotated_line(ui, rotation_axis, &mut line_small.to_owned(), 45.0);
        self.draw_rotated_line(ui, rotation_axis, &mut line_small.to_owned(), -45.0);
        self.draw_rotated_line(ui, rotation_axis, &mut line_small.to_owned(), 60.0);
        self.draw_rotated_line(ui, rotation_axis, &mut line_small.to_owned(), -60.0);

    }

    fn draw_rotated_line(&self, ui: &mut Ui, rotation_axis: Pos2, points: &mut [Pos2; 2], angle_in_degrees: f32) {

        AttitudeIndicator::rotate_line(rotation_axis, rust_math::trigonometry::deg2rad(angle_in_degrees), points);
        ui.painter().add(Shape::line(points.to_vec(), Stroke { width: 2.0, color: Color32::WHITE }));
    }

    fn get_middle_pos(position_min: f32, width_or_height:f32) -> f32 {

        return position_min + (width_or_height / 2.0);
    }

    fn rotate_line(rotation_axis: Pos2, roll_angle_in_radians: f32, pos: &mut[Pos2; 2]) {


        let (xc1, yc1) = AttitudeIndicator::rotate_pos2(rotation_axis, roll_angle_in_radians, pos[0]);

        let (xc2, yc2) = AttitudeIndicator::rotate_pos2(rotation_axis, roll_angle_in_radians, pos[1]);

        *pos = [Pos2{x: xc1, y: yc1}, Pos2{x: xc2, y: yc2}];
    }

    fn rotate_pos2(rotation_axis: Pos2, roll_angle_in_radians: f32, pos: Pos2) -> (f32, f32){

        let xo: f32 = rotation_axis.x;
        let yo: f32 = rotation_axis.y;
        
        let x_m: f32 = pos.x - xo;
        let y_m: f32 = pos.y - yo;

        let xc: f32 = x_m * roll_angle_in_radians.cos() + y_m * roll_angle_in_radians.sin() + xo;
        let yc: f32 = - x_m * roll_angle_in_radians.sin() + y_m * roll_angle_in_radians.cos() + yo;
        //println!("xc : {}, yc : {}", xc, yc);

        (xc, yc)
    }

    fn build_path_shape_rect(x: f32, y: f32, width: f32, height: f32) -> Vec<Pos2> {

        let mut result: Vec<Pos2> = Vec::new();
        result.push(Pos2{x: x, y: y});
        result.push(Pos2{x: x + width, y: y});
        result.push(Pos2{x: x + width, y: y + height});
        result.push(Pos2{x: x, y: y + height});

        result
    }

    fn rotate_vec_pos2(rotation_axis: Pos2, roll_angle_in_radians: f32, vec: &mut Vec<Pos2>) {

        for p in vec.iter_mut() {

            let (x1, y1) = AttitudeIndicator::rotate_pos2(rotation_axis, roll_angle_in_radians, *p);
            p.x = x1;
            p.y = y1;
        }
    }

}