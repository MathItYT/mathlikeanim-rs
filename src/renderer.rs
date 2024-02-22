#[cfg(not(target_arch = "wasm32"))]
use std::io::Write;

#[cfg(not(target_arch = "wasm32"))]
use std::process::{Command, Stdio};

use crate::objects::vector_object::{
    generate_cubic_bezier_tuples, generate_subpaths_wasm, VectorFeatures
};
use crate::objects::vector_object::generate_subpaths;

use crate::utils::consider_points_equals;
#[cfg(not(target_arch = "wasm32"))]
use cairo::{Context, ImageSurface};
#[cfg(not(target_arch = "wasm32"))]
use indicatif::ProgressBar;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlCanvasElement;

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


pub fn vec_to_def_and_use_string(
    vec: &VectorFeatures,
    document: &web_sys::Document
) -> (String, String) {
    let mut def_string = "".to_string();
    let mut use_string = "".to_string();
    if vec.background_image.is_some() {
        let img = vec.background_image.as_ref().unwrap();
        let src = img.src();
        let image_svg = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "image").unwrap();
        image_svg.set_attribute("href", src.as_str()).unwrap();
        image_svg.set_attribute("x", &format!("{}", vec.image_position.0)).unwrap();
        image_svg.set_attribute("y", &format!("{}", vec.image_position.1)).unwrap();
        image_svg.set_attribute("width", &format!("{}", img.width())).unwrap();
        image_svg.set_attribute("height", &format!("{}", img.height())).unwrap();
        let image_id = format!("image_{}", uuid::Uuid::new_v4().to_string());
        let clip_path_id = format!("clip_{}", uuid::Uuid::new_v4().to_string());
        def_string.push_str(&format!("<clipPath id=\"{}\">\n", clip_path_id));
        if vec.points.len() > 0 {
            def_string.push_str("<path d=\"");
            let subpaths = generate_subpaths(&vec.points);
            for subpath in subpaths {
                let quads = generate_cubic_bezier_tuples(&subpath);
                let start = subpath[0];
                def_string.push_str(&format!("M {} {} ", start.0, start.1));
                for quad in quads {
                    let p1 = quad.1;
                    let p2 = quad.2;
                    let p3 = quad.3;
                    def_string.push_str(&format!("C {} {} {} {} {} {} ", p1.0, p1.1, p2.0, p2.1, p3.0, p3.1));
                }
                if consider_points_equals(subpath[0], subpath[subpath.len() - 1]) {
                    def_string.push_str("Z ");
                }
            }
        }
        def_string.push_str("\"/>\n");
        def_string.push_str("</clipPath>\n");
        use_string.push_str(&format!("<use href=\"#{}\" clip-path=\"url(#{})\"/>\n", image_id, clip_path_id));
        def_string.push_str(&format!("{}\n", image_svg.outer_html()));
    } else if vec.points.len() > 0 {
        def_string.push_str("<path d=\"");
        let subpaths = generate_subpaths(&vec.points);
        for subpath in subpaths {
            let quads = generate_cubic_bezier_tuples(&subpath);
            let start = subpath[0];
            def_string.push_str(&format!("M {} {} ", start.0, start.1));
            for quad in quads {
                let p1 = quad.1;
                let p2 = quad.2;
                let p3 = quad.3;
                def_string.push_str(&format!("C {} {} {} {} {} {} ", p1.0, p1.1, p2.0, p2.1, p3.0, p3.1));
            }
            if consider_points_equals(subpath[0], subpath[subpath.len() - 1]) {
                def_string.push_str("Z ");
            }
        }
        def_string.push_str("\" fill=\"rgba(");
        def_string.push_str(&format!("{}, {}, {}, {})", (vec.fill_color.0 * 255.0) as u8, (vec.fill_color.1 * 255.0) as u8, (vec.fill_color.2 * 255.0) as u8, vec.fill_color.3));
        def_string.push_str("\" stroke=\"rgba(");
        def_string.push_str(&format!("{}, {}, {}, {})", (vec.stroke_color.0 * 255.0) as u8, (vec.stroke_color.1 * 255.0) as u8, (vec.stroke_color.2 * 255.0) as u8, vec.stroke_color.3));
        def_string.push_str("\" stroke-width=\"");
        def_string.push_str(&format!("{}", vec.stroke_width));
        def_string.push_str("\" stroke-linecap=\"");
        def_string.push_str(&vec.line_cap);
        def_string.push_str("\" stroke-linejoin=\"");
        def_string.push_str(&vec.line_join);
        let id = format!("vector_{}", uuid::Uuid::new_v4().to_string());
        def_string.push_str(&format!("\" id=\"{}\"/>\n", id));
        use_string.push_str(&format!("<use href=\"#{}\"/>\n", id));
    }
    for subvec in &vec.subobjects {
        let (subdef_string, subuse_string) = vec_to_def_and_use_string(subvec, document);
        def_string.push_str(&subdef_string);
        use_string.push_str(&subuse_string);
    }
    return (def_string, use_string);
}


pub fn render_all_vectors_svg(
    vecs: &Vec<VectorFeatures>,
    width: u64,
    height: u64,
    background_color: (f64, f64, f64, f64),
    top_left_corner: (f64, f64),
    bottom_right_corner: (f64, f64),
    div_container_id: &str
) {
    let document = web_sys::window().unwrap().document().unwrap();
    let div = document.get_element_by_id(div_container_id).unwrap();
    div.set_inner_html("");
    let mut svg = format!("<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"{} {} {} {}\">", width, height, top_left_corner.0, top_left_corner.1, bottom_right_corner.0 - top_left_corner.0, bottom_right_corner.1 - top_left_corner.1);
    svg.push_str(&format!("<rect x=\"0\" y=\"0\" width=\"{}\" height=\"{}\" fill=\"rgba({}, {}, {}, {})\"/>", width, height, background_color.0 * 255.0, background_color.1 * 255.0, background_color.2 * 255.0, background_color.3));
    let mut defs = "<defs>\n".to_string();
    let mut use_strings = "".to_string();
    for vec in vecs {
        let (def_string, use_string) = vec_to_def_and_use_string(vec, &document);
        defs.push_str(&def_string);
        use_strings.push_str(&use_string);
    }
    defs.push_str("</defs>\n");
    svg.push_str(&defs);
    svg.push_str(&use_strings);
    svg.push_str("</svg>");
    div.set_inner_html(&svg);
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


pub fn render_vector_with_image_wasm(
    img: &web_sys::HtmlImageElement,
    context: web_sys::CanvasRenderingContext2d,
    points: &JsValue,
    fill_alpha: f64,
    stroke_alpha: f64,
    line_cap: String,
    line_join: String,
    stroke_width: f64,
    image_position: (f64, f64),
    width: u64,
    height: u64
) {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas_img = document.create_element("canvas").unwrap().dyn_into::<HtmlCanvasElement>().unwrap();
    canvas_img.set_width(width as u32);
    canvas_img.set_height(height as u32);
    let img_context = canvas_img.get_context("2d").unwrap().unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
    img_context.draw_image_with_html_image_element_and_dw_and_dh(&img, image_position.0, image_position.1, img.width() as f64, img.height() as f64).unwrap();
    let pattern = context.create_pattern_with_html_canvas_element(&canvas_img, "no-repeat").unwrap().unwrap();
    draw_context_path_wasm(&context, &points);
    context.set_global_alpha(fill_alpha);
    context.set_fill_style(&pattern);
    context.fill();
    context.set_global_alpha(stroke_alpha);
    context.set_stroke_style(&pattern);
    context.set_line_width(stroke_width);
    match line_cap.as_str() {
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
    match line_join.as_str() {
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
    context.set_global_alpha(1.0);
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
    if vec.background_image.is_some() && vec.points.len() > 0 {
        render_vector_with_image_wasm(vec.background_image.as_ref().unwrap(), context.clone(), &points, vec.fill_color.3, vec.stroke_color.3, line_cap, line_join, stroke_width, vec.image_position, width, height);
    } else {
        draw_context_path_wasm(&context, &points);
        apply_fill_wasm(&context, &fill_color, &points);
        apply_stroke_wasm(&context, &stroke_color, stroke_width, &line_cap, &line_join, &points);
    }
    for subvec in &vec.subobjects {
        render_vector_wasm(&subvec, width, height, context.clone());
    }
}


pub fn render_all_vectors(
    vecs: &Vec<VectorFeatures>,
    width: u64,
    height: u64,
    context: Option<web_sys::CanvasRenderingContext2d>,
    background_color: (f64, f64, f64, f64),
    top_left_corner: (f64, f64),
    bottom_right_corner: (f64, f64)
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
    context.reset_transform().unwrap();
    let scale_xy = (width as f64 / (bottom_right_corner.0 - top_left_corner.0), height as f64 / (bottom_right_corner.1 - top_left_corner.1));
    context.scale(scale_xy.0, scale_xy.1).unwrap();
    context.translate(-top_left_corner.0, -top_left_corner.1).unwrap();
    context.clear_rect(top_left_corner.0, top_left_corner.1, bottom_right_corner.0 - top_left_corner.0, bottom_right_corner.1 - top_left_corner.1);
    context.set_fill_style(&JsValue::from_str(&format!("rgba({}, {}, {}, {})", background_color.0 * 255.0, background_color.1 * 255.0, background_color.2 * 255.0, background_color.3)));
    context.fill_rect(top_left_corner.0, top_left_corner.1, bottom_right_corner.0 - top_left_corner.0, bottom_right_corner.1 - top_left_corner.1);
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