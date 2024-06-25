use wasm_bindgen::prelude::*;

use crate::{colors::GradientImageOrColor, objects::vector_object::{generate_cubic_bezier_tuples, generate_subpaths, VectorFeatures}, utils::consider_points_equals};


#[wasm_bindgen(module = "buf")]
extern "C" {
    pub type Buffer;
}

#[wasm_bindgen(module = "canvas")]
extern "C" {
    pub type Image;
    pub type Canvas;
    pub type CanvasRenderingContext2D;
    pub type CanvasGradient;
    pub type CanvasPattern;
    #[wasm_bindgen(constructor)]
    fn new() -> Image;
    #[wasm_bindgen(method, setter, js_name = "src")]
    fn set_src(this: &Image, src: String);
    #[wasm_bindgen(js_name = "createCanvas")]
    fn create_canvas(width: u32, height: u32) -> Canvas;
    #[wasm_bindgen(method, js_name = getContext)]
    fn get_context(this: &Canvas, context: &str) -> CanvasRenderingContext2D;
    #[wasm_bindgen(method, js_name = "fillRect")]
    fn fill_rect(this: &CanvasRenderingContext2D, x: f64, y: f64, width: f64, height: f64);
    #[wasm_bindgen(method, js_name = "clearRect")]
    fn clear_rect(this: &CanvasRenderingContext2D, x: f64, y: f64, width: f64, height: f64);
    #[wasm_bindgen(method, js_name = "beginPath")]
    fn begin_path(this: &CanvasRenderingContext2D);
    #[wasm_bindgen(method, js_name = "moveTo")]
    fn move_to(this: &CanvasRenderingContext2D, x: f64, y: f64);
    #[wasm_bindgen(method, js_name = "bezierCurveTo")]
    fn bezier_curve_to(this: &CanvasRenderingContext2D, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64);
    #[wasm_bindgen(method, js_name = "closePath")]
    fn close_path(this: &CanvasRenderingContext2D);
    #[wasm_bindgen(method, setter, js_name = "fillStyle")]
    fn set_fill_style(this: &CanvasRenderingContext2D, style: String);
    #[wasm_bindgen(method, setter, js_name = "fillStyle")]
    fn set_fill_style_gradient(this: &CanvasRenderingContext2D, gradient: CanvasGradient);
    #[wasm_bindgen(method, setter, js_name = "fillStyle")]
    fn set_fill_style_pattern(this: &CanvasRenderingContext2D, pattern: CanvasPattern);
    #[wasm_bindgen(method, setter, js_name = "strokeStyle")]
    fn set_stroke_style(this: &CanvasRenderingContext2D, style: String);
    #[wasm_bindgen(method, setter, js_name = "strokeStyle")]
    fn set_stroke_style_gradient(this: &CanvasRenderingContext2D, gradient: CanvasGradient);
    #[wasm_bindgen(method, setter, js_name = "strokeStyle")]
    fn set_stroke_style_pattern(this: &CanvasRenderingContext2D, pattern: CanvasPattern);
    #[wasm_bindgen(method, js_name = "fill")]
    fn fill(this: &CanvasRenderingContext2D);
    #[wasm_bindgen(method, js_name = "stroke")]
    fn stroke(this: &CanvasRenderingContext2D);
    #[wasm_bindgen(method, setter, js_name = "lineWidth")]
    fn set_line_width(this: &CanvasRenderingContext2D, width: f64);
    #[wasm_bindgen(method, setter, js_name = "lineCap")]
    fn set_line_cap(this: &CanvasRenderingContext2D, cap: String);
    #[wasm_bindgen(method, setter, js_name = "lineJoin")]
    fn set_line_join(this: &CanvasRenderingContext2D, join: String);
    #[wasm_bindgen(method, setter, js_name = "width")]
    fn set_width(this: &Canvas, width: u32);
    #[wasm_bindgen(method, setter, js_name = "height")]
    fn set_height(this: &Canvas, height: u32);
    #[wasm_bindgen(method, js_name = "createLinearGradient")]
    fn create_linear_gradient(this: &CanvasRenderingContext2D, x0: f64, y0: f64, x1: f64, y1: f64) -> CanvasGradient;
    #[wasm_bindgen(method, js_name = "addColorStop")]
    fn add_color_stop(this: &CanvasGradient, offset: f32, color: String);
    #[wasm_bindgen(method, js_name = "createRadialGradient")]
    fn create_radial_gradient(this: &CanvasRenderingContext2D, x0: f64, y0: f64, r0: f64, x1: f64, y1: f64, r1: f64) -> CanvasGradient;
    #[wasm_bindgen(method, setter, js_name = "globalAlpha")]
    fn set_global_alpha(this: &CanvasRenderingContext2D, alpha: f64);
    #[wasm_bindgen(method, js_name = "drawImage")]
    fn draw_image_with_html_image_element_and_dw_and_dh(this: &CanvasRenderingContext2D, image: &Image, dx: f64, dy: f64, dw: f64, dh: f64);
    #[wasm_bindgen(method, js_name = "createPattern")]
    fn create_pattern_with_html_canvas_element(this: &CanvasRenderingContext2D, canvas: &Canvas, repetition: &str) -> CanvasPattern;
    #[wasm_bindgen(method, js_name = "resetTransform")]
    fn reset_transform(this: &CanvasRenderingContext2D);
    #[wasm_bindgen(method, js_name = "scale")]
    fn scale(this: &CanvasRenderingContext2D, x: f64, y: f64);
    #[wasm_bindgen(method, js_name = "translate")]
    fn translate(this: &CanvasRenderingContext2D, x: f64, y: f64);
    #[wasm_bindgen(method, getter, js_name = "canvas")]
    fn canvas(this: &CanvasRenderingContext2D) -> Canvas;
    #[wasm_bindgen(method, js_name = "toBuffer")]
    fn to_buffer(this: &Canvas, raw: &str) -> Buffer;
}


#[wasm_bindgen(module = "stream")]
extern "C" {
    pub type Writable;
    #[wasm_bindgen(method, js_name = write)]
    pub fn write(this: &Writable, data: Buffer);
    #[wasm_bindgen(method, js_name = write)]
    pub fn write_str(this: &Writable, data: &str);
    #[wasm_bindgen(method, js_name = end)]
    pub fn end(this: &Writable);
    #[wasm_bindgen(method, js_name = on)]
    pub fn pipe(this: &Writable, stream: &Writable);
}


#[wasm_bindgen(module = "child_process")]
extern "C" {
    pub type ChildProcess;
    #[wasm_bindgen(js_name = spawn)]
    pub fn spawn(command: &str, args: Vec<String>) -> ChildProcess;
    #[wasm_bindgen(method, getter, js_name = stdin)]
    pub fn stdin(this: &ChildProcess) -> Writable;
    #[wasm_bindgen(method, js_name = on)]
    pub fn on(this: &ChildProcess, event: &str, callback: &js_sys::Function);
}


#[wasm_bindgen(module = "process")]
extern "C" {
    #[wasm_bindgen(js_name = "exit")]
    pub fn exit(code: i32);
}


pub fn draw_context_path_wasm(
    context: &CanvasRenderingContext2D,
    points: Vec<(f64, f64)>
) {
    if points.len() == 0 {
        return;
    }
    context.begin_path();
    let subpaths = generate_subpaths(&points);
    for subpath in subpaths {
        let quads = generate_cubic_bezier_tuples(&subpath);
        let start = subpath[0];
        context.move_to(start.0, start.1);
        for quad in quads {
            let p1 = quad.1;
            let p2 = quad.2;
            let p3 = quad.3;
            context.bezier_curve_to(p1.0, p1.1, p2.0, p2.1, p3.0, p3.1);
        }
        if consider_points_equals(subpath[0], subpath[subpath.len() - 1]) {
            context.close_path();
        }
    }
}


pub fn apply_fill_wasm(
    context: &CanvasRenderingContext2D,
    fill: GradientImageOrColor,
    points: Vec<(f64, f64)>,
    width: u64,
    height: u64
) {
    if points.len() == 0 {
        return;
    }
    match fill {
        GradientImageOrColor::Color(color) => {
            let r_string = format!("{}", (color.red * 255.0) as u8);
            let g_string = format!("{}", (color.green * 255.0) as u8);
            let b_string = format!("{}", (color.blue * 255.0) as u8);
            let a_string = format!("{}", color.alpha);
            let fill_color = format!("rgba({}, {}, {}, {})", r_string, g_string, b_string, a_string);
            context.set_fill_style(fill_color);
            context.fill();
        },
        GradientImageOrColor::LinearGradient(gradient) => {
            let alpha = gradient.alpha;
            let grd = context.create_linear_gradient(gradient.x1, gradient.y1, gradient.x2, gradient.y2);
            for stop in gradient.stops.clone() {
                let r_string = format!("{}", (stop.color.red * 255.0) as u8);
                let g_string = format!("{}", (stop.color.green * 255.0) as u8);
                let b_string = format!("{}", (stop.color.blue * 255.0) as u8);
                let a_string = format!("{}", stop.color.alpha * alpha);
                let color = format!("rgba({}, {}, {}, {})", r_string, g_string, b_string, a_string);
                grd.add_color_stop(stop.offset as f32, color);
            }
            context.set_fill_style_gradient(grd);
            context.fill();
        },
        GradientImageOrColor::RadialGradient(gradient) => {
            let alpha = gradient.alpha;
            let grd = context.create_radial_gradient(gradient.fx, gradient.fy, 0.0, gradient.cx, gradient.cy, gradient.r);
            for stop in gradient.stops.clone() {
                let r_string = format!("{}", (stop.color.red * 255.0) as u8);
                let g_string = format!("{}", (stop.color.green * 255.0) as u8);
                let b_string = format!("{}", (stop.color.blue * 255.0) as u8);
                let a_string = format!("{}", stop.color.alpha * alpha);
                let color = format!("rgba({}, {}, {}, {})", r_string, g_string, b_string, a_string);
                grd.add_color_stop(stop.offset as f32, color);
            }
            context.set_fill_style_gradient(grd);
            context.fill();
        },
        GradientImageOrColor::Image(image) => {
            let img = Image::new();
            img.set_src(format!("data:{};base64,{}", image.mime_type, image.image_base64));
            let tl_corner = image.top_left_corner;
            let br_corner = image.bottom_right_corner;
            let w = br_corner.0 - tl_corner.0;
            let h= br_corner.1 - tl_corner.1;
            let alpha = image.alpha;
            let canvas2 = create_canvas(width as u32, height as u32);
            let context2 = canvas2.get_context("2d");
            context2.set_global_alpha(alpha);
            context2.draw_image_with_html_image_element_and_dw_and_dh(&img, tl_corner.0, tl_corner.1, w, h);
            let pattern = context.create_pattern_with_html_canvas_element(&canvas2, "repeat");
            context.set_fill_style_pattern(pattern);
            context.fill();
        }
    }
}


pub fn apply_stroke_wasm(
    context: &CanvasRenderingContext2D,
    stroke: GradientImageOrColor,
    stroke_width: f64,
    line_cap: String,
    line_join: String,
    points: Vec<(f64, f64)>,
    width: u64,
    height: u64
) {
    if points.len() == 0 {
        return;
    }
    match stroke {
        GradientImageOrColor::Color(color) => {
            let r_string = format!("{}", (color.red * 255.0) as u8);
            let g_string = format!("{}", (color.green * 255.0) as u8);
            let b_string = format!("{}", (color.blue * 255.0) as u8);
            let a_string = format!("{}", color.alpha);
            let stroke_color = format!("rgba({}, {}, {}, {})", r_string, g_string, b_string, a_string);
            context.set_stroke_style(stroke_color);
            context.set_line_width(stroke_width);
            context.set_line_cap(line_cap);
            context.set_line_join(line_join);
            context.stroke();
        },
        GradientImageOrColor::LinearGradient(gradient) => {
            let alpha = gradient.alpha;
            let grd = context.create_linear_gradient(gradient.x1, gradient.y1, gradient.x2, gradient.y2);
            for stop in gradient.stops.clone() {
                let r_string = format!("{}", (stop.color.red * 255.0) as u8);
                let g_string = format!("{}", (stop.color.green * 255.0) as u8);
                let b_string = format!("{}", (stop.color.blue * 255.0) as u8);
                let a_string = format!("{}", stop.color.alpha * alpha);
                let color = format!("rgba({}, {}, {}, {})", r_string, g_string, b_string, a_string);
                grd.add_color_stop(stop.offset as f32, color);
            }
            context.set_stroke_style_gradient(grd);
            context.set_line_width(stroke_width);
            context.set_line_cap(line_cap);
            context.set_line_join(line_join);
            context.stroke();
        },
        GradientImageOrColor::RadialGradient(gradient) => {
            let alpha = gradient.alpha;
            let grd = context.create_radial_gradient(gradient.fx, gradient.fy, 0.0, gradient.cx, gradient.cy, gradient.r);
            for stop in gradient.stops.clone() {
                let r_string = format!("{}", (stop.color.red * 255.0) as u8);
                let g_string = format!("{}", (stop.color.green * 255.0) as u8);
                let b_string = format!("{}", (stop.color.blue * 255.0) as u8);
                let a_string = format!("{}", stop.color.alpha * alpha);
                let color = format!("rgba({}, {}, {}, {})", r_string, g_string, b_string, a_string);
                grd.add_color_stop(stop.offset as f32, color);
            }
            context.set_stroke_style_gradient(grd);
            context.set_line_width(stroke_width);
            context.set_line_cap(line_cap);
            context.set_line_join(line_join);
            context.stroke();
        },
        GradientImageOrColor::Image(image) => {
            let img = Image::new();
            img.set_src(format!("data:{};base64,{}", image.mime_type, image.image_base64));
            let tl_corner = image.top_left_corner;
            let br_corner = image.bottom_right_corner;
            let w = br_corner.0 - tl_corner.0;
            let h= br_corner.1 - tl_corner.1;
            let alpha = image.alpha;
            let canvas2 = create_canvas(width as u32, height as u32);
            let context2 = canvas2.get_context("2d");
            context2.set_global_alpha(alpha);
            context2.draw_image_with_html_image_element_and_dw_and_dh(&img, tl_corner.0, tl_corner.1, w, h);
            let pattern = context.create_pattern_with_html_canvas_element(&canvas2, "repeat");
            context.set_stroke_style_pattern(pattern);
            context.set_line_width(stroke_width);
            context.set_line_cap(line_cap);
            context.set_line_join(line_join);
            context.stroke();
        }
    }
}


pub fn render_vector_wasm(
    vec: &VectorFeatures,
    width: u64,
    height: u64,
    context: &CanvasRenderingContext2D
) {
    let points = vec.points.clone();
    let fill = vec.fill.clone();
    let stroke = vec.stroke.clone();
    let stroke_width = vec.stroke_width;
    let line_cap = vec.line_cap;
    let line_join = vec.line_join;
    draw_context_path_wasm(&context, points.clone());
    apply_fill_wasm(&context, fill, points.clone(), width, height);
    apply_stroke_wasm(&context, stroke, stroke_width, line_cap.to_string(), line_join.to_string(), points.clone(), width, height);
    for subvec in &vec.subobjects {
        render_vector_wasm(&subvec, width, height, &context);
    }
}


pub fn render_all_vectors(
    vecs: &Vec<VectorFeatures>,
    width: u64,
    height: u64,
    context: &CanvasRenderingContext2D,
    background: GradientImageOrColor,
    top_left_corner: (f64, f64),
    bottom_right_corner: (f64, f64)
) {
    context.reset_transform();
    let scale_xy = (width as f64 / (bottom_right_corner.0 - top_left_corner.0), height as f64 / (bottom_right_corner.1 - top_left_corner.1));
    context.scale(scale_xy.0, scale_xy.1);
    context.translate(-top_left_corner.0, -top_left_corner.1);
    context.clear_rect(top_left_corner.0, top_left_corner.1, bottom_right_corner.0 - top_left_corner.0, bottom_right_corner.1 - top_left_corner.1);
    match background {
        GradientImageOrColor::Color(color) => {
            let fill_style = format!("rgba({}, {}, {}, {})", (color.red * 255.0) as u8, (color.green * 255.0) as u8, (color.blue * 255.0) as u8, color.alpha);
            context.set_fill_style(fill_style);
        }
        GradientImageOrColor::LinearGradient(gradient) => {
            let grd = context.create_linear_gradient(gradient.x1, gradient.y1, gradient.x2, gradient.y2);
            for stop in gradient.stops.clone() {
                let r_string = format!("{}", (stop.color.red * 255.0) as u8);
                let g_string = format!("{}", (stop.color.green * 255.0) as u8);
                let b_string = format!("{}", (stop.color.blue * 255.0) as u8);
                let a_string = format!("{}", stop.color.alpha);
                let color = format!("rgba({}, {}, {}, {})", r_string, g_string, b_string, a_string);
                grd.add_color_stop(stop.offset as f32, color);
            }
            context.set_fill_style_gradient(grd);
        },
        GradientImageOrColor::RadialGradient(gradient) => {
            let grd = context.create_radial_gradient(gradient.fx, gradient.fy, 0.0, gradient.cx, gradient.cy, gradient.r);
            for stop in gradient.stops.clone() {
                let r_string = format!("{}", (stop.color.red * 255.0) as u8);
                let g_string = format!("{}", (stop.color.green * 255.0) as u8);
                let b_string = format!("{}", (stop.color.blue * 255.0) as u8);
                let a_string = format!("{}", stop.color.alpha);
                let color = format!("rgba({}, {}, {}, {})", r_string, g_string, b_string, a_string);
                grd.add_color_stop(stop.offset as f32, color);
            }
            context.set_fill_style_gradient(grd);
        },
        GradientImageOrColor::Image(image) => {
            let img = Image::new();
            img.set_src(format!("data:{};base64,{}", image.mime_type, image.image_base64));
            let tl_corner = image.top_left_corner;
            let br_corner = image.bottom_right_corner;
            let w = br_corner.0 - tl_corner.0;
            let h = br_corner.1 - tl_corner.1;
            let alpha = image.alpha;
            let canvas2 = create_canvas(w as u32, h as u32);
            canvas2.set_width(w as u32);
            canvas2.set_height(h as u32);
            let context2 = canvas2.get_context("2d");
            context2.set_global_alpha(alpha);
            context2.draw_image_with_html_image_element_and_dw_and_dh(&img, tl_corner.0, tl_corner.1, w, h);
            let pattern = context.create_pattern_with_html_canvas_element(&canvas2, "repeat");
            context.set_fill_style_pattern(pattern);
        }
    };
    context.fill_rect(top_left_corner.0, top_left_corner.1, bottom_right_corner.0 - top_left_corner.0, bottom_right_corner.1 - top_left_corner.1);
    for vec in vecs {
        render_vector_wasm(&vec, width, height, &context);
    }
}


pub fn init_ffmpeg(width: u64, height: u64, fps: i32, codec: &str, pix_fmt: &str, file_name: &str, qp: &str) -> ChildProcess {
    let command = "ffmpeg";
    let args = vec![
        "-y".to_string(),
        "-f".to_string(),
        "rawvideo".to_string(),
        "-s".to_string(),
        format!("{}x{}", width, height),
        "-pix_fmt".to_string(),
        "bgra".to_string(),
        "-r".to_string(),
        format!("{}", fps),
        "-i".to_string(),
        "-".to_string(),
        "-an".to_string(),
        "-vcodec".to_string(),
        codec.to_string(),
        "-pix_fmt".to_string(),
        pix_fmt.to_string(),
        "-qp".to_string(),
        qp.to_string(),
        file_name.to_string()
    ];
    spawn(command, args)
}


pub fn write_frame_to_ffmpeg(
    context: &CanvasRenderingContext2D,
    ffmpeg_stream: &Writable
) {
    let buffer = context.canvas().to_buffer("raw");
    ffmpeg_stream.write(buffer);
}


pub fn end_ffmpeg(child: &ChildProcess) {
    child.stdin().end();
}
