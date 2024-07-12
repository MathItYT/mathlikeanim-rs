use js_sys::{Array, Function, Reflect};
use wasm_bindgen::prelude::*;

use crate::colors::{Color, GradientImageOrColor, GradientStop, Image, LinearGradient, RadialGradient};

use super::{geometry::{add_tip::{add_both_sides_tips, add_final_tip, add_initial_tip}, arc::{annular_sector, arc, circle, ellipse, elliptical_arc}, line::line, poly::{equilateral_triangle, polygon, rectangle, regular_polygon, right_triangle, square, triangle}}, plotting::{axes::{area_under_curve, axes, coords_to_point, contour_plot_in_axes, parametric_plot_in_axes, plot_in_axes, point_to_coords, riemann_rectangles_for_plot, secant_line_for_plot}, functions::{function, contour_plot, parametric_function}, number_line::{get_numbers_tex, number_line, number_to_point, point_to_number}}, svg_to_vector::svg_to_vector_pin, vector_object::{VectorFeatures, VectorObject}};


#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct WasmGradientImageOrColor {
    #[wasm_bindgen(skip)]
    pub gradient_image_or_color: GradientImageOrColor
}


impl JsCast for WasmGradientImageOrColor {
    fn instanceof(val: &JsValue) -> bool {
        // Check if the objects has all getters to avoid recursion errors
        Reflect::get(&val, &JsValue::from_str("isColor")).is_ok() &&
        Reflect::get(&val, &JsValue::from_str("isLinearGradient")).is_ok() &&
        Reflect::get(&val, &JsValue::from_str("isRadialGradient")).is_ok() &&
        Reflect::get(&val, &JsValue::from_str("isImage")).is_ok() &&
        Reflect::get(&val, &JsValue::from_str("getColor")).is_ok() &&
        Reflect::get(&val, &JsValue::from_str("getLinearGradient")).is_ok() &&
        Reflect::get(&val, &JsValue::from_str("getRadialGradient")).is_ok() &&
        Reflect::get(&val, &JsValue::from_str("getImage")).is_ok()
    }
    fn unchecked_from_js(val: JsValue) -> Self {
        let is_color_func = Reflect::get(&val, &JsValue::from_str("isColor")).unwrap();
        let is_linear_gradient_func = Reflect::get(&val, &JsValue::from_str("isLinearGradient")).unwrap();
        let is_radial_gradient_func = Reflect::get(&val, &JsValue::from_str("isRadialGradient")).unwrap();
        if Function::from(is_color_func).call0(&val).unwrap().as_bool().unwrap() {
            let get_color_func = Reflect::get(&val, &JsValue::from_str("getColor")).unwrap();
            let color = Function::from(get_color_func).call0(&val).unwrap();
            let get_r_func = Reflect::get(&color, &JsValue::from_str("getR")).unwrap();
            let get_g_func = Reflect::get(&color, &JsValue::from_str("getG")).unwrap();
            let get_b_func = Reflect::get(&color, &JsValue::from_str("getB")).unwrap();
            let get_a_func = Reflect::get(&color, &JsValue::from_str("getA")).unwrap();
            let r = Function::from(get_r_func).call0(&color).unwrap().as_f64().unwrap();
            let g = Function::from(get_g_func).call0(&color).unwrap().as_f64().unwrap();
            let b = Function::from(get_b_func).call0(&color).unwrap().as_f64().unwrap();
            let a = Function::from(get_a_func).call0(&color).unwrap().as_f64().unwrap();
            return WasmGradientImageOrColor {
                gradient_image_or_color: GradientImageOrColor::Color(Color { red: r, green: g, blue: b, alpha: a })
            };
        } else if Function::from(is_linear_gradient_func).call0(&val).unwrap().as_bool().unwrap() {
            let get_linear_gradient_func = Reflect::get(&val, &JsValue::from_str("getLinearGradient")).unwrap();
            let linear_gradient = Function::from(get_linear_gradient_func).call0(&val).unwrap();
            let get_x1_func = Reflect::get(&linear_gradient, &JsValue::from_str("getX1")).unwrap();
            let get_y1_func = Reflect::get(&linear_gradient, &JsValue::from_str("getY1")).unwrap();
            let get_x2_func = Reflect::get(&linear_gradient, &JsValue::from_str("getX2")).unwrap();
            let get_y2_func = Reflect::get(&linear_gradient, &JsValue::from_str("getY2")).unwrap();
            let get_stops_func = Reflect::get(&linear_gradient, &JsValue::from_str("getStops")).unwrap();
            let get_alpha_func = Reflect::get(&linear_gradient, &JsValue::from_str("getAlpha")).unwrap();
            let x1 = Function::from(get_x1_func).call0(&linear_gradient).unwrap().as_f64().unwrap();
            let y1 = Function::from(get_y1_func).call0(&linear_gradient).unwrap().as_f64().unwrap();
            let x2 = Function::from(get_x2_func).call0(&linear_gradient).unwrap().as_f64().unwrap();
            let y2 = Function::from(get_y2_func).call0(&linear_gradient).unwrap().as_f64().unwrap();
            let stops = Function::from(get_stops_func).call0(&linear_gradient).unwrap();
            let stops = Array::from(&stops);
            let stops = stops.iter().map(|stop| {
                let stop = stop.dyn_into::<WasmGradientStop>().unwrap();
                stop.stop
            }).collect();
            let alpha = Function::from(get_alpha_func).call0(&linear_gradient).unwrap().as_f64().unwrap();
            return WasmGradientImageOrColor {
                gradient_image_or_color: GradientImageOrColor::LinearGradient(LinearGradient { x1, y1, x2, y2, stops, alpha })
            };
        } else if Function::from(is_radial_gradient_func).call0(&val).unwrap().as_bool().unwrap() {
            let get_radial_gradient_func = Reflect::get(&val, &JsValue::from_str("getRadialGradient")).unwrap();
            let radial_gradient = Function::from(get_radial_gradient_func).call0(&val).unwrap();
            let get_cx_func = Reflect::get(&radial_gradient, &JsValue::from_str("getCx")).unwrap();
            let get_cy_func = Reflect::get(&radial_gradient, &JsValue::from_str("getCy")).unwrap();
            let get_r_func = Reflect::get(&radial_gradient, &JsValue::from_str("getR")).unwrap();
            let get_fx_func = Reflect::get(&radial_gradient, &JsValue::from_str("getFx")).unwrap();
            let get_fy_func = Reflect::get(&radial_gradient, &JsValue::from_str("getFy")).unwrap();
            let get_stops_func = Reflect::get(&radial_gradient, &JsValue::from_str("getStops")).unwrap();
            let get_alpha_func = Reflect::get(&radial_gradient, &JsValue::from_str("getAlpha")).unwrap();
            let cx = Function::from(get_cx_func).call0(&radial_gradient).unwrap().as_f64().unwrap();
            let cy = Function::from(get_cy_func).call0(&radial_gradient).unwrap().as_f64().unwrap();
            let r = Function::from(get_r_func).call0(&radial_gradient).unwrap().as_f64().unwrap();
            let fx = Function::from(get_fx_func).call0(&radial_gradient).unwrap().as_f64().unwrap();
            let fy = Function::from(get_fy_func).call0(&radial_gradient).unwrap().as_f64().unwrap();
            let stops = Function::from(get_stops_func).call0(&radial_gradient).unwrap();
            let stops = Array::from(&stops);
            let stops = stops.iter().map(|stop| {
                let stop = stop.dyn_into::<WasmGradientStop>().unwrap();
                stop.stop
            }).collect();
            let alpha = Function::from(get_alpha_func).call0(&radial_gradient).unwrap().as_f64().unwrap();
            return WasmGradientImageOrColor {
                gradient_image_or_color: GradientImageOrColor::RadialGradient(RadialGradient { cx, cy, r, fx, fy, stops, alpha })
            };
        }
        let get_image_func = Reflect::get(&val, &JsValue::from_str("getImage")).unwrap();
        let image = Function::from(get_image_func).call0(&val).unwrap();
        let get_image_base64_func = Reflect::get(&image, &JsValue::from_str("getImageBase64")).unwrap();
        let image_base64 = Function::from(get_image_base64_func).call0(&image).unwrap().as_string().unwrap();
        let get_mime_type_func = Reflect::get(&image, &JsValue::from_str("getMimeType")).unwrap();
        let mime_type = Function::from(get_mime_type_func).call0(&image).unwrap().as_string().unwrap();
        let get_top_func = Reflect::get(&image, &JsValue::from_str("getTop")).unwrap();
        let get_left_func = Reflect::get(&image, &JsValue::from_str("getLeft")).unwrap();
        let get_bottom_func = Reflect::get(&image, &JsValue::from_str("getBottom")).unwrap();
        let get_right_func = Reflect::get(&image, &JsValue::from_str("getRight")).unwrap();
        let get_alpha_func = Reflect::get(&image, &JsValue::from_str("getAlpha")).unwrap();
        let top = Function::from(get_top_func).call0(&image).unwrap().as_f64().unwrap();
        let left = Function::from(get_left_func).call0(&image).unwrap().as_f64().unwrap();
        let bottom = Function::from(get_bottom_func).call0(&image).unwrap().as_f64().unwrap();
        let right = Function::from(get_right_func).call0(&image).unwrap().as_f64().unwrap();
        let alpha = Function::from(get_alpha_func).call0(&image).unwrap().as_f64().unwrap();
        WasmGradientImageOrColor {
            gradient_image_or_color: GradientImageOrColor::Image(Image { image_base64, mime_type, top_left_corner: (left, top), bottom_right_corner: (right, bottom), alpha })
        }
    }
    fn unchecked_from_js_ref(val: &JsValue) -> &Self {
        &val.unchecked_ref::<WasmGradientImageOrColor>()
    }
}


impl AsRef<JsValue> for WasmGradientImageOrColor {
    fn as_ref(&self) -> &JsValue {
        self.unchecked_ref()
    }
}


#[wasm_bindgen]
impl WasmGradientImageOrColor {
    #[wasm_bindgen(js_name = fromColor)]
    pub fn from_color(color: WasmColor) -> WasmGradientImageOrColor {
        WasmGradientImageOrColor {
            gradient_image_or_color: GradientImageOrColor::Color(color.color)
        }
    }
    #[wasm_bindgen(js_name = fromLinearGradient)]
    pub fn from_linear_gradient(linear_gradient: WasmLinearGradient) -> WasmGradientImageOrColor {
        WasmGradientImageOrColor {
            gradient_image_or_color: GradientImageOrColor::LinearGradient(linear_gradient.linear_gradient)
        }
    }
    #[wasm_bindgen(js_name = fromRadialGradient)]
    pub fn from_radial_gradient(radial_gradient: WasmRadialGradient) -> WasmGradientImageOrColor {
        WasmGradientImageOrColor {
            gradient_image_or_color: GradientImageOrColor::RadialGradient(radial_gradient.radial_gradient)
        }
    }
    #[wasm_bindgen(js_name = fromImage)]
    pub fn from_image(image: WasmImage) -> WasmGradientImageOrColor {
        WasmGradientImageOrColor {
            gradient_image_or_color: GradientImageOrColor::Image(image.image)
        }
    }
    #[wasm_bindgen(js_name = isColor)]
    pub fn is_color(&self) -> bool {
        match &self.gradient_image_or_color {
            GradientImageOrColor::Color(_) => true,
            _ => false
        }
    }
    #[wasm_bindgen(js_name = isLinearGradient)]
    pub fn is_linear_gradient(&self) -> bool {
        match &self.gradient_image_or_color {
            GradientImageOrColor::LinearGradient(_) => true,
            _ => false 
        }   
    }
    #[wasm_bindgen(js_name = isRadialGradient)]
    pub fn is_radial_gradient(&self) -> bool {
        match &self.gradient_image_or_color {
            GradientImageOrColor::RadialGradient(_) => true,
            _ => false
        }  
    }
    #[wasm_bindgen(js_name = isImage)]
    pub fn is_image(&self) -> bool {
        match &self.gradient_image_or_color {
            GradientImageOrColor::Image(_) => true,
            _ => false
        }  
    }
    #[wasm_bindgen(js_name = getColor)]
    pub fn get_color(&self) -> Option<WasmColor> {
        match &self.gradient_image_or_color {
            GradientImageOrColor::Color(color) => Some(WasmColor { color: color.clone() }),
            _ => None
        }
    }
    #[wasm_bindgen(js_name = getLinearGradient)]
    pub fn get_linear_gradient(&self) -> Option<WasmLinearGradient> {
        match &self.gradient_image_or_color {
            GradientImageOrColor::LinearGradient(linear_gradient) => Some(WasmLinearGradient { linear_gradient: linear_gradient.clone() }),
            _ => None
        }
    }
    #[wasm_bindgen(js_name = getRadialGradient)]
    pub fn get_radial_gradient(&self) -> Option<WasmRadialGradient> {
        match &self.gradient_image_or_color {
            GradientImageOrColor::RadialGradient(radial_gradient) => Some(WasmRadialGradient { radial_gradient: radial_gradient.clone() }),
            _ => None
        }
    }
    #[wasm_bindgen(js_name = getImage)]
    pub fn get_image(&self) -> Option<WasmImage> {
        match &self.gradient_image_or_color {
            GradientImageOrColor::Image(image) => Some(WasmImage { image: image.clone() }),
            _ => None
        }
    }
    #[wasm_bindgen(js_name = clone)]
    pub fn clone_js(&self) -> WasmGradientImageOrColor {
        self.clone()
    }
}


#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct WasmColor {
    #[wasm_bindgen(skip)]
    pub color: Color,
}


impl JsCast for WasmColor {
    fn instanceof(val: &JsValue) -> bool {
        // Check if the objects has all getters to avoid recursion errors
        Reflect::get(&val, &JsValue::from_str("getR")).is_ok() &&
        Reflect::get(&val, &JsValue::from_str("getG")).is_ok() &&
        Reflect::get(&val, &JsValue::from_str("getB")).is_ok() &&
        Reflect::get(&val, &JsValue::from_str("getA")).is_ok()
    }
    fn unchecked_from_js(val: JsValue) -> Self {
        let get_r_func = Reflect::get(&val, &JsValue::from_str("getR")).unwrap();
        let get_g_func = Reflect::get(&val, &JsValue::from_str("getG")).unwrap();
        let get_b_func = Reflect::get(&val, &JsValue::from_str("getB")).unwrap();
        let get_a_func = Reflect::get(&val, &JsValue::from_str("getA")).unwrap();
        let r = Function::from(get_r_func).call0(&val).unwrap().as_f64().unwrap();
        let g = Function::from(get_g_func).call0(&val).unwrap().as_f64().unwrap();
        let b = Function::from(get_b_func).call0(&val).unwrap().as_f64().unwrap();
        let a = Function::from(get_a_func).call0(&val).unwrap().as_f64().unwrap();
        WasmColor {
            color: Color { red: r, green: g, blue: b, alpha: a }
        }
    }
    fn unchecked_from_js_ref(val: &JsValue) -> &Self {
        &val.unchecked_ref::<WasmColor>()
    }
}


impl AsRef<JsValue> for WasmColor {
    fn as_ref(&self) -> &JsValue {
        self.unchecked_ref()
    }
}


#[wasm_bindgen]
impl WasmColor {
    #[wasm_bindgen(constructor)]
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> WasmColor {
        WasmColor {
            color: Color { red: r, green: g, blue: b, alpha: a }
        }
    }
    #[wasm_bindgen(js_name = getR)]
    pub fn get_r(&self) -> f64 {
        self.color.red
    }
    #[wasm_bindgen(js_name = getG)]
    pub fn get_g(&self) -> f64 {
        self.color.green
    }
    #[wasm_bindgen(js_name = getB)]
    pub fn get_b(&self) -> f64 {
        self.color.blue
    }
    #[wasm_bindgen(js_name = getA)]
    pub fn get_a(&self) -> f64 {
        self.color.alpha
    }
}


#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct WasmGradientStop {
    stop: GradientStop
}


impl JsCast for WasmGradientStop {
    fn instanceof(val: &JsValue) -> bool {
        // Check if the objects has all getters to avoid recursion errors
        Reflect::get(&val, &JsValue::from_str("getOffset")).is_ok() &&
        Reflect::get(&val, &JsValue::from_str("getColor")).is_ok()
    }
    fn unchecked_from_js(val: JsValue) -> Self {
        let get_offset_func = Reflect::get(&val, &JsValue::from_str("getOffset")).unwrap();
        let get_color_func = Reflect::get(&val, &JsValue::from_str("getColor")).unwrap();
        let offset = Function::from(get_offset_func).call0(&val).unwrap().as_f64().unwrap();
        let color = Function::from(get_color_func).call0(&val).unwrap();
        let get_r_func = Reflect::get(&color, &JsValue::from_str("getR")).unwrap();
        let get_g_func = Reflect::get(&color, &JsValue::from_str("getG")).unwrap();
        let get_b_func = Reflect::get(&color, &JsValue::from_str("getB")).unwrap();
        let get_a_func = Reflect::get(&color, &JsValue::from_str("getA")).unwrap();
        let r = Function::from(get_r_func).call0(&color).unwrap().as_f64().unwrap();
        let g = Function::from(get_g_func).call0(&color).unwrap().as_f64().unwrap();
        let b = Function::from(get_b_func).call0(&color).unwrap().as_f64().unwrap();
        let a = Function::from(get_a_func).call0(&color).unwrap().as_f64().unwrap();
        WasmGradientStop {
            stop: GradientStop { offset, color: Color { red: r, green: g, blue: b, alpha: a } }
        }
    }
    fn unchecked_from_js_ref(val: &JsValue) -> &Self {
        &val.unchecked_ref::<WasmGradientStop>()
    }
}


impl AsRef<JsValue> for WasmGradientStop {
    fn as_ref(&self) -> &JsValue {
        self.unchecked_ref()
    }
}


#[wasm_bindgen]
impl WasmGradientStop {
    #[wasm_bindgen(constructor)]
    pub fn new(offset: f64, color: WasmColor) -> WasmGradientStop {
        WasmGradientStop {
            stop: GradientStop { offset, color: color.color }
        }
    }
    #[wasm_bindgen(js_name = getOffset)]
    pub fn get_offset(&self) -> f64 {
        self.stop.offset
    }
    #[wasm_bindgen(js_name = getColor)]
    pub fn get_color(&self) -> WasmColor {
        WasmColor { color: self.stop.color.clone() }
    }
}


#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct WasmLinearGradient {
    linear_gradient: LinearGradient
}


#[wasm_bindgen]
impl WasmLinearGradient {
    #[wasm_bindgen(constructor)]
    pub fn new(
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        stops: Vec<WasmGradientStop>,
        alpha: f64
    ) -> WasmLinearGradient {
        WasmLinearGradient {
            linear_gradient: LinearGradient { 
                x1,
                x2,
                y1,
                y2,
                stops: stops.iter().map(|stop| stop.stop.clone()).collect(),
                alpha: alpha
            }
        }
    }
    #[wasm_bindgen(js_name = getX1)]
    pub fn get_x1(&self) -> f64 {
        self.linear_gradient.x1
    }
    #[wasm_bindgen(js_name = getY1)]
    pub fn get_y1(&self) -> f64 {
        self.linear_gradient.y1
    }
    #[wasm_bindgen(js_name = getX2)]
    pub fn get_x2(&self) -> f64 {
        self.linear_gradient.x2
    }
    #[wasm_bindgen(js_name = getY2)]
    pub fn get_y2(&self) -> f64 {
        self.linear_gradient.y2
    }
    #[wasm_bindgen(js_name = getStops)]
    pub fn get_stops(&self) -> Vec<WasmGradientStop> {
        let stops = self.linear_gradient.stops.iter().map(|stop| WasmGradientStop { stop: stop.clone() }).collect();
        stops
    }
    #[wasm_bindgen(js_name = getAlpha)]
    pub fn get_alpha(&self) -> f64 {
        self.linear_gradient.alpha
    }
}


#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct WasmRadialGradient {
    radial_gradient: RadialGradient
}


#[wasm_bindgen]
impl WasmRadialGradient {
    #[wasm_bindgen(constructor)]
    pub fn new(
        cx: f64,
        cy: f64,
        r: f64,
        fx: f64,
        fy: f64,
        stops: Vec<WasmGradientStop>,
        alpha: f64
    ) -> WasmRadialGradient {
        WasmRadialGradient {
            radial_gradient: RadialGradient { 
                cx,
                cy,
                r,
                fx,
                fy,
                stops: stops.iter().map(|stop| stop.stop.clone()).collect(),
                alpha: alpha
            }
        }
    }
    #[wasm_bindgen(js_name = getCx)]
    pub fn get_cx(&self) -> f64 {
        self.radial_gradient.cx
    }
    #[wasm_bindgen(js_name = getCy)]
    pub fn get_cy(&self) -> f64 {
        self.radial_gradient.cy
    }
    #[wasm_bindgen(js_name = getR)]
    pub fn get_r(&self) -> f64 {
        self.radial_gradient.r
    }
    #[wasm_bindgen(js_name = getFx)]
    pub fn get_fx(&self) -> f64 {
        self.radial_gradient.fx
    }
    #[wasm_bindgen(js_name = getFy)]
    pub fn get_fy(&self) -> f64 {
        self.radial_gradient.fy
    }
    #[wasm_bindgen(js_name = getStops)]
    pub fn get_stops(&self) -> Vec<WasmGradientStop> {
        let stops = self.radial_gradient.stops.iter().map(|stop| WasmGradientStop { stop: stop.clone() }).collect();
        stops
    }
    #[wasm_bindgen(js_name = getAlpha)]
    pub fn get_alpha(&self) -> f64 {
        self.radial_gradient.alpha
    }
}


#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct WasmImage {
    image: Image
}


#[wasm_bindgen]
impl WasmImage {
    #[wasm_bindgen(constructor)]
    pub fn new(
        image_base64: String,
        mime_type: String,
        top: f64,
        left: f64,
        bottom: f64,
        right: f64,
        alpha: f64
    ) -> WasmImage {
        let top_left_corner = (left, top);
        let bottom_right_corner = (right, bottom);
        WasmImage {
            image: Image {
                image_base64,
                mime_type,
                top_left_corner,
                bottom_right_corner,
                alpha
            }
        }
    }
    #[wasm_bindgen(js_name = getImageBase64)]
    pub fn get_image_base64(&self) -> String {
        self.image.image_base64.clone()
    }
    #[wasm_bindgen(js_name = getMimeType)]
    pub fn get_mime_type(&self) -> String {
        self.image.mime_type.clone()
    }
    #[wasm_bindgen(js_name = getTop)]
    pub fn get_top(&self) -> f64 {
        self.image.top_left_corner.1
    }
    #[wasm_bindgen(js_name = getLeft)]
    pub fn get_left(&self) -> f64 {
        self.image.top_left_corner.0
    }
    #[wasm_bindgen(js_name = getBottom)]
    pub fn get_bottom(&self) -> f64 {
        self.image.bottom_right_corner.1
    }
    #[wasm_bindgen(js_name = getRight)]
    pub fn get_right(&self) -> f64 {
        self.image.bottom_right_corner.0
    }
    #[wasm_bindgen(js_name = getAlpha)]
    pub fn get_alpha(&self) -> f64 {
        self.image.alpha
    }
}


#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct WasmVectorObject {
    #[wasm_bindgen(skip)]
    pub native_vec_features: VectorFeatures
}


#[wasm_bindgen]
impl WasmVectorObject {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmVectorObject {
        WasmVectorObject {
            native_vec_features: VectorFeatures::new()
        }
    }
    #[wasm_bindgen(js_name = getIndex)]
    pub fn get_index(&self) -> usize {
        self.native_vec_features.get_index()
    }
    #[wasm_bindgen(js_name = incrementIndex)]
    pub fn increment_index(&self, increment: usize, recursive: bool) -> Self {
        WasmVectorObject {
            native_vec_features: self.native_vec_features.increment_index(increment, recursive)
        }
    }
    #[wasm_bindgen(js_name = getPoints)]
    pub fn get_points(&self) -> Array {
        let points = self.native_vec_features.get_points();
        let points = points.iter().map(|point| Array::of2(&point.0.into(), &point.1.into())).collect();
        points
    }
    #[wasm_bindgen(js_name = add)]
    pub fn add(&self, new_subobject: WasmVectorObject) -> Self {
        WasmVectorObject {
            native_vec_features: self.native_vec_features.add(&new_subobject.native_vec_features)
        }
    }
    #[wasm_bindgen(js_name = remove)]
    pub fn remove(&self, index: usize) -> Self {
        WasmVectorObject {
            native_vec_features: self.native_vec_features.remove(index)
        }
    }
    #[wasm_bindgen(js_name = getSubobject)]
    pub fn get_subobject(&self, index: usize) -> WasmVectorObject {
        WasmVectorObject {
            native_vec_features: self.native_vec_features.get_subobject(index)
        }
    }
    #[wasm_bindgen(js_name = sliceSubobjects)]
    pub fn slice_subobjects(&self, start: usize, end: usize) -> Vec<WasmVectorObject> {
        let subobjects = self.native_vec_features.slice_subobjects(start, end);
        let subobjects = subobjects.iter().map(|object| WasmVectorObject { native_vec_features: object.clone() }).collect();
        subobjects
    }
    #[wasm_bindgen(js_name = setSubobject)]
    pub fn set_subobject(&self, index: usize, new_subobject: WasmVectorObject) -> Self {
        WasmVectorObject {
            native_vec_features: self.native_vec_features.set_subobject(index, new_subobject.native_vec_features)
        }
    }
    #[wasm_bindgen(js_name = setSliceSubobjects)]
    pub fn set_slice_subobjects(&self, start: usize, end: usize, new_subobjects: Vec<WasmVectorObject>) -> Self {
        let new_subobjects = new_subobjects.iter().map(|object| object.native_vec_features.clone()).collect();
        WasmVectorObject {
            native_vec_features: self.native_vec_features.set_slice_subobjects(start, end, new_subobjects)
        }
    }
    #[wasm_bindgen(js_name = getFill)]
    pub fn get_fill(&self) -> WasmGradientImageOrColor {
        WasmGradientImageOrColor { gradient_image_or_color: self.native_vec_features.get_fill() }
    }
    #[wasm_bindgen(js_name = getStroke)]
    pub fn get_stroke(&self) -> WasmGradientImageOrColor {
        WasmGradientImageOrColor { gradient_image_or_color: self.native_vec_features.get_stroke() }
    }
    #[wasm_bindgen(js_name = getStrokeWidth)]
    pub fn get_stroke_width(&self) -> f64 {
        self.native_vec_features.get_stroke_width()
    }
    #[wasm_bindgen(js_name = getLineCap)]
    pub fn get_line_cap(&self) -> String {
        self.native_vec_features.get_line_cap().to_string()
    }
    #[wasm_bindgen(js_name = getLineJoin)]
    pub fn get_line_join(&self) -> String {
        self.native_vec_features.get_line_join().to_string()
    }
    #[wasm_bindgen(js_name = getPartialCopy)]
    pub fn get_partial_copy(&self, start: f64, end: f64, recursive: bool) -> Self {
        WasmVectorObject {
            native_vec_features: self.native_vec_features.get_partial_copy(start, end, recursive)
        }
    }
    #[wasm_bindgen(js_name = getSubpaths)]
    pub fn get_subpaths(&self) -> Array {
        let subpaths = self.native_vec_features.get_subpaths();
        let subpaths = subpaths.iter().map(|subpath| {
            let points = subpath.iter().map(|point| Array::of2(&point.0.into(), &point.1.into())).collect::<Array>();
            points
        }).collect();
        subpaths
    }
    #[wasm_bindgen(js_name = getPieces)]
    pub fn get_pieces(&self, n_pieces: usize) -> WasmVectorObject {
        let pieces = self.native_vec_features.get_pieces(n_pieces);
        WasmVectorObject { native_vec_features: pieces }
    }
    #[wasm_bindgen(js_name = getCubicBezierTuples)]
    pub fn get_cubic_bezier_tuples(&self) -> Array {
        let cubic_bezier_tuples = self.native_vec_features.get_cubic_bezier_tuples();
        let cubic_bezier_tuples = cubic_bezier_tuples.iter().map(|tuple| {
            let point1 = Array::of2(&tuple.0.0.into(), &tuple.0.1.into());
            let point2 = Array::of2(&tuple.1.0.into(), &tuple.1.1.into());
            let point3 = Array::of2(&tuple.2.0.into(), &tuple.2.1.into());
            let point4 = Array::of2(&tuple.3.0.into(), &tuple.3.1.into());
            Array::of4(&point1, &point2, &point3, &point4)
        }).collect();
        cubic_bezier_tuples
    }
    #[wasm_bindgen(js_name = getSubobjects)]
    pub fn get_subobjects(&self) -> Vec<WasmVectorObject> {
        let subobjects = self.native_vec_features.get_subobjects();
        let subobjects = subobjects.iter().map(|object| WasmVectorObject { native_vec_features: object.clone() }).collect();
        subobjects
    }
    #[wasm_bindgen(js_name = scale)]
    pub fn scale(&self, factor: f64, recursive: bool) -> Self {
        WasmVectorObject {
            native_vec_features: self.native_vec_features.scale(factor, recursive)
        }
    }
    #[wasm_bindgen(js_name = stretch)]
    pub fn stretch(&self, x_factor: f64, y_factor: f64, recursive: bool) -> Self {
        WasmVectorObject {
            native_vec_features: self.native_vec_features.stretch((x_factor, y_factor), recursive)
        }
    }
    #[wasm_bindgen(js_name = shift)]
    pub fn shift(&self, x_shift: f64, y_shift: f64, recursive: bool) -> Self {
        WasmVectorObject {
            native_vec_features: self.native_vec_features.shift((x_shift, y_shift), recursive)
        }
    }
    #[wasm_bindgen(js_name = mergedPoints)]
    pub fn merged_points(&self) -> Array {
        let merged_points = self.native_vec_features.merged_points();
        let merged_points = merged_points.iter().map(|point| Array::of2(&point.0.into(), &point.1.into())).collect();
        merged_points
    }
    #[wasm_bindgen(js_name = getBoundingBox)]
    pub fn get_bounding_box(&self) -> Array {
        let ((min_x, min_y), (max_x, max_y)) = self.native_vec_features.get_bounding_box();
        let min_point = Array::of2(&min_x.into(), &min_y.into());
        let max_point = Array::of2(&max_x.into(), &max_y.into());
        Array::of2(&min_point, &max_point)
    }
    #[wasm_bindgen(js_name = getCenter)]
    pub fn get_center(&self) -> Array {
        let (center_x, center_y) = self.native_vec_features.get_center();
        Array::of2(&center_x.into(), &center_y.into())
    }
    #[wasm_bindgen(js_name = getCenterOfMass)]
    pub fn get_center_of_mass(&self) -> Array {
        let (center_x, center_y) = self.native_vec_features.get_center_of_mass();
        Array::of2(&center_x.into(), &center_y.into())
    }
    #[wasm_bindgen(js_name = getHeight)]
    pub fn get_height(&self) -> f64 {
        self.native_vec_features.get_height()
    }
    #[wasm_bindgen(js_name = getWidth)]
    pub fn get_width(&self) -> f64 {
        self.native_vec_features.get_width()
    }
    #[wasm_bindgen(js_name = setIndex)]
    pub fn set_index(&self, index: usize) -> Self {
        WasmVectorObject {
            native_vec_features: self.native_vec_features.set_index(index)
        }
    }
    #[wasm_bindgen(js_name = setFill)]
    pub fn set_fill(&self, fill: WasmGradientImageOrColor, recursive: bool) -> Self {
        WasmVectorObject {
            native_vec_features: self.native_vec_features.set_fill(fill.gradient_image_or_color, recursive)
        }
    }
    #[wasm_bindgen(js_name = setFillOpacity)]
    pub fn set_fill_opacity(&self, opacity: f64, recursive: bool) -> Self {
        WasmVectorObject {
            native_vec_features: self.native_vec_features.set_fill_opacity(opacity, recursive)
        }
    }
    #[wasm_bindgen(js_name = moveTo)]
    pub fn move_to(&self, x: f64, y: f64, recursive: bool) -> Self {
        WasmVectorObject {
            native_vec_features: self.native_vec_features.move_to((x, y), recursive)
        }
    }
    #[wasm_bindgen(js_name = setStroke)]
    pub fn set_stroke(&self, stroke: WasmGradientImageOrColor, recursive: bool) -> Self {
        WasmVectorObject {
            native_vec_features: self.native_vec_features.set_stroke(stroke.gradient_image_or_color, recursive)
        }
    }
    #[wasm_bindgen(js_name = setStrokeOpacity)]
    pub fn set_stroke_opacity(&self, opacity: f64, recursive: bool) -> Self {
        WasmVectorObject {
            native_vec_features: self.native_vec_features.set_stroke_opacity(opacity, recursive)
        }
    }
    #[wasm_bindgen(js_name = setStrokeWidth)]
    pub fn set_stroke_width(&self, width: f64, recursive: bool) -> Self {
        WasmVectorObject {
            native_vec_features: self.native_vec_features.set_stroke_width(width, recursive)
        }
    }
    #[wasm_bindgen(js_name = setLineCap)]
    pub fn set_line_cap(&self, line_cap: String, recursive: bool) -> Self {
        match line_cap.as_str() {
            "butt" => WasmVectorObject {
                native_vec_features: self.native_vec_features.set_line_cap("butt", recursive)
            },
            "round" => WasmVectorObject {
                native_vec_features: self.native_vec_features.set_line_cap("round", recursive)
            },
            "square" => WasmVectorObject {
                native_vec_features: self.native_vec_features.set_line_cap("square", recursive)
            },
            _ => WasmVectorObject {
                native_vec_features: self.native_vec_features.set_line_cap("butt", recursive)
            }
        }
    }
    #[wasm_bindgen(js_name = setLineJoin)]
    pub fn set_line_join(&self, line_join: String, recursive: bool) -> Self {
        match line_join.as_str() {
            "miter" => WasmVectorObject {
                native_vec_features: self.native_vec_features.set_line_join("miter", recursive)
            },
            "round" => WasmVectorObject {
                native_vec_features: self.native_vec_features.set_line_join("round", recursive)
            },
            "bevel" => WasmVectorObject {
                native_vec_features: self.native_vec_features.set_line_join("bevel", recursive)
            },
            _ => WasmVectorObject {
                native_vec_features: self.native_vec_features.set_line_join("miter", recursive)
            }
        }
    }
    #[wasm_bindgen(js_name = setPoints)]
    pub fn set_points(&self, points: Array) -> Self {
        let points = points.iter().map(|point| {
            let point = point.dyn_into::<Array>().unwrap();
            let x = point.get(0).as_f64().unwrap();
            let y = point.get(1).as_f64().unwrap();
            (x, y)
        }).collect();
        WasmVectorObject {
            native_vec_features: self.native_vec_features.set_points(points)
        }
    }
    #[wasm_bindgen(js_name = setSubobjects)]
    pub fn set_subobjects(&self, subobjects: Vec<WasmVectorObject>) -> Self {
        let subobjects = subobjects.iter().map(|object| object.native_vec_features.clone()).collect();
        WasmVectorObject {
            native_vec_features: self.native_vec_features.set_subobjects(subobjects)
        }
    }
    #[wasm_bindgen(js_name = rotate)]
    pub fn rotate(&self, angle: f64, recursive: bool) -> Self {
        WasmVectorObject {
            native_vec_features: self.native_vec_features.rotate(angle, recursive)
        }
    }
    #[wasm_bindgen(js_name = getCriticalPoint)]
    pub fn get_critical_point(&self, key_x: f64, key_y: f64) -> Array {
        let (critical_x, critical_y) = self.native_vec_features.get_critical_point((key_x, key_y));
        Array::of2(&critical_x.into(), &critical_y.into())
    }
    #[wasm_bindgen(js_name = getFillOpacity)]
    pub fn get_fill_opacity(&self) -> f64 {
        self.native_vec_features.get_fill_opacity()
    }
    #[wasm_bindgen(js_name = getStrokeOpacity)]
    pub fn get_stroke_opacity(&self) -> f64 {
        self.native_vec_features.get_stroke_opacity()
    }
    #[wasm_bindgen(js_name = nextToOther)]
    pub fn next_to_other(
        &self,
        other: WasmVectorObject,
        direction: Array,
        buff: f64,
        aligned_edge: Array,
        recursive: bool
    ) -> Self {
        let direction = (direction.get(0).as_f64().unwrap(), direction.get(1).as_f64().unwrap());
        let aligned_edge = (aligned_edge.get(0).as_f64().unwrap(), aligned_edge.get(1).as_f64().unwrap());
        WasmVectorObject {
            native_vec_features: self.native_vec_features.next_to_other(&other.native_vec_features, direction, buff, aligned_edge, recursive)
        }
    }
    #[wasm_bindgen(js_name = arrangeSubobjects)]
    pub fn arrange_subobjects(
        &self,
        direction: Array,
        buff: f64,
        aligned_edge: Array,
        recursive: bool
    ) -> Self {
        let direction = (direction.get(0).as_f64().unwrap(), direction.get(1).as_f64().unwrap());
        let aligned_edge = (aligned_edge.get(0).as_f64().unwrap(), aligned_edge.get(1).as_f64().unwrap());
        WasmVectorObject {
            native_vec_features: self.native_vec_features.arrange_subobjects(direction, buff, aligned_edge, recursive)
        }
    }
    #[wasm_bindgen(js_name = nextToPoint)]
    pub fn next_to_point(
        &self,
        point: Array,
        direction: Array,
        buff: f64,
        aligned_edge: Array,
        recursive: bool
    ) -> Self {
        let point = (point.get(0).as_f64().unwrap(), point.get(1).as_f64().unwrap());
        let direction = (direction.get(0).as_f64().unwrap(), direction.get(1).as_f64().unwrap());
        let aligned_edge = (aligned_edge.get(0).as_f64().unwrap(), aligned_edge.get(1).as_f64().unwrap());
        WasmVectorObject {
            native_vec_features: self.native_vec_features.next_to_point(point, direction, buff, aligned_edge, recursive)
        }
    }
    #[wasm_bindgen(js_name = clone)]
    pub fn clone_js(&self) -> Self {
        self.clone()
    }
}


impl JsCast for WasmVectorObject {
    fn instanceof(val: &JsValue) -> bool {
        // Check if the objects has all getters to avoid recursion errors
        Reflect::get(&val, &JsValue::from_str("getPoints")).is_ok() &&
        Reflect::get(&val, &JsValue::from_str("getFill")).is_ok() &&
        Reflect::get(&val, &JsValue::from_str("getStroke")).is_ok() &&
        Reflect::get(&val, &JsValue::from_str("getStrokeWidth")).is_ok() &&
        Reflect::get(&val, &JsValue::from_str("getLineCap")).is_ok() &&
        Reflect::get(&val, &JsValue::from_str("getLineJoin")).is_ok() &&
        Reflect::get(&val, &JsValue::from_str("getSubobjects")).is_ok() &&
        Reflect::get(&val, &JsValue::from_str("getIndex")).is_ok()
    }
    fn unchecked_from_js(val: JsValue) -> Self {
        let get_points = Reflect::get(&val, &JsValue::from_str("getPoints")).unwrap();
        let get_fill = Reflect::get(&val, &JsValue::from_str("getFill")).unwrap();
        let get_stroke = Reflect::get(&val, &JsValue::from_str("getStroke")).unwrap();
        let get_stroke_width = Reflect::get(&val, &JsValue::from_str("getStrokeWidth")).unwrap();
        let get_line_cap = Reflect::get(&val, &JsValue::from_str("getLineCap")).unwrap();
        let get_line_join = Reflect::get(&val, &JsValue::from_str("getLineJoin")).unwrap();
        let get_subobjects = Reflect::get(&val, &JsValue::from_str("getSubobjects")).unwrap();
        let get_index = Reflect::get(&val, &JsValue::from_str("getIndex")).unwrap();
        let points = get_points.dyn_into::<js_sys::Function>().unwrap().call0(&val).unwrap();
        let fill = get_fill.dyn_into::<js_sys::Function>().unwrap().call0(&val).unwrap();
        let stroke = get_stroke.dyn_into::<js_sys::Function>().unwrap().call0(&val).unwrap();
        let stroke_width = get_stroke_width.dyn_into::<js_sys::Function>().unwrap().call0(&val).unwrap();
        let line_cap = get_line_cap.dyn_into::<js_sys::Function>().unwrap().call0(&val).unwrap();
        let line_join = get_line_join.dyn_into::<js_sys::Function>().unwrap().call0(&val).unwrap();
        let subobjects = get_subobjects.dyn_into::<js_sys::Function>().unwrap().call0(&val).unwrap();
        let index = get_index.dyn_into::<js_sys::Function>().unwrap().call0(&val).unwrap();
        let points = points.dyn_into::<Array>().unwrap();
        let fill = fill.dyn_into::<WasmGradientImageOrColor>().unwrap();
        let stroke = stroke.dyn_into::<WasmGradientImageOrColor>().unwrap();
        let stroke_width = stroke_width.as_f64().unwrap();
        let line_cap = line_cap.as_string().unwrap();
        let line_join = line_join.as_string().unwrap();
        let subobjects = subobjects.dyn_into::<Array>().unwrap();
        let index = index.as_f64().unwrap() as usize;
        let subobjects = subobjects.iter().map(|object| object.dyn_into::<WasmVectorObject>().unwrap()).collect::<Vec<WasmVectorObject>>();
        return WasmVectorObject::new()
            .set_points(points)
            .set_fill(fill, false)
            .set_stroke(stroke, false)
            .set_stroke_width(stroke_width, false)
            .set_line_cap(line_cap, false)
            .set_line_join(line_join, false)
            .set_subobjects(subobjects)
            .set_index(index);
    }
    fn unchecked_from_js_ref(val: &JsValue) -> &Self {
        val.unchecked_ref::<WasmVectorObject>()
    }
}


impl AsRef<JsValue> for WasmVectorObject {
    fn as_ref(&self) -> &JsValue {
        Box::leak(Box::new(JsValue::from(self.clone())))
    }
}


#[wasm_bindgen(js_name = addFinalTip)]
pub fn add_final_tip_js(
    shape: WasmVectorObject,
    tip_side_length: f64,
    tip_color: WasmColor
) -> WasmVectorObject {
    WasmVectorObject {
        native_vec_features: add_final_tip(shape.native_vec_features, tip_side_length, (
            tip_color.color.red,
            tip_color.color.green,
            tip_color.color.blue,
            tip_color.color.alpha
        ))
    }
}


#[wasm_bindgen(js_name = addInitialTip)]
pub fn add_initial_tip_js(
    shape: WasmVectorObject,
    tip_side_length: f64,
    tip_color: WasmColor
) -> WasmVectorObject {
    WasmVectorObject {
        native_vec_features: add_initial_tip(shape.native_vec_features, tip_side_length, (
            tip_color.color.red,
            tip_color.color.green,
            tip_color.color.blue,
            tip_color.color.alpha
        ))
    }
}


#[wasm_bindgen(js_name = addBothSidesTips)]
pub fn add_both_sides_tips_js(
    shape: WasmVectorObject,
    tip_side_length: f64,
    tip_color: WasmColor
) -> WasmVectorObject {
    WasmVectorObject {
        native_vec_features: add_both_sides_tips(shape.native_vec_features, tip_side_length, (
            tip_color.color.red,
            tip_color.color.green,
            tip_color.color.blue,
            tip_color.color.alpha
        ))
    }
}


#[wasm_bindgen(js_name = arc)]
pub fn arc_js(
    center: Array,
    radius: f64,
    start_angle: f64,
    end_angle: f64,
    num_points: Option<usize>,
    stroke_color: Option<WasmColor>,
    fill_color: Option<WasmColor>,
    stroke_width: Option<f64>,
    line_cap: Option<String>,
    line_join: Option<String>,
    index: Option<usize>
) -> WasmVectorObject {
    let center = (center.get(0).as_f64().unwrap(), center.get(1).as_f64().unwrap());
    let stroke_color = match stroke_color {
        Some(color) => Some((
            color.color.red,
            color.color.green,
            color.color.blue,
            color.color.alpha
        )),
        None => None
    };
    let fill_color = match fill_color {
        Some(color) => Some((
            color.color.red,
            color.color.green,
            color.color.blue,
            color.color.alpha
        )),
        None => None
    };
    let stroke_width = match stroke_width {
        Some(width) => Some(width),
        None => None
    };
    let line_cap = match line_cap {
        Some(cap) => match cap.as_str() {
            "butt" => Some("butt"),
            "round" => Some("round"),
            "square" => Some("square"),
            _ => Some("butt")
        },
        None => None
    };
    let line_join = match line_join {
        Some(join) => match join.as_str() {
            "miter" => Some("miter"),
            "round" => Some("round"),
            "bevel" => Some("bevel"),
            _ => Some("miter")
        },
        None => None
    };
    let index = match index {
        Some(index) => Some(index),
        None => None
    };
    WasmVectorObject {
        native_vec_features: arc(
            center,
            radius,
            start_angle,
            end_angle,
            num_points,
            stroke_color,
            fill_color,
            stroke_width,
            line_cap,
            line_join,
            index
        )
    }
}


#[wasm_bindgen(js_name = circle)]
pub fn circle_js(
    center: Array,
    radius: f64,
    num_points: Option<usize>,
    stroke_color: Option<WasmColor>,
    fill_color: Option<WasmColor>,
    stroke_width: Option<f64>,
    line_cap: Option<String>,
    line_join: Option<String>,
    index: Option<usize>
) -> WasmVectorObject {
    let center = (center.get(0).as_f64().unwrap(), center.get(1).as_f64().unwrap());
    let stroke_color = match stroke_color {
        Some(color) => Some((
            color.color.red,
            color.color.green,
            color.color.blue,
            color.color.alpha
        )),
        None => None
    };
    let fill_color = match fill_color {
        Some(color) => Some((
            color.color.red,
            color.color.green,
            color.color.blue,
            color.color.alpha
        )),
        None => None
    };
    let stroke_width = match stroke_width {
        Some(width) => Some(width),
        None => None
    };
    let line_cap = match line_cap {
        Some(cap) => match cap.as_str() {
            "butt" => Some("butt"),
            "round" => Some("round"),
            "square" => Some("square"),
            _ => Some("butt")
        },
        None => None
    };
    let line_join = match line_join {
        Some(join) => match join.as_str() {
            "miter" => Some("miter"),
            "round" => Some("round"),
            "bevel" => Some("bevel"),
            _ => Some("miter")
        },
        None => None
    };
    let index = match index {
        Some(index) => Some(index),
        None => None
    };
    WasmVectorObject {
        native_vec_features: circle(
            center,
            radius,
            num_points,
            stroke_color,
            fill_color,
            stroke_width,
            line_cap,
            line_join,
            index
        )
    }
}


#[wasm_bindgen(js_name = ellipticalArc)]
pub fn elliptical_arc_js(
    center: Array,
    x_radius: f64,
    y_radius: f64,
    start_angle: f64,
    end_angle: f64,
    num_points: Option<usize>,
    stroke_color: Option<WasmColor>,
    fill_color: Option<WasmColor>,
    stroke_width: Option<f64>,
    line_cap: Option<String>,
    line_join: Option<String>,
    index: Option<usize>
) -> WasmVectorObject {
    let center = (center.get(0).as_f64().unwrap(), center.get(1).as_f64().unwrap());
    let stroke_color = match stroke_color {
        Some(color) => Some((
            color.color.red,
            color.color.green,
            color.color.blue,
            color.color.alpha
        )),
        None => None
    };
    let fill_color = match fill_color {
        Some(color) => Some((
            color.color.red,
            color.color.green,
            color.color.blue,
            color.color.alpha
        )),
        None => None
    };
    let stroke_width = match stroke_width {
        Some(width) => Some(width),
        None => None
    };
    let line_cap = match line_cap {
        Some(cap) => match cap.as_str() {
            "butt" => Some("butt"),
            "round" => Some("round"),
            "square" => Some("square"),
            _ => Some("butt")
        },
        None => None
    };
    let line_join = match line_join {
        Some(join) => match join.as_str() {
            "miter" => Some("miter"),
            "round" => Some("round"),
            "bevel" => Some("bevel"),
            _ => Some("miter")
        },
        None => None
    };
    let index = match index {
        Some(index) => Some(index),
        None => None
    };
    WasmVectorObject {
        native_vec_features: elliptical_arc(
            center,
            x_radius,
            y_radius,
            start_angle,
            end_angle,
            num_points,
            stroke_color,
            fill_color,
            stroke_width,
            line_cap,
            line_join,
            index
        )
    }
}


#[wasm_bindgen(js_name = ellipse)]
pub fn ellipse_js(
    center: Array,
    x_radius: f64,
    y_radius: f64,
    num_points: Option<usize>,
    stroke_color: Option<WasmColor>,
    fill_color: Option<WasmColor>,
    stroke_width: Option<f64>,
    line_cap: Option<String>,
    line_join: Option<String>,
    index: Option<usize>
) -> WasmVectorObject {
    let center = (center.get(0).as_f64().unwrap(), center.get(1).as_f64().unwrap());
    let stroke_color = match stroke_color {
        Some(color) => Some((
            color.color.red,
            color.color.green,
            color.color.blue,
            color.color.alpha
        )),
        None => None
    };
    let fill_color = match fill_color {
        Some(color) => Some((
            color.color.red,
            color.color.green,
            color.color.blue,
            color.color.alpha
        )),
        None => None
    };
    let stroke_width = match stroke_width {
        Some(width) => Some(width),
        None => None
    };
    let line_cap = match line_cap {
        Some(cap) => match cap.as_str() {
            "butt" => Some("butt"),
            "round" => Some("round"),
            "square" => Some("square"),
            _ => Some("butt")
        },
        None => None
    };
    let line_join = match line_join {
        Some(join) => match join.as_str() {
            "miter" => Some("miter"),
            "round" => Some("round"),
            "bevel" => Some("bevel"),
            _ => Some("miter")
        },
        None => None
    };
    let index = match index {
        Some(index) => Some(index),
        None => None
    };
    WasmVectorObject {
        native_vec_features: ellipse(
            center,
            x_radius,
            y_radius,
            num_points,
            stroke_color,
            fill_color,
            stroke_width,
            line_cap,
            line_join,
            index
        )
    }
}


#[wasm_bindgen(js_name = annularSector)]
pub fn annular_sector_js(
    center: Array,
    inner_radius: f64,
    outer_radius: f64,
    start_angle: f64,
    end_angle: f64,
    num_points: Option<usize>,
    stroke_color: Option<WasmColor>,
    fill_color: Option<WasmColor>,
    stroke_width: Option<f64>,
    line_cap: Option<String>,
    line_join: Option<String>,
    index: Option<usize>
) -> WasmVectorObject {
    let center = (center.get(0).as_f64().unwrap(), center.get(1).as_f64().unwrap());
    let stroke_color = match stroke_color {
        Some(color) => Some((
            color.color.red,
            color.color.green,
            color.color.blue,
            color.color.alpha
        )),
        None => None
    };
    let fill_color = match fill_color {
        Some(color) => Some((
            color.color.red,
            color.color.green,
            color.color.blue,
            color.color.alpha
        )),
        None => None
    };
    let stroke_width = match stroke_width {
        Some(width) => Some(width),
        None => None
    };
    let line_cap = match line_cap {
        Some(cap) => match cap.as_str() {
            "butt" => Some("butt"),
            "round" => Some("round"),
            "square" => Some("square"),
            _ => Some("butt")
        },
        None => None
    };
    let line_join = match line_join {
        Some(join) => match join.as_str() {
            "miter" => Some("miter"),
            "round" => Some("round"),
            "bevel" => Some("bevel"),
            _ => Some("miter")
        },
        None => None
    };
    let index = match index {
        Some(index) => Some(index),
        None => None
    };
    WasmVectorObject {
        native_vec_features: annular_sector(
            center,
            inner_radius,
            outer_radius,
            start_angle,
            end_angle,
            num_points,
            stroke_color,
            fill_color,
            stroke_width,
            line_cap,
            line_join,
            index
        )
    }
}


#[wasm_bindgen(js_name = line)]
pub fn line_js(
    start_point: Array,
    end_point: Array,
    stroke_color: Option<WasmColor>,
    stroke_width: Option<f64>,
    line_cap: Option<String>,
    line_join: Option<String>,
    index: Option<usize>
) -> WasmVectorObject {
    let start_point = (start_point.get(0).as_f64().unwrap(), start_point.get(1).as_f64().unwrap());
    let end_point = (end_point.get(0).as_f64().unwrap(), end_point.get(1).as_f64().unwrap());
    let stroke_color = match stroke_color {
        Some(color) => Some((
            color.color.red,
            color.color.green,
            color.color.blue,
            color.color.alpha
        )),
        None => None
    };
    let stroke_width = match stroke_width {
        Some(width) => Some(width),
        None => None
    };
    let line_cap = match line_cap {
        Some(cap) => match cap.as_str() {
            "butt" => Some("butt"),
            "round" => Some("round"),
            "square" => Some("square"),
            _ => Some("butt")
        },
        None => None
    };
    let line_join = match line_join {
        Some(join) => match join.as_str() {
            "miter" => Some("miter"),
            "round" => Some("round"),
            "bevel" => Some("bevel"),
            _ => Some("miter")
        },
        None => None
    };
    let index = match index {
        Some(index) => Some(index),
        None => None
    };
    WasmVectorObject {
        native_vec_features: line(
            start_point,
            end_point,
            stroke_color,
            stroke_width,
            line_cap,
            line_join,
            index
        )
    }
}


#[wasm_bindgen(js_name = polygon)]
pub fn polygon_js(
    points: Array,
    stroke_color: Option<WasmColor>,
    fill_color: Option<WasmColor>,
    stroke_width: Option<f64>,
    line_cap: Option<String>,
    line_join: Option<String>,
    index: Option<usize>
) -> WasmVectorObject {
    let points = points.iter().map(|point| {
        let point = point.dyn_into::<Array>().unwrap();
        let x = point.get(0).as_f64().unwrap();
        let y = point.get(1).as_f64().unwrap();
        (x, y)
    }).collect();
    let stroke_color = match stroke_color {
        Some(color) => Some((
            color.color.red,
            color.color.green,
            color.color.blue,
            color.color.alpha
        )),
        None => None
    };
    let fill_color = match fill_color {
        Some(color) => Some((
            color.color.red,
            color.color.green,
            color.color.blue,
            color.color.alpha
        )),
        None => None
    };
    let stroke_width = match stroke_width {
        Some(width) => Some(width),
        None => None
    };
    let line_cap = match line_cap {
        Some(cap) => match cap.as_str() {
            "butt" => Some("butt"),
            "round" => Some("round"),
            "square" => Some("square"),
            _ => Some("butt")
        },
        None => None
    };
    let line_join = match line_join {
        Some(join) => match join.as_str() {
            "miter" => Some("miter"),
            "round" => Some("round"),
            "bevel" => Some("bevel"),
            _ => Some("miter")
        },
        None => None
    };
    let index = match index {
        Some(index) => Some(index),
        None => None
    };
    WasmVectorObject {
        native_vec_features: polygon(
            points,
            stroke_color,
            fill_color,
            stroke_width,
            line_cap,
            line_join,
            index
        )
    }
}


#[wasm_bindgen(js_name = regularPolygon)]
pub fn regular_polygon_js(
    center: Array,
    side_length: f64,
    num_sides: usize,
    stroke_color: Option<WasmColor>,
    fill_color: Option<WasmColor>,
    stroke_width: Option<f64>,
    line_cap: Option<String>,
    line_join: Option<String>,
    index: Option<usize>
) -> WasmVectorObject {
    let center = (center.get(0).as_f64().unwrap(), center.get(1).as_f64().unwrap());
    let stroke_color = match stroke_color {
        Some(color) => Some((
            color.color.red,
            color.color.green,
            color.color.blue,
            color.color.alpha
        )),
        None => None
    };
    let fill_color = match fill_color {
        Some(color) => Some((
            color.color.red,
            color.color.green,
            color.color.blue,
            color.color.alpha
        )),
        None => None
    };
    let stroke_width = match stroke_width {
        Some(width) => Some(width),
        None => None
    };
    let line_cap = match line_cap {
        Some(cap) => match cap.as_str() {
            "butt" => Some("butt"),
            "round" => Some("round"),
            "square" => Some("square"),
            _ => Some("butt")
        },
        None => None
    };
    let line_join = match line_join {
        Some(join) => match join.as_str() {
            "miter" => Some("miter"),
            "round" => Some("round"),
            "bevel" => Some("bevel"),
            _ => Some("miter")
        },
        None => None
    };
    let index = match index {
        Some(index) => Some(index),
        None => None
    };
    WasmVectorObject {
        native_vec_features: regular_polygon(
            center,
            side_length,
            num_sides,
            stroke_color,
            fill_color,
            stroke_width,
            line_cap,
            line_join,
            index
        )
    }
}


#[wasm_bindgen(js_name = square)]
pub fn square_js(
    center: Array,
    side_length: f64,
    stroke_color: Option<WasmColor>,
    fill_color: Option<WasmColor>,
    stroke_width: Option<f64>,
    line_cap: Option<String>,
    line_join: Option<String>,
    index: Option<usize>
) -> WasmVectorObject {
    let center = (center.get(0).as_f64().unwrap(), center.get(1).as_f64().unwrap());
    let stroke_color = match stroke_color {
        Some(color) => Some((
            color.color.red,
            color.color.green,
            color.color.blue,
            color.color.alpha
        )),
        None => None
    };
    let fill_color = match fill_color {
        Some(color) => Some((
            color.color.red,
            color.color.green,
            color.color.blue,
            color.color.alpha
        )),
        None => None
    };
    let stroke_width = match stroke_width {
        Some(width) => Some(width),
        None => None
    };
    let line_cap = match line_cap {
        Some(cap) => match cap.as_str() {
            "butt" => Some("butt"),
            "round" => Some("round"),
            "square" => Some("square"),
            _ => Some("butt")
        },
        None => None
    };
    let line_join = match line_join {
        Some(join) => match join.as_str() {
            "miter" => Some("miter"),
            "round" => Some("round"),
            "bevel" => Some("bevel"),
            _ => Some("miter")
        },
        None => None
    };
    let index = match index {
        Some(index) => Some(index),
        None => None
    };
    WasmVectorObject {
        native_vec_features: square(
            center,
            side_length,
            stroke_color,
            fill_color,
            stroke_width,
            line_cap,
            line_join,
            index
        )
    }
}


#[wasm_bindgen(js_name = rectangle)]
pub fn rectangle_js(
    center: Array,
    width: f64,
    height: f64,
    stroke_color: Option<WasmColor>,
    fill_color: Option<WasmColor>,
    stroke_width: Option<f64>,
    line_cap: Option<String>,
    line_join: Option<String>,
    index: Option<usize>
) -> WasmVectorObject {
    let center = (center.get(0).as_f64().unwrap(), center.get(1).as_f64().unwrap());
    let stroke_color = match stroke_color {
        Some(color) => Some((
            color.color.red,
            color.color.green,
            color.color.blue,
            color.color.alpha
        )),
        None => None
    };
    let fill_color = match fill_color {
        Some(color) => Some((
            color.color.red,
            color.color.green,
            color.color.blue,
            color.color.alpha
        )),
        None => None
    };
    let stroke_width = match stroke_width {
        Some(width) => Some(width),
        None => None
    };
    let line_cap = match line_cap {
        Some(cap) => match cap.as_str() {
            "butt" => Some("butt"),
            "round" => Some("round"),
            "square" => Some("square"),
            _ => Some("butt")
        },
        None => None
    };
    let line_join = match line_join {
        Some(join) => match join.as_str() {
            "miter" => Some("miter"),
            "round" => Some("round"),
            "bevel" => Some("bevel"),
            _ => Some("miter")
        },
        None => None
    };
    let index = match index {
        Some(index) => Some(index),
        None => None
    };
    WasmVectorObject {
        native_vec_features: rectangle(
            center,
            width,
            height,
            stroke_color,
            fill_color,
            stroke_width,
            line_cap,
            line_join,
            index
        )
    }
}


#[wasm_bindgen(js_name = equilateralTriangle)]
pub fn equilateral_triangle_js(
    center: Array,
    side_length: f64,
    stroke_color: Option<WasmColor>,
    fill_color: Option<WasmColor>,
    stroke_width: Option<f64>,
    line_cap: Option<String>,
    line_join: Option<String>,
    index: Option<usize>
) -> WasmVectorObject {
    let center = (center.get(0).as_f64().unwrap(), center.get(1).as_f64().unwrap());
    let stroke_color = match stroke_color {
        Some(color) => Some((
            color.color.red,
            color.color.green,
            color.color.blue,
            color.color.alpha
        )),
        None => None
    };
    let fill_color = match fill_color {
        Some(color) => Some((
            color.color.red,
            color.color.green,
            color.color.blue,
            color.color.alpha
        )),
        None => None
    };
    let stroke_width = match stroke_width {
        Some(width) => Some(width),
        None => None
    };
    let line_cap = match line_cap {
        Some(cap) => match cap.as_str() {
            "butt" => Some("butt"),
            "round" => Some("round"),
            "square" => Some("square"),
            _ => Some("butt")
        },
        None => None
    };
    let line_join = match line_join {
        Some(join) => match join.as_str() {
            "miter" => Some("miter"),
            "round" => Some("round"),
            "bevel" => Some("bevel"),
            _ => Some("miter")
        },
        None => None
    };
    let index = match index {
        Some(index) => Some(index),
        None => None
    };
    WasmVectorObject {
        native_vec_features: equilateral_triangle(
            center,
            side_length,
            stroke_color,
            fill_color,
            stroke_width,
            line_cap,
            line_join,
            index
        )
    }
}


#[wasm_bindgen(js_name = triangle)]
pub fn triangle_js(
    point1: Array,
    point2: Array,
    point3: Array,
    stroke_color: Option<WasmColor>,
    fill_color: Option<WasmColor>,
    stroke_width: Option<f64>,
    line_cap: Option<String>,
    line_join: Option<String>,
    index: Option<usize>
) -> WasmVectorObject {
    let point1 = (point1.get(0).as_f64().unwrap(), point1.get(1).as_f64().unwrap());
    let point2 = (point2.get(0).as_f64().unwrap(), point2.get(1).as_f64().unwrap());
    let point3 = (point3.get(0).as_f64().unwrap(), point3.get(1).as_f64().unwrap());
    let stroke_color = match stroke_color {
        Some(color) => Some((
            color.color.red,
            color.color.green,
            color.color.blue,
            color.color.alpha
        )),
        None => None
    };
    let fill_color = match fill_color {
        Some(color) => Some((
            color.color.red,
            color.color.green,
            color.color.blue,
            color.color.alpha
        )),
        None => None
    };
    let stroke_width = match stroke_width {
        Some(width) => Some(width),
        None => None
    };
    let line_cap = match line_cap {
        Some(cap) => match cap.as_str() {
            "butt" => Some("butt"),
            "round" => Some("round"),
            "square" => Some("square"),
            _ => Some("butt")
        },
        None => None
    };
    let line_join = match line_join {
        Some(join) => match join.as_str() {
            "miter" => Some("miter"),
            "round" => Some("round"),
            "bevel" => Some("bevel"),
            _ => Some("miter")
        },
        None => None
    };
    let index = match index {
        Some(index) => Some(index),
        None => None
    };
    WasmVectorObject {
        native_vec_features: triangle(
            point1,
            point2,
            point3,
            stroke_color,
            fill_color,
            stroke_width,
            line_cap,
            line_join,
            index
        )
    }
}


#[wasm_bindgen(js_name = rightTriangle)]
pub fn right_triangle_js(
    point1: Array,
    point2: Array,
    stroke_color: Option<WasmColor>,
    fill_color: Option<WasmColor>,
    stroke_width: Option<f64>,
    line_cap: Option<String>,
    line_join: Option<String>,
    index: Option<usize>
) -> WasmVectorObject {
    let point1 = (point1.get(0).as_f64().unwrap(), point1.get(1).as_f64().unwrap());
    let point2 = (point2.get(0).as_f64().unwrap(), point2.get(1).as_f64().unwrap());
    let stroke_color = match stroke_color {
        Some(color) => Some((
            color.color.red,
            color.color.green,
            color.color.blue,
            color.color.alpha
        )),
        None => None
    };
    let fill_color = match fill_color {
        Some(color) => Some((
            color.color.red,
            color.color.green,
            color.color.blue,
            color.color.alpha
        )),
        None => None
    };
    let stroke_width = match stroke_width {
        Some(width) => Some(width),
        None => None
    };
    let line_cap = match line_cap {
        Some(cap) => match cap.as_str() {
            "butt" => Some("butt"),
            "round" => Some("round"),
            "square" => Some("square"),
            _ => Some("butt")
        },
        None => None
    };
    let line_join = match line_join {
        Some(join) => match join.as_str() {
            "miter" => Some("miter"),
            "round" => Some("round"),
            "bevel" => Some("bevel"),
            _ => Some("miter")
        },
        None => None
    };
    let index = match index {
        Some(index) => Some(index),
        None => None
    };
    WasmVectorObject {
        native_vec_features: right_triangle(
            point1,
            point2,
            stroke_color,
            fill_color,
            stroke_width,
            line_cap,
            line_join,
            index
        )
    }
}


#[wasm_bindgen(js_name = axes)]
pub fn axes_js(
    x_min: f64,
    x_max: f64,
    x_step: f64,
    y_min: f64,
    y_max: f64,
    y_step: f64,
    center: Array,
    x_length: Option<f64>,
    y_length: Option<f64>,
    color: Option<WasmColor>,
    stroke_width: Option<f64>,
    line_cap: Option<String>,
    line_join: Option<String>,
    index: Option<usize>,
    add_x_ticks: Option<bool>,
    add_y_ticks: Option<bool>,
    x_tick_size: Option<f64>,
    y_tick_size: Option<f64>,
    add_x_tip: Option<bool>,
    add_y_tip: Option<bool>
) -> WasmVectorObject {
    let color = match color {
        Some(color) => Some((
            color.color.red,
            color.color.green,
            color.color.blue,
            color.color.alpha
        )),
        None => None
    };
    let center = (center.get(0).as_f64().unwrap(), center.get(1).as_f64().unwrap());
    let line_cap = match line_cap {
        Some(cap) => match cap.as_str() {
            "butt" => Some("butt"),
            "round" => Some("round"),
            "square" => Some("square"),
            _ => Some("butt")
        },
        None => None
    };
    let line_join = match line_join {
        Some(join) => match join.as_str() {
            "miter" => Some("miter"),
            "round" => Some("round"),
            "bevel" => Some("bevel"),
            _ => Some("miter")
        },
        None => None
    };
    return WasmVectorObject {
        native_vec_features: axes(
            x_min,
            x_max,
            x_step,
            y_min,
            y_max,
            y_step,
            center,
            x_length,
            y_length,
            color,
            stroke_width,
            line_cap,
            line_join,
            index,
            add_x_ticks,
            add_y_ticks,
            x_tick_size,
            y_tick_size,
            add_x_tip,
            add_y_tip
        )
    }
}


#[wasm_bindgen(js_name = coordsToPoint)]
pub fn coords_to_point_js(
    axes: &WasmVectorObject,
    x: f64,
    y: f64,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64
) -> Array {
    let point = coords_to_point(&axes.native_vec_features, x, y, x_min, x_max, y_min, y_max);
    Array::of2(&point.0.into(), &point.1.into())
}


#[wasm_bindgen(js_name = pointToCoords)]
pub fn point_to_coords_js(
    axes: &WasmVectorObject,
    point: Array,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64
) -> Array {
    let point = (point.get(0).as_f64().unwrap(), point.get(1).as_f64().unwrap());
    let coords = point_to_coords(&axes.native_vec_features, point, x_min, x_max, y_min, y_max);
    Array::of2(&coords.0.into(), &coords.1.into())
}


#[wasm_bindgen(js_name = parametricPlotInAxes)]
pub fn parametric_plot_in_axes_js(
    f: &js_sys::Function,
    t_min: f64,
    t_max: f64,
    t_step: f64,
    axes: &WasmVectorObject,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    color: Option<WasmColor>,
    stroke_width: Option<f64>,
    line_cap: Option<String>,
    line_join: Option<String>,
    index: Option<usize>
) -> WasmVectorObject {
    let color = match color {
        Some(color) => Some((
            color.color.red,
            color.color.green,
            color.color.blue,
            color.color.alpha
        )),
        None => None
    };
    let line_cap = match line_cap {
        Some(cap) => match cap.as_str() {
            "butt" => Some("butt"),
            "round" => Some("round"),
            "square" => Some("square"),
            _ => Some("butt")
        },
        None => None
    };
    let line_join = match line_join {
        Some(join) => match join.as_str() {
            "miter" => Some("miter"),
            "round" => Some("round"),
            "bevel" => Some("bevel"),
            _ => Some("miter")
        },
        None => None
    };
    return WasmVectorObject {
        native_vec_features: parametric_plot_in_axes(
            |t| {
                let result = f.call1(&JsValue::NULL, &JsValue::from_f64(t)).unwrap();
                let arr = js_sys::Array::from(&result);
                (arr.get(0).as_f64().unwrap(), arr.get(1).as_f64().unwrap())
            },
            t_min,
            t_max,
            t_step,
            &axes.native_vec_features,
            x_min,
            x_max,
            y_min,
            y_max,
            color,
            stroke_width,
            line_cap,
            line_join,
            index
        )
    }
}


#[wasm_bindgen(js_name = plotInAxes)]
pub fn plot_in_axes_js(
    f: &js_sys::Function,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    x1: f64,
    x2: f64,
    x_step: f64,
    axes: &WasmVectorObject,
    color: Option<WasmColor>,
    stroke_width: Option<f64>,
    line_cap: Option<String>,
    line_join: Option<String>,
    index: Option<usize>
) -> WasmVectorObject {
    let color = match color {
        Some(color) => Some((
            color.color.red,
            color.color.green,
            color.color.blue,
            color.color.alpha
        )),
        None => None
    };
    let line_cap = match line_cap {
        Some(cap) => match cap.as_str() {
            "butt" => Some("butt"),
            "round" => Some("round"),
            "square" => Some("square"),
            _ => Some("butt")
        },
        None => None
    };
    let line_join = match line_join {
        Some(join) => match join.as_str() {
            "miter" => Some("miter"),
            "round" => Some("round"),
            "bevel" => Some("bevel"),
            _ => Some("miter")
        },
        None => None
    };
    return WasmVectorObject {
        native_vec_features: plot_in_axes(
            |x| f.call1(&JsValue::NULL, &JsValue::from_f64(x)).unwrap().as_f64().unwrap(),
            x_min,
            x_max,
            y_min,
            y_max,
            x1,
            x2,
            x_step,
            &axes.native_vec_features,
            color,
            stroke_width,
            line_cap,
            line_join,
            index
        )
    }
}


#[wasm_bindgen(js_name = contourPlotInAxes)]
pub fn contour_plot_in_axes_js(
    f: &js_sys::Function,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    x_1: f64,
    x_2: f64,
    x_step: f64,
    y_1: f64,
    y_2: f64,
    y_step: f64,
    axes: &WasmVectorObject,
    color: Option<WasmColor>,
    stroke_width: Option<f64>,
    line_cap: Option<String>,
    line_join: Option<String>,
    index: Option<usize>,
    intervals: &[f64]
) -> WasmVectorObject {
    let color = match color {
        Some(color) => Some((
            color.color.red,
            color.color.green,
            color.color.blue,
            color.color.alpha
        )),
        None => None
    };
    let line_cap = match line_cap {
        Some(cap) => match cap.as_str() {
            "butt" => Some("butt"),
            "round" => Some("round"),
            "square" => Some("square"),
            _ => Some("butt")
        },
        None => None
    };
    let line_join = match line_join {
        Some(join) => match join.as_str() {
            "miter" => Some("miter"),
            "round" => Some("round"),
            "bevel" => Some("bevel"),
            _ => Some("miter")
        },
        None => None
    };
    return WasmVectorObject {
        native_vec_features: contour_plot_in_axes(
            |x, y| f.call2(&JsValue::NULL, &JsValue::from_f64(x), &JsValue::from_f64(y)).unwrap().as_f64().unwrap(),
            x_min,
            x_max,
            y_min,
            y_max,
            x_1,
            x_2,
            x_step,
            y_1,
            y_2,
            y_step,
            &axes.native_vec_features,
            color,
            stroke_width,
            line_cap,
            line_join,
            index,
            intervals
        )
    }
}


#[wasm_bindgen(js_name = areaUnderCurve)]
pub fn area_under_curve_js(
    axes: &WasmVectorObject,
    plot: &WasmVectorObject,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    x1: f64,
    x2: f64,
    color: Option<WasmColor>,
    index: Option<usize>
) -> WasmVectorObject {
    let color = match color {
        Some(color) => Some((
            color.color.red,
            color.color.green,
            color.color.blue,
            color.color.alpha
        )),
        None => None
    };
    return WasmVectorObject {
        native_vec_features: area_under_curve(
            &axes.native_vec_features,
            &plot.native_vec_features,
            x_min,
            x_max,
            y_min,
            y_max,
            x1,
            x2,
            color,
            index
        )
    }
}


#[wasm_bindgen(js_name = riemannRectanglesForPlot)]
pub fn riemann_rectangles_for_plot_js(
    f: &js_sys::Function,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    direction: f64,
    x_1: f64,
    x_2: f64,
    n_rects: usize,
    axes: &WasmVectorObject,
    stroke_color: Option<WasmColor>,
    fill_color: Option<WasmColor>,
    stroke_width: Option<f64>,
    line_cap: Option<String>,
    line_join: Option<String>,
    index: Option<usize>
) -> WasmVectorObject {
    let stroke_color = match stroke_color {
        Some(color) => Some((
            color.color.red,
            color.color.green,
            color.color.blue,
            color.color.alpha
        )),
        None => None
    };
    let fill_color = match fill_color {
        Some(color) => Some((
            color.color.red,
            color.color.green,
            color.color.blue,
            color.color.alpha
        )),
        None => None
    };
    let line_cap = match line_cap {
        Some(cap) => match cap.as_str() {
            "butt" => Some("butt"),
            "round" => Some("round"),
            "square" => Some("square"),
            _ => Some("butt")
        },
        None => None
    };
    let line_join = match line_join {
        Some(join) => match join.as_str() {
            "miter" => Some("miter"),
            "round" => Some("round"),
            "bevel" => Some("bevel"),
            _ => Some("miter")
        },
        None => None
    };
    return WasmVectorObject {
        native_vec_features: riemann_rectangles_for_plot(
            |x| f.call1(&JsValue::NULL, &JsValue::from_f64(x)).unwrap().as_f64().unwrap(),
            x_min,
            x_max,
            y_min,
            y_max,
            direction,
            x_1,
            x_2,
            n_rects,
            &axes.native_vec_features,
            stroke_color,
            fill_color,
            stroke_width,
            line_cap,
            line_join,
            index
        )
    }
}


#[wasm_bindgen(js_name = secantLineForPlot)]
pub fn secant_line_for_plot_js(
    f: &js_sys::Function,
    x_1: f64,
    x_2: f64,
    length: f64,
    axes: &WasmVectorObject,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    color: Option<WasmColor>,
    stroke_width: Option<f64>,
    line_cap: Option<String>,
    line_join: Option<String>,
    index: Option<usize>
) -> WasmVectorObject {
    let color = match color {
        Some(color) => Some((
            color.color.red,
            color.color.green,
            color.color.blue,
            color.color.alpha
        )),
        None => None
    };
    let line_cap = match line_cap {
        Some(cap) => match cap.as_str() {
            "butt" => Some("butt"),
            "round" => Some("round"),
            "square" => Some("square"),
            _ => Some("butt")
        },
        None => None
    };
    let line_join = match line_join {
        Some(join) => match join.as_str() {
            "miter" => Some("miter"),
            "round" => Some("round"),
            "bevel" => Some("bevel"),
            _ => Some("miter")
        },
        None => None
    };
    return WasmVectorObject {
        native_vec_features: secant_line_for_plot(
            |x| f.call1(&JsValue::NULL, &JsValue::from_f64(x)).unwrap().as_f64().unwrap(),
            x_1,
            x_2,
            length,
            &axes.native_vec_features,
            x_min,
            x_max,
            y_min,
            y_max,
            color,
            stroke_width,
            line_cap,
            line_join,
            index
        )
    }
}


#[wasm_bindgen(js_name = parametricFunction)]
pub fn parametric_function_js(
    f: &js_sys::Function,
    t_min: f64,
    t_max: f64,
    t_step: f64,
    color: Option<WasmColor>,
    stroke_width: Option<f64>,
    line_cap: Option<String>,
    line_join: Option<String>,
    index: Option<usize>
) -> WasmVectorObject {
    let color = match color {
        Some(color) => Some((
            color.color.red,
            color.color.green,
            color.color.blue,
            color.color.alpha
        )),
        None => None
    };
    let line_cap = match line_cap {
        Some(cap) => match cap.as_str() {
            "butt" => Some("butt"),
            "round" => Some("round"),
            "square" => Some("square"),
            _ => Some("butt")
        },
        None => None
    };
    let line_join = match line_join {
        Some(join) => match join.as_str() {
            "miter" => Some("miter"),
            "round" => Some("round"),
            "bevel" => Some("bevel"),
            _ => Some("miter")
        },
        None => None
    };
    return WasmVectorObject {
        native_vec_features: parametric_function(
            |t| {
                let result = f.call1(&JsValue::NULL, &JsValue::from_f64(t)).unwrap();
                let arr = js_sys::Array::from(&result);
                (arr.get(0).as_f64().unwrap(), arr.get(1).as_f64().unwrap())
            },
            t_min,
            t_max,
            t_step,
            color,
            stroke_width,
            line_cap,
            line_join,
            index
        )
    }
}


#[wasm_bindgen(js_name = contourPlot)]
pub fn contour_plot_js(
    f: &js_sys::Function,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    x_step: f64,
    y_step: f64,
    color: Option<WasmColor>,
    stroke_width: Option<f64>,
    line_cap: Option<String>,
    line_join: Option<String>,
    index: Option<usize>,
    intervals: &[f64]
) -> WasmVectorObject {
    let color = match color {
        Some(color) => Some((
            color.color.red,
            color.color.green,
            color.color.blue,
            color.color.alpha
        )),
        None => None
    };
    let line_cap = match line_cap {
        Some(cap) => match cap.as_str() {
            "butt" => Some("butt"),
            "round" => Some("round"),
            "square" => Some("square"),
            _ => Some("butt")
        },
        None => None
    };
    let line_join = match line_join {
        Some(join) => match join.as_str() {
            "miter" => Some("miter"),
            "round" => Some("round"),
            "bevel" => Some("bevel"),
            _ => Some("miter")
        },
        None => None
    };
    return WasmVectorObject {
        native_vec_features: contour_plot(
            |x, y| f.call2(&JsValue::NULL, &JsValue::from_f64(x), &JsValue::from_f64(y)).unwrap().as_f64().unwrap(),
            x_min,
            x_max,
            y_min,
            y_max,
            x_step,
            y_step,
            color,
            stroke_width,
            line_cap,
            line_join,
            index,
            intervals
        )
    }
}


#[wasm_bindgen(js_name = realFunction)]
pub fn real_function_js(
    f: &js_sys::Function,
    x_min: f64,
    x_max: f64,
    x_step: f64,
    color: Option<WasmColor>,
    stroke_width: Option<f64>,
    line_cap: Option<String>,
    line_join: Option<String>,
    index: Option<usize>
) -> WasmVectorObject {
    let color = match color {
        Some(color) => Some((
            color.color.red,
            color.color.green,
            color.color.blue,
            color.color.alpha
        )),
        None => None
    };
    let line_cap = match line_cap {
        Some(cap) => match cap.as_str() {
            "butt" => Some("butt"),
            "round" => Some("round"),
            "square" => Some("square"),
            _ => Some("butt")
        },
        None => None
    };
    let line_join = match line_join {
        Some(join) => match join.as_str() {
            "miter" => Some("miter"),
            "round" => Some("round"),
            "bevel" => Some("bevel"),
            _ => Some("miter")
        },
        None => None
    };
    return WasmVectorObject {
        native_vec_features: function(
            |x| f.call1(&JsValue::NULL, &JsValue::from_f64(x)).unwrap().as_f64().unwrap(),
            x_min,
            x_max,
            x_step,
            color,
            stroke_width,
            line_cap,
            line_join,
            index
        )
    }
}


#[wasm_bindgen(js_name = numberLine)]
pub fn number_line_js(
    x_min: f64,
    x_max: f64,
    x_step: f64,
    color: Option<WasmColor>,
    stroke_width: Option<f64>,
    line_cap: Option<String>,
    line_join: Option<String>,
    index: Option<usize>,
    center: Option<Array>,
    length: Option<f64>,
    add_tip: Option<bool>,
    add_ticks: Option<bool>,
    tick_size: Option<f64>,
    angle: Option<f64>
) -> WasmVectorObject {
    let color = match color {
        Some(color) => Some((
            color.color.red,
            color.color.green,
            color.color.blue,
            color.color.alpha
        )),
        None => None
    };
    let line_cap = match line_cap {
        Some(cap) => match cap.as_str() {
            "butt" => Some("butt"),
            "round" => Some("round"),
            "square" => Some("square"),
            _ => Some("butt")
        },
        None => None
    };
    let line_join = match line_join {
        Some(join) => match join.as_str() {
            "miter" => Some("miter"),
            "round" => Some("round"),
            "bevel" => Some("bevel"),
            _ => Some("miter")
        },
        None => None
    };
    let center = match center {
        Some(center) => Some((center.get(0).as_f64().unwrap(), center.get(1).as_f64().unwrap())),
        None => None
    };
    return WasmVectorObject {
        native_vec_features: number_line(
            x_min,
            x_max,
            x_step,
            color,
            stroke_width,
            line_cap,
            line_join,
            index,
            center,
            length,
            add_tip,
            add_ticks,
            tick_size,
            angle
        )
    }
}


#[wasm_bindgen(js_name = numberToPoint)]
pub fn number_to_point_js(
    number_line: &WasmVectorObject,
    number: f64,
    x_min: f64,
    x_max: f64
) -> Array {
    let point = number_to_point(&number_line.native_vec_features, number, x_min, x_max);
    Array::of2(&point.0.into(), &point.1.into())
}


#[wasm_bindgen(js_name = pointToNumber)]
pub fn point_to_number_js(
    number_line: &WasmVectorObject,
    point: Array,
    x_min: f64,
    x_max: f64
) -> f64 {
    let point = (point.get(0).as_f64().unwrap(), point.get(1).as_f64().unwrap());
    point_to_number(&number_line.native_vec_features, point, x_min, x_max)
}


#[wasm_bindgen(js_name = getNumbersTex)]
pub async fn get_numbers_tex_js(
    number_line: WasmVectorObject,
    numbers: Array,
    number_to_vector: Function,
    x_min: f64,
    x_max: f64,
    height: f64,
    direction: Option<Array>,
    buff: Option<f64>,
    index: Option<usize>
) -> WasmVectorObject {
    let numbers = numbers.iter().map(|number| number.as_f64().unwrap()).collect();
    let direction = match direction {
        Some(direction) => Some((direction.get(0).as_f64().unwrap(), direction.get(1).as_f64().unwrap())),
        None => None
    };
    let index = match index {
        Some(index) => Some(index),
        None => None
    };
    return WasmVectorObject {
        native_vec_features: get_numbers_tex(
            number_line.native_vec_features,
            numbers,
            number_to_vector,
            x_min,
            x_max,
            height,
            direction,
            buff,
            index
        ).await
    }
}


#[wasm_bindgen(js_name = svgToVector)]
pub async fn svg_to_vector_js(
    svg: String,
    default_font_family: Option<String>,
    default_font_size: Option<f64>
) -> WasmVectorObject {
    let vec_obj = WasmVectorObject {
        native_vec_features: svg_to_vector_pin(&svg, default_font_family, default_font_size).await
    };
    vec_obj
}
