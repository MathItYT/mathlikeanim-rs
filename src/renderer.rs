#[cfg(not(target_arch = "wasm32"))]
use std::io::Write;

#[cfg(not(target_arch = "wasm32"))]
use std::process::{Command, Stdio};

use crate::objects::vector_object::{
    generate_cubic_bezier_tuples, generate_subpaths_wasm, VectorFeatures
};
#[cfg(not(target_arch = "wasm32"))]
use crate::objects::vector_object::generate_subpaths;

use crate::utils::consider_points_equals;
#[cfg(not(target_arch = "wasm32"))]
use cairo::{Context, ImageSurface};
#[cfg(not(target_arch = "wasm32"))]
use indicatif::ProgressBar;
use wasm_bindgen::JsValue;

#[cfg(not(target_arch = "wasm32"))]
fn draw_context_path(context: &Context, points: &Vec<(f64, f64)>) {
    if points.len() == 0 {
        return;
    }
    context.new_path();
    let subpaths = generate_subpaths(points);
    for subpath in subpaths {
        let quads = generate_cubic_bezier_tuples(&subpath);
        context.new_sub_path();
        let start = subpath[0];
        context.move_to(start.0, start.1);
        for quad in quads {
            let p1 = quad.1;
            let p2 = quad.2;
            let p3 = quad.3;
            context.curve_to(p1.0, p1.1, p2.0, p2.1, p3.0, p3.1);
        }
        if consider_points_equals(subpath[0], subpath[subpath.len() - 1]) {
            context.close_path();
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn apply_fill(context: &Context, fill_color: &(f64, f64, f64, f64), points: &Vec<(f64, f64)>) {
    if points.len() == 0 {
        return;
    }
    context.set_source_rgba(fill_color.2, fill_color.1, fill_color.0, fill_color.3);
    context.fill_preserve().unwrap();
}

#[cfg(not(target_arch = "wasm32"))]
fn apply_stroke(context: &Context, stroke_color: &(f64, f64, f64, f64), stroke_width: f64, line_cap: &str, line_join: &str, points: &Vec<(f64, f64)>) {
    if points.len() == 0 {
        return;
    }
    context.set_source_rgba(stroke_color.2, stroke_color.1, stroke_color.0, stroke_color.3);
    context.set_line_width(stroke_width);
    match line_cap {
        "butt" => {
            context.set_line_cap(cairo::LineCap::Butt);
        },
        "square" => {
            context.set_line_cap(cairo::LineCap::Square);
        },
        "round" => {
            context.set_line_cap(cairo::LineCap::Round);
        },
        _ => {
            panic!("Unknown line cap");
        }
    }
    match line_join {
        "miter" => {
            context.set_line_join(cairo::LineJoin::Miter);
        },
        "bevel" => {
            context.set_line_join(cairo::LineJoin::Bevel);
        },
        "round" => {
            context.set_line_join(cairo::LineJoin::Round);
        },
        _ => {
            panic!("Unknown line join");
        }
    }
    context.stroke_preserve().unwrap();
    context.set_line_cap(cairo::LineCap::Butt);
    context.set_line_join(cairo::LineJoin::Miter);
}

#[deprecated(note = "Cairo rendering is deprecated, please use WebAssembly with MediaRecorder instead")]
#[cfg(not(target_arch = "wasm32"))]
fn render_vector(context: &Context, vec: &VectorFeatures, width: u64, height: u64) {
    let points = vec.points.clone();
    let fill_color = vec.fill_color;
    let stroke_color = vec.stroke_color;
    let stroke_width = vec.stroke_width;
    let line_cap = vec.line_cap;
    let line_join = vec.line_join;
    draw_context_path(&context, &points);
    apply_fill(&context, &fill_color, &points);
    apply_stroke(&context, &stroke_color, stroke_width, &line_cap, &line_join, &points);
    for subvec in &vec.subobjects {
        render_vector(&context, &subvec, width, height);
    }
}


pub fn draw_context_path_wasm(
    context: &web_sys::CanvasRenderingContext2d,
    points: &JsValue
) {
    let points = js_sys::Array::from(&points.clone());
    if points.length() == 0 {
        return;
    }
    context.begin_path();
    let subpaths = generate_subpaths_wasm(points);
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
    context: &web_sys::CanvasRenderingContext2d,
    fill_color: &JsValue,
    points: &JsValue
) {
    let points = js_sys::Array::from(&points.clone());
    if points.length() == 0 {
        return;
    }
    let fill_color = js_sys::Array::from(&fill_color.clone());
    let r_string = js_sys::JsString::from(fill_color.get(0).as_string().unwrap());
    let g_string = js_sys::JsString::from(fill_color.get(1).as_string().unwrap());
    let b_string = js_sys::JsString::from(fill_color.get(2).as_string().unwrap());
    let a_string = js_sys::JsString::from(fill_color.get(3).as_string().unwrap());
    let fill_color = js_sys::JsString::from(format!("rgba({}, {}, {}, {})", r_string, g_string, b_string, a_string));
    context.set_fill_style(&fill_color);
    context.fill();
}


pub fn apply_stroke_wasm(
    context: &web_sys::CanvasRenderingContext2d,
    stroke_color: &JsValue,
    stroke_width: f64,
    line_cap: &str,
    line_join: &str,
    points: &JsValue
) {
    let points = js_sys::Array::from(&points.clone());
    if points.length() == 0 {
        return;
    }
    if stroke_width == 0.0 {
        return;
    }
    let stroke_color = js_sys::Array::from(&stroke_color.clone());
    let r_string = js_sys::JsString::from(stroke_color.get(0).as_string().unwrap());
    let g_string = js_sys::JsString::from(stroke_color.get(1).as_string().unwrap());
    let b_string = js_sys::JsString::from(stroke_color.get(2).as_string().unwrap());
    let a_string = js_sys::JsString::from(stroke_color.get(3).as_string().unwrap());
    let stroke_color = js_sys::JsString::from(format!("rgba({}, {}, {}, {})", r_string, g_string, b_string, a_string));
    context.set_stroke_style(&stroke_color);
    context.set_line_width(stroke_width);
    match line_cap {
        "butt" => {
            context.set_line_cap("butt");
        },
        "square" => {
            context.set_line_cap("square");
        },
        "round" => {
            context.set_line_cap("round");
        },
        _ => {
            panic!("Unknown line cap");
        }
    }
    match line_join {
        "miter" => {
            context.set_line_join("miter");
        },
        "bevel" => {
            context.set_line_join("bevel");
        },
        "round" => {
            context.set_line_join("round");
        },
        _ => {
            panic!("Unknown line join");
        }
    }
    context.stroke();
    context.set_line_cap("butt");
    context.set_line_join("miter");
}


pub fn render_vector_wasm(
    vec: &VectorFeatures,
    width: u64,
    height: u64,
    context: web_sys::CanvasRenderingContext2d
) {
    let points = serde_wasm_bindgen::to_value(&vec.points).unwrap();
    let fill_color = js_sys::Array::of4(
        &JsValue::from(((vec.fill_color.0 * 255.0) as u8).to_string()),
        &JsValue::from(((vec.fill_color.1 * 255.0) as u8).to_string()),
        &JsValue::from(((vec.fill_color.2 * 255.0) as u8).to_string()),
        &JsValue::from((vec.fill_color.3).to_string())
    );
    let stroke_color = js_sys::Array::of4(
        &JsValue::from(((vec.stroke_color.0 * 255.0) as u8).to_string()),
        &JsValue::from(((vec.stroke_color.1 * 255.0) as u8).to_string()),
        &JsValue::from(((vec.stroke_color.2 * 255.0) as u8).to_string()),
        &JsValue::from((vec.stroke_color.3).to_string())
    );
    let stroke_width = vec.stroke_width;
    let line_cap = vec.line_cap.to_string();
    let line_join = vec.line_join.to_string();
    draw_context_path_wasm(&context, &points);
    apply_fill_wasm(&context, &fill_color, &points);
    apply_stroke_wasm(&context, &stroke_color, stroke_width, &line_cap, &line_join, &points);
    for subvec in &vec.subobjects {
        render_vector_wasm(&subvec, width, height, context.clone());
    }
}


pub fn render_all_vectors(
    vecs: &Vec<VectorFeatures>,
    width: u64,
    height: u64,
    context: Option<web_sys::CanvasRenderingContext2d>,
    background_color: (f64, f64, f64, f64)
) -> Option<Vec<u8>> {
    #[cfg(not(target_arch = "wasm32"))]
    if context.is_none() {
        let surface = ImageSurface::create(cairo::Format::ARgb32, width as i32, height as i32).unwrap();
        let context = Context::new(&surface).unwrap();
        context.set_source_rgba(background_color.2, background_color.1, background_color.0, background_color.3);
        context.paint().unwrap();
        #[allow(deprecated)]
        for vec in vecs {
            render_vector(&context, &vec, width, height);
        }
        drop(context);
        return Some(surface.take_data().unwrap().to_vec());
    }
    let context = context.unwrap();
    context.clear_rect(0.0, 0.0, width as f64, height as f64);
    context.set_fill_style(&JsValue::from_str(&format!("rgba({}, {}, {}, {})", (background_color.0 * 255.0) as u8, (background_color.1 * 255.0) as u8, (background_color.2 * 255.0) as u8, background_color.3)));
    context.fill_rect(0.0, 0.0, width as f64, height as f64);
    for vec in vecs {
        render_vector_wasm(&vec, width, height, context.clone());
    }
    return None;
}

#[cfg(not(target_arch = "wasm32"))]
pub fn render_video(
    make_frame_function: &mut dyn FnMut(u64, u64, u64, u64) -> Option<Vec<u8>>,
    width: u64,
    height: u64,
    fps: u64,
    total_frames: u64,
    output_file: &str
) {
    let mut child = Command::new("ffmpeg")
        .args([
            "-y",
            "-f",
            "rawvideo",
            "-s", &format!("{}x{}", width, height),
            "-pix_fmt", "rgba",
            "-r", &format!("{}", fps),
            "-i",
            "-",
            "-loglevel","error",
            "-vcodec", "libx264", "-pix_fmt", "yuv420p",
            output_file
        ])
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();

    let k = child.stdin.as_mut().unwrap();
    let progress_bar = ProgressBar::new(total_frames);
    for i in 0..total_frames {
        let data = make_frame_function(i, width, height, total_frames);
        k.write(data.unwrap().as_slice()).unwrap();
        progress_bar.inc(1);
    }
    child.wait().unwrap();
}


#[cfg(not(target_arch = "wasm32"))]
pub fn concat_videos(files: Vec<String>, output_file: &str) {
    let mut input_files = Vec::new();
    for file in files.clone() {
        input_files.push("-i".to_string());
        input_files.push(file);
    }
    let mut args = vec![
        "-y".to_string()
    ];
    args.extend(input_files);
    args.extend(vec![
        "-filter_complex".to_string(),
        format!("concat=n={}:v=1:a=0", files.len()),
        "-loglevel".to_string(), "error".to_string(),
        output_file.to_string()
    ]);
    let mut child = Command::new("ffmpeg")
        .args(args)
        .spawn()
        .unwrap();
    child.wait().unwrap();    
}