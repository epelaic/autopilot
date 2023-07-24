

use std::sync::{MutexGuard, Arc};

use egui::{Align2, Painter, Ui, Pos2, Color32, Stroke, Shape, Rounding, FontId,
    epaint::RectShape, epaint::Rect, epaint::TextShape, epaint::text::Fonts, epaint::text::FontFamily, epaint::text::FontDefinitions, FontData};
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
        
        //self.load_fonts(ctx);

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
            fill: Color32::BLACK, 
            stroke: Stroke { width: 1.0, color: Color32::WHITE } 
        };

        ui.painter().add(Shape::Rect(wing_left_rect));  
        ui.painter().add(Shape::Rect(mini_centered_rect)); 
        ui.painter().add(Shape::Rect(wing_right_rect)); 

    }

    fn draw_aircraft_attitude(&self, ui: &mut Ui, ctx: &egui::Context, roll_angle: f32, pitch_angle: f32, cliped_painter: Painter) {

        let view_visible_angles: f32 = 60.0;

        // Calc y offet pitch
        let pitch_line_y_offset: f32 = pitch_angle * self.height / view_visible_angles;
        let pitch_line_y_pos: f32 = pitch_line_y_offset + self.y_middle_pos;

        // Draw ground attitude
        let ground_rect: RectShape = RectShape { 
            rect: Rect{
                min: Pos2{x: self.box_min_x - 100.0, y: pitch_line_y_pos }, 
                max: Pos2{x: self.box_max_x + 100.0, y: pitch_line_y_pos + self.height + 50.0}
            }, 
            rounding: Rounding::none(), 
            fill: Color32::BROWN, 
            stroke: Stroke { width: 1.0, color: Color32::WHITE } 
        };

        cliped_painter.add(Shape::Rect(ground_rect));

        // Draw horizon line attitude
        let attitude_line_pos: [Pos2; 2] = [Pos2{x: self.box_min_x, y: pitch_line_y_pos}, Pos2{x: self.box_max_x, y: pitch_line_y_pos}];
        let attitude_line_shape: Shape = Shape::line_segment(attitude_line_pos, Stroke { width: 1.0, color: Color32::WHITE } );
    
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

            let agl_attitude_line_pos: [Pos2; 2] = [Pos2{x: self.x_middle_pos - min_x, y: agl_pitch_line_y_pos}, Pos2{x: self.x_middle_pos + max_x, y: agl_pitch_line_y_pos}];
            let agl_attitude_line_shape: Shape = Shape::line_segment(agl_attitude_line_pos, Stroke { width: 1.0, color: Color32::WHITE } );
        
            cliped_painter.add(agl_attitude_line_shape);

            // if draw_angle_label {

            //     let text_label = agl.to_string();
            //     let x_anchor_pos: f32 = self.x_middle_pos - min_x;
            //     let anchor: Align2  = Align2::RIGHT_CENTER;

            //     self.draw_attitude_ref_angle_label(&cliped_painter, ctx, text_label, agl_pitch_line_y_pos, x_anchor_pos, anchor);
            // }
        }
    }

    // fn draw_attitude_ref_angle_label(&self, cliped_painter: &Painter, ctx: &egui::Context, text_label: String, agl_pitch_line_y_pos: f32, x_anchor_pos: f32, anchor: Align2) {

    //     let fonts_defs: Box<FontDefinitions> = self.load_fonts(ctx);
    //     let fd: FontDefinitions = *fonts_defs; 

    //     for f in fd.families.iter() {
    //         println!("Font ctx a : {:?}", f);
    //     }

    //     let fonts: Fonts = Fonts::new(ctx.pixels_per_point(), 1024, fd);


    //     for f in fonts.families().iter() {
    //         println!("Font ctx b: {:?}", f);
    //     }

    //     fonts.begin_frame(ctx.pixels_per_point(), 1024);

    //     cliped_painter.add(Shape::text(
    //         &fonts, 
    //         Pos2{x: x_anchor_pos, y: agl_pitch_line_y_pos}, 
    //         anchor, 
    //         text_label, 
    //         FontId::new(30.0, FontFamily::Name("FreeMono".into())), 
    //         Color32::WHITE));

    //     fonts.font_image_delta();
        
    // }

    fn get_middle_pos(position_min: f32, width_or_height:f32) -> f32 {

        return position_min + (width_or_height / 2.0);
    }

    // fn load_fonts(&self, ctx: &egui::Context) -> Box<FontDefinitions> {

    //     // Install my own font:
    //     println!("Installing custom fonts");
        
    //     let mut fonts_def = FontDefinitions::default();

    //     for f in fonts_def.families.iter() {
    //         println!("Font ctx 1 : {:?}", f);
    //     }

    //     fonts_def.font_data.insert("FreeMono".to_owned(), FontData::from_static(include_bytes!("../../fonts/FreeMono.ttf")));
    //     fonts_def.families.insert(egui::FontFamily::Name("FreeMono".into()), vec!["FreeMono".to_owned()]);

    //     // Put my font first (highest priority):
    //     fonts_def.families.get_mut(&FontFamily::Proportional).unwrap()
    //         .insert(0, "FreeMono".to_owned());
        
    //     // Put my font as last fallback for monospace:
    //     fonts_def.families.get_mut(&FontFamily::Monospace).unwrap()
    //         .insert(0, "FreeMono".to_owned());

    //     for f in fonts_def.families.iter() {
    //         println!("Font ctx 2 : {:?}", f);
    //     }

    //     //ctx.set_fonts(fonts_def.clone());

    //     return Box::new(fonts_def);

    // }

}