use std::io::Write;
use std::process::{Command, Stdio};
use crate::objects::vector_object::{
    VectorFeatures,
    get_subobjects_recursively,
    generate_subpaths,
    generate_cubic_bezier_tuples
};
use crate::utils::consider_points_equals;
use cairo::{Context, ImageSurface, ImageSurfaceDataOwned};
use indicatif::ProgressBar;


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


fn apply_fill(context: &Context, fill_color: &(f64, f64, f64, f64)) {
    context.set_source_rgba(fill_color.2, fill_color.1, fill_color.0, fill_color.3);
    context.fill_preserve().unwrap();
}


fn apply_stroke(context: &Context, stroke_color: &(f64, f64, f64, f64), stroke_width: f64, line_cap: &str, line_join: &str) {
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


fn render_vector(context: &Context, vec: &VectorFeatures, width: u64, height: u64) {
    let family = get_subobjects_recursively(&vec);
    let points = vec.points.clone();
    let fill_color = vec.fill_color;
    let stroke_color = vec.stroke_color;
    let stroke_width = vec.stroke_width;
    let line_cap = vec.line_cap;
    let line_join = vec.line_join;
    draw_context_path(&context, &points);
    apply_fill(&context, &fill_color);
    apply_stroke(&context, &stroke_color, stroke_width, &line_cap, &line_join);
    for subvec in family {
        render_vector(&context, &subvec, width, height);
    }
}


pub fn render_all_vectors(vecs: &Vec<VectorFeatures>, width: u64, height: u64) -> ImageSurfaceDataOwned {
    let surface = ImageSurface::create(cairo::Format::ARgb32, width as i32, height as i32).unwrap();
    let context = Context::new(&surface).unwrap();
    for vec in vecs {
        render_vector(&context, &vec, width, height);
    }
    drop(context);
    return surface.take_data().unwrap();
}


pub fn render_video(
    make_frame_function: &mut dyn FnMut(u64, u64, u64, u64) -> ImageSurfaceDataOwned,
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
        k.write(data.as_ref()).unwrap();
        progress_bar.inc(1);
    }
    child.wait().unwrap();
}


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