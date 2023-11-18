use egui::{Painter, Align, Pos2, FontId, FontFamily, text::LayoutJob, Color32, epaint::TextShape, Stroke};



pub fn get_middle_pos(position_min: f32, width_or_height:f32) -> f32 {

    return position_min + (width_or_height / 2.0);
}

pub fn rotate_pos2(rotation_axis: Pos2, roll_angle_in_radians: f32, pos: Pos2) -> (f32, f32){

    let xo: f32 = rotation_axis.x;
    let yo: f32 = rotation_axis.y;
    
    let x_m: f32 = pos.x - xo;
    let y_m: f32 = pos.y - yo;

    let xc: f32 = x_m * roll_angle_in_radians.cos() + y_m * roll_angle_in_radians.sin() + xo;
    let yc: f32 = - x_m * roll_angle_in_radians.sin() + y_m * roll_angle_in_radians.cos() + yo;
    //println!("xc : {}, yc : {}", xc, yc);

    (xc, yc)
}

pub fn rotate_line(rotation_axis: Pos2, roll_angle_in_radians: f32, pos: &mut[Pos2; 2]) {


    let (xc1, yc1) = rotate_pos2(rotation_axis, roll_angle_in_radians, pos[0]);

    let (xc2, yc2) = rotate_pos2(rotation_axis, roll_angle_in_radians, pos[1]);

    *pos = [Pos2{x: xc1, y: yc1}, Pos2{x: xc2, y: yc2}];
}


pub fn draw_text_label(
    cliped_painter: &Painter, ctx: &egui::Context, 
    text_label: String, font_size: f32, font_color: Color32, font_stroke: Stroke, pos: Pos2, 
    anchor: Align, angle_in_radians: Option<f32>) {

    
    let font_id: FontId = FontId::new(font_size, FontFamily::Monospace);
    let mut layout_job: LayoutJob = LayoutJob::simple_singleline(text_label, font_id, font_color);
    layout_job.halign = anchor;

    let galley = ctx.fonts(|f| {

        f.layout_job(layout_job)
    });

    let angle: f32 = match angle_in_radians {
        Some(val) => val,
        None => 0.0f32
    };

    let text_shape: TextShape = TextShape { pos, galley, underline: font_stroke, override_text_color: None, angle: angle * -1.0 };
    
    cliped_painter.add(text_shape);
}