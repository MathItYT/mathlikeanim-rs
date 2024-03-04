use crate::{colors::{Color, GradientImageOrColor}, objects::vector_object::{VectorFeatures, VectorObject}, utils::interpolate_color};

pub fn set_fill_animation(target_fill: (f64, f64, f64, f64)) -> impl Fn(VectorFeatures, f64) -> VectorFeatures {
    move |vec_obj: VectorFeatures, t: f64| {
        let mut vec_obj = vec_obj;
        match vec_obj.get_fill() {
            GradientImageOrColor::Color(color) => {
                let color = (color.red, color.green, color.blue, color.alpha);
                let new_color = interpolate_color(color, target_fill, t);
                vec_obj = vec_obj.set_fill(GradientImageOrColor::Color(Color {
                    red: new_color.0,
                    green: new_color.1,
                    blue: new_color.2,
                    alpha: new_color.3,
                }), true);
            },
            _ => {}
        }
        return vec_obj;
    }
}