use crate::colors::GradientImageOrColor;
use crate::objects::vector_object::{
    generate_cubic_bezier_tuples, VectorFeatures
};
use crate::objects::vector_object::generate_subpaths;

use crate::svg_scene::SVGScene;
use crate::utils::consider_points_equals;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;


pub fn draw_context_path_wasm(
    context: &web_sys::CanvasRenderingContext2d,
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


pub fn get_d_string_from_points(
    points: &Vec<(f64, f64)>
) -> String {
    let mut d = "".to_string();
    if points.len() == 0 {
        return d;
    }
    let subpaths = generate_subpaths(points);
    for subpath in subpaths {
        let quads = generate_cubic_bezier_tuples(&subpath);
        let start = subpath[0];
        d.push_str(format!("M {} {} ", start.0, start.1).as_str());
        for quad in quads {
            let p1 = quad.1;
            let p2 = quad.2;
            let p3 = quad.3;
            d.push_str(format!("C {} {} {} {} {} {} ", p1.0, p1.1, p2.0, p2.1, p3.0, p3.1).as_str());
        }
        if consider_points_equals(subpath[0], subpath[subpath.len() - 1]) {
            d.push_str("Z ");
        }
    }
    d = d.trim().to_string();
    return d;
}


pub fn vec_to_def_and_use_string(
    vec: &VectorFeatures,
    document: &web_sys::Document
) -> (String, String) {
    let mut def_string = "".to_string();
    let mut use_string = "".to_string();
    if vec.points.len() > 0 {
        #[allow(unused_assignments)]
        let mut fill = "".to_string();
        #[allow(unused_assignments)]
        let mut stroke = "".to_string();
        match &vec.fill {
            GradientImageOrColor::Color(color) => {
                fill = format!("rgba({}, {}, {}, {})", (color.red * 255.0) as u8, (color.green * 255.0) as u8, (color.blue * 255.0) as u8, color.alpha);
            },
            GradientImageOrColor::LinearGradient(gradient) => {
                let alpha = gradient.alpha;
                let grd = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "linearGradient").unwrap();
                let id = format!("gradient_{}", uuid::Uuid::new_v4());
                grd.set_attribute("id", &id).unwrap();
                grd.set_attribute("x1", &gradient.x1.to_string()).unwrap();
                grd.set_attribute("y1", &gradient.y1.to_string()).unwrap();
                grd.set_attribute("x2", &gradient.x2.to_string()).unwrap();
                grd.set_attribute("y2", &gradient.y2.to_string()).unwrap();
                grd.set_attribute("gradientUnits", "userSpaceOnUse").unwrap();
                for stop in gradient.stops.clone() {
                    let stop_element = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "stop").unwrap();
                    stop_element.set_attribute("offset", &stop.offset.to_string()).unwrap();
                    let r_string = format!("{}", (stop.color.red * 255.0) as u8);
                    let g_string = format!("{}", (stop.color.green * 255.0) as u8);
                    let b_string = format!("{}", (stop.color.blue * 255.0) as u8);
                    let a_string = format!("{}", stop.color.alpha * alpha);
                    let color = format!("rgba({}, {}, {}, {})", r_string, g_string, b_string, a_string);
                    stop_element.set_attribute("stop-color", &color).unwrap();
                    grd.append_child(&stop_element).unwrap();
                }
                def_string.push_str((grd.outer_html() + "\n").as_str());
                fill = format!("url(#{})", id);
            },
            GradientImageOrColor::RadialGradient(gradient) => {
                let alpha = gradient.alpha;
                let grd = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "radialGradient").unwrap();
                let id = format!("gradient_{}", uuid::Uuid::new_v4());
                grd.set_attribute("id", &id).unwrap();
                grd.set_attribute("cx", &gradient.cx.to_string()).unwrap();
                grd.set_attribute("cy", &gradient.cy.to_string()).unwrap();
                grd.set_attribute("r", &gradient.r.to_string()).unwrap();
                grd.set_attribute("fx", &gradient.fx.to_string()).unwrap();
                grd.set_attribute("fy", &gradient.fy.to_string()).unwrap();
                grd.set_attribute("gradientUnits", "userSpaceOnUse").unwrap();
                for stop in gradient.stops.clone() {
                    let stop_element = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "stop").unwrap();
                    stop_element.set_attribute("offset", &stop.offset.to_string()).unwrap();
                    let r_string = format!("{}", (stop.color.red * 255.0) as u8);
                    let g_string = format!("{}", (stop.color.green * 255.0) as u8);
                    let b_string = format!("{}", (stop.color.blue * 255.0) as u8);
                    let a_string = format!("{}", stop.color.alpha * alpha);
                    let color = format!("rgba({}, {}, {}, {})", r_string, g_string, b_string, a_string);
                    stop_element.set_attribute("stop-color", &color).unwrap();
                    grd.append_child(&stop_element).unwrap();
                }
                def_string.push_str((grd.outer_html() + "\n").as_str());
                fill = format!("url(#{})", id);
            },
            GradientImageOrColor::Image(image) => {
                let img = image.image.clone();
                let top_left_corner = image.top_left_corner;
                let bottom_right_corner = image.bottom_right_corner;
                let alpha = image.alpha;
                let x = top_left_corner.0.to_string();
                let y = top_left_corner.1.to_string();
                let width = (bottom_right_corner.0 - top_left_corner.0).to_string();
                let height = (bottom_right_corner.1 - top_left_corner.1).to_string();
                let pattern_id = format!("pattern_{}", uuid::Uuid::new_v4());
                let pattern = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "pattern").unwrap();
                pattern.set_attribute("id", &pattern_id).unwrap();
                pattern.set_attribute("x", &x).unwrap();
                pattern.set_attribute("y", &y).unwrap();
                pattern.set_attribute("width", &width).unwrap();
                pattern.set_attribute("height", &height).unwrap();
                let img_element = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "image").unwrap();
                img_element.set_attribute("x", &x).unwrap();
                img_element.set_attribute("y", &y).unwrap();
                img_element.set_attribute("width", &width).unwrap();
                img_element.set_attribute("height", &height).unwrap();
                img_element.set_attribute("href", &img.src()).unwrap();
                img_element.set_attribute("opacity", &alpha.to_string()).unwrap();
                pattern.append_child(&img_element).unwrap();
                def_string.push_str((pattern.outer_html() + "\n").as_str());
                fill = format!("url(#{})", pattern_id);
            },
        }
        match &vec.stroke {
            GradientImageOrColor::Color(color) => {
                stroke = format!("rgba({}, {}, {}, {})", (color.red * 255.0) as u8, (color.green * 255.0) as u8, (color.blue * 255.0) as u8, color.alpha);
            },
            GradientImageOrColor::LinearGradient(gradient) => {
                let alpha = gradient.alpha;
                let grd = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "linearGradient").unwrap();
                let id = format!("gradient_{}", uuid::Uuid::new_v4());
                grd.set_attribute("id", &id).unwrap();
                grd.set_attribute("x1", &gradient.x1.to_string()).unwrap();
                grd.set_attribute("y1", &gradient.y1.to_string()).unwrap();
                grd.set_attribute("x2", &gradient.x2.to_string()).unwrap();
                grd.set_attribute("y2", &gradient.y2.to_string()).unwrap();
                grd.set_attribute("gradientUnits", "userSpaceOnUse").unwrap();
                for stop in gradient.stops.clone() {
                    let stop_element = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "stop").unwrap();
                    stop_element.set_attribute("offset", &stop.offset.to_string()).unwrap();
                    let r_string = format!("{}", (stop.color.red * 255.0) as u8);
                    let g_string = format!("{}", (stop.color.green * 255.0) as u8);
                    let b_string = format!("{}", (stop.color.blue * 255.0) as u8);
                    let a_string = format!("{}", stop.color.alpha * alpha);
                    let color = format!("rgba({}, {}, {}, {})", r_string, g_string, b_string, a_string);
                    stop_element.set_attribute("stop-color", &color).unwrap();
                    grd.append_child(&stop_element).unwrap();
                }
                def_string.push_str((grd.outer_html() + "\n").as_str());
                stroke = format!("url(#{})", id);
            },
            GradientImageOrColor::RadialGradient(gradient) => {
                let alpha = gradient.alpha;
                let grd = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "radialGradient").unwrap();
                let id = format!("gradient_{}", uuid::Uuid::new_v4());
                grd.set_attribute("id", &id).unwrap();
                grd.set_attribute("cx", &gradient.cx.to_string()).unwrap();
                grd.set_attribute("cy", &gradient.cy.to_string()).unwrap();
                grd.set_attribute("r", &gradient.r.to_string()).unwrap();
                grd.set_attribute("fx", &gradient.fx.to_string()).unwrap();
                grd.set_attribute("fy", &gradient.fy.to_string()).unwrap();
                grd.set_attribute("gradientUnits", "userSpaceOnUse").unwrap();
                for stop in gradient.stops.clone() {
                    let stop_element = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "stop").unwrap();
                    stop_element.set_attribute("offset", &stop.offset.to_string()).unwrap();
                    let r_string = format!("{}", (stop.color.red * 255.0) as u8);
                    let g_string = format!("{}", (stop.color.green * 255.0) as u8);
                    let b_string = format!("{}", (stop.color.blue * 255.0) as u8);
                    let a_string = format!("{}", stop.color.alpha * alpha);
                    let color = format!("rgba({}, {}, {}, {})", r_string, g_string, b_string, a_string);
                    stop_element.set_attribute("stop-color", &color).unwrap();
                    grd.append_child(&stop_element).unwrap();
                }
                def_string.push_str((grd.outer_html() + "\n").as_str());
                stroke = format!("url(#{})", id);
            },
            GradientImageOrColor::Image(image) => {
                let img = image.image.clone();
                let top_left_corner = image.top_left_corner;
                let bottom_right_corner = image.bottom_right_corner;
                let alpha = image.alpha;
                let x = top_left_corner.0.to_string();
                let y = top_left_corner.1.to_string();
                let width = (bottom_right_corner.0 - top_left_corner.0).to_string();
                let height = (bottom_right_corner.1 - top_left_corner.1).to_string();
                let pattern_id = format!("pattern_{}", uuid::Uuid::new_v4());
                let pattern = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "pattern").unwrap();
                pattern.set_attribute("id", &pattern_id).unwrap();
                pattern.set_attribute("x", &x).unwrap();
                pattern.set_attribute("y", &y).unwrap();
                pattern.set_attribute("width", &width).unwrap();
                pattern.set_attribute("height", &height).unwrap();
                let img_element = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "image").unwrap();
                img_element.set_attribute("x", &x).unwrap();
                img_element.set_attribute("y", &y).unwrap();
                img_element.set_attribute("width", &width).unwrap();
                img_element.set_attribute("height", &height).unwrap();
                img_element.set_attribute("href", &img.src()).unwrap();
                img_element.set_attribute("opacity", &alpha.to_string()).unwrap();
                pattern.append_child(&img_element).unwrap();
                def_string.push_str((pattern.outer_html() + "\n").as_str());
                stroke = format!("url(#{})", pattern_id);
            },
        }
        let path_id = format!("path_{}", uuid::Uuid::new_v4());
        let d = get_d_string_from_points(&vec.points);
        let path = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path").unwrap();
        path.set_attribute("id", &path_id).unwrap();
        path.set_attribute("d", &d).unwrap();
        path.set_attribute("fill", &fill).unwrap();
        path.set_attribute("stroke", &stroke).unwrap();
        path.set_attribute("stroke-width", &vec.stroke_width.to_string()).unwrap();
        path.set_attribute("stroke-linecap", &vec.line_cap).unwrap();
        path.set_attribute("stroke-linejoin", &vec.line_join).unwrap();
        def_string.push_str((path.outer_html() + "\n").as_str());
        use_string.push_str(format!("<use href=\"#{}\"/>\n", path_id).as_str());
    }
    for subvec in &vec.subobjects {
        let (subdef_string, subuse_string) = vec_to_def_and_use_string(subvec, document);
        def_string.push_str(&subdef_string);
        use_string.push_str(&subuse_string);
    }
    return (def_string, use_string);
}


pub fn render_all_vectors_svg(
    svg_scene: &SVGScene
) {
    let vecs = svg_scene.objects.clone();
    let width = svg_scene.width;
    let height = svg_scene.height;
    let background = svg_scene.background.clone();
    let top_left_corner = svg_scene.top_left_corner;
    let bottom_right_corner = svg_scene.bottom_right_corner;
    let div = svg_scene.div_container.clone().unwrap();
    let document = web_sys::window().unwrap().document().unwrap();
    div.set_inner_html("");
    let mut svg = format!("<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"{} {} {} {}\">", width, height, top_left_corner.0, top_left_corner.1, bottom_right_corner.0 - top_left_corner.0, bottom_right_corner.1 - top_left_corner.1);
    let mut defs = "<defs>\n".to_string();
    let mut use_strings = "".to_string();
    #[allow(unused_assignments)]
    let mut rec_fill = "".to_string();
    match background {
        GradientImageOrColor::Color(color) => {
            rec_fill = format!("rgba({}, {}, {}, {})", (color.red * 255.0) as u8, (color.green * 255.0) as u8, (color.blue * 255.0) as u8, color.alpha);
        },
        GradientImageOrColor::LinearGradient(gradient) => {
            let alpha = gradient.alpha;
            let grd = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "linearGradient").unwrap();
            let id = format!("gradient_{}", uuid::Uuid::new_v4());
            grd.set_attribute("id", &id).unwrap();
            grd.set_attribute("x1", &gradient.x1.to_string()).unwrap();
            grd.set_attribute("y1", &gradient.y1.to_string()).unwrap();
            grd.set_attribute("x2", &gradient.x2.to_string()).unwrap();
            grd.set_attribute("y2", &gradient.y2.to_string()).unwrap();
            grd.set_attribute("gradientUnits", "userSpaceOnUse").unwrap();
            for stop in gradient.stops.clone() {
                let stop_element = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "stop").unwrap();
                stop_element.set_attribute("offset", &stop.offset.to_string()).unwrap();
                let r_string = format!("{}", (stop.color.red * 255.0) as u8);
                let g_string = format!("{}", (stop.color.green * 255.0) as u8);
                let b_string = format!("{}", (stop.color.blue * 255.0) as u8);
                let a_string = format!("{}", stop.color.alpha * alpha);
                let color = format!("rgba({}, {}, {}, {})", r_string, g_string, b_string, a_string);
                stop_element.set_attribute("stop-color", &color).unwrap();
                grd.append_child(&stop_element).unwrap();
            }
            defs.push_str((grd.outer_html() + "\n").as_str());
            rec_fill = format!("url(#{})", id);
        },
        GradientImageOrColor::RadialGradient(gradient) => {
            let alpha = gradient.alpha;
            let grd = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "radialGradient").unwrap();
            let id = format!("gradient_{}", uuid::Uuid::new_v4());
            grd.set_attribute("id", &id).unwrap();
            grd.set_attribute("cx", &gradient.cx.to_string()).unwrap();
            grd.set_attribute("cy", &gradient.cy.to_string()).unwrap();
            grd.set_attribute("r", &gradient.r.to_string()).unwrap();
            grd.set_attribute("fx", &gradient.fx.to_string()).unwrap();
            grd.set_attribute("fy", &gradient.fy.to_string()).unwrap();
            grd.set_attribute("gradientUnits", "userSpaceOnUse").unwrap();
            for stop in gradient.stops.clone() {
                let stop_element = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "stop").unwrap();
                stop_element.set_attribute("offset", &stop.offset.to_string()).unwrap();
                let r_string = format!("{}", (stop.color.red * 255.0) as u8);
                let g_string = format!("{}", (stop.color.green * 255.0) as u8);
                let b_string = format!("{}", (stop.color.blue * 255.0) as u8);
                let a_string = format!("{}", stop.color.alpha * alpha);
                let color = format!("rgba({}, {}, {}, {})", r_string, g_string, b_string, a_string);
                stop_element.set_attribute("stop-color", &color).unwrap();
                grd.append_child(&stop_element).unwrap();
            }
            defs.push_str((grd.outer_html() + "\n").as_str());
            rec_fill = format!("url(#{})", id);
        },
        GradientImageOrColor::Image(image) => {
            let img = image.image.clone();
            let top_left_corner = image.top_left_corner;
            let bottom_right_corner = image.bottom_right_corner;
            let alpha = image.alpha;
            let x = top_left_corner.0.to_string();
            let y = top_left_corner.1.to_string();
            let width = (bottom_right_corner.0 - top_left_corner.0).to_string();
            let height = (bottom_right_corner.1 - top_left_corner.1).to_string();
            let pattern_id = format!("pattern_{}", uuid::Uuid::new_v4());
            let pattern = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "pattern").unwrap();
            pattern.set_attribute("id", &pattern_id).unwrap();
            pattern.set_attribute("x", &x).unwrap();
            pattern.set_attribute("y", &y).unwrap();
            pattern.set_attribute("width", &width).unwrap();
            pattern.set_attribute("height", &height).unwrap();
            let img_element = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "image").unwrap();
            img_element.set_attribute("x", &x).unwrap();
            img_element.set_attribute("y", &y).unwrap();
            img_element.set_attribute("width", &width).unwrap();
            img_element.set_attribute("height", &height).unwrap();
            img_element.set_attribute("href", &img.src()).unwrap();
            img_element.set_attribute("opacity", &alpha.to_string()).unwrap();
            pattern.append_child(&img_element).unwrap();
            defs.push_str((pattern.outer_html() + "\n").as_str());
            rec_fill = format!("url(#{})", pattern_id);
        },
    }
    svg.push_str(format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\"/>\n", top_left_corner.0, top_left_corner.1, bottom_right_corner.0 - top_left_corner.0, bottom_right_corner.1 - top_left_corner.1, rec_fill).as_str());
    for vec in vecs {
        let (def_string, use_string) = vec_to_def_and_use_string(&vec, &document);
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
            let fill_color = js_sys::JsString::from(format!("rgba({}, {}, {}, {})", r_string, g_string, b_string, a_string));
            context.set_fill_style(&fill_color);
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
                grd.add_color_stop(stop.offset as f32, &color).unwrap();
            }
            context.set_fill_style(&grd);
            context.fill();
        },
        GradientImageOrColor::RadialGradient(gradient) => {
            let alpha = gradient.alpha;
            let grd = context.create_radial_gradient(gradient.fx, gradient.fy, 0.0, gradient.cx, gradient.cy, gradient.r).unwrap();
            for stop in gradient.stops.clone() {
                let r_string = format!("{}", (stop.color.red * 255.0) as u8);
                let g_string = format!("{}", (stop.color.green * 255.0) as u8);
                let b_string = format!("{}", (stop.color.blue * 255.0) as u8);
                let a_string = format!("{}", stop.color.alpha * alpha);
                let color = format!("rgba({}, {}, {}, {})", r_string, g_string, b_string, a_string);
                grd.add_color_stop(stop.offset as f32, &color).unwrap();
            }
            context.set_fill_style(&grd);
            context.fill();
        },
        GradientImageOrColor::Image(image) => {
            let img = image.image;
            let tl_corner = image.top_left_corner;
            let br_corner = image.bottom_right_corner;
            let w = br_corner.0 - tl_corner.0;
            let h= br_corner.1 - tl_corner.1;
            let alpha = image.alpha;
            let document = web_sys::window().unwrap().document().unwrap();
            let canvas2 = document.create_element("canvas").unwrap().dyn_into::<HtmlCanvasElement>().unwrap();
            canvas2.set_width(width as u32);
            canvas2.set_height(height as u32);
            let context2 = canvas2.get_context("2d").unwrap().unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
            context2.set_global_alpha(alpha);
            context2.draw_image_with_html_image_element_and_dw_and_dh(&img, tl_corner.0, tl_corner.1, w, h).unwrap();
            let pattern = context.create_pattern_with_html_canvas_element(&canvas2, "repeat").unwrap().unwrap();
            context.set_fill_style(&pattern);
            context.fill();
        }
    }
}


pub fn apply_stroke_wasm(
    context: &web_sys::CanvasRenderingContext2d,
    stroke: GradientImageOrColor,
    stroke_width: f64,
    line_cap: &str,
    line_join: &str,
    points: Vec<(f64, f64)>,
    width: u64,
    height: u64
) {
    if points.len() == 0 {
        return;
    }
    if stroke_width == 0.0 {
        return;
    }
    match stroke {
        GradientImageOrColor::Color(color) => {
            let r_string = format!("{}", (color.red * 255.0) as u8);
            let g_string = format!("{}", (color.green * 255.0) as u8);
            let b_string = format!("{}", (color.blue * 255.0) as u8);
            let a_string = format!("{}", color.alpha);
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
                grd.add_color_stop(stop.offset as f32, &color).unwrap();
            }
            context.set_stroke_style(&grd);
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
        },
        GradientImageOrColor::RadialGradient(gradient) => {
            let alpha = gradient.alpha;
            let grd = context.create_radial_gradient(gradient.fx, gradient.fy, 0.0, gradient.cx, gradient.cy, gradient.r).unwrap();
            for stop in gradient.stops.clone() {
                let r_string = format!("{}", (stop.color.red * 255.0) as u8);
                let g_string = format!("{}", (stop.color.green * 255.0) as u8);
                let b_string = format!("{}", (stop.color.blue * 255.0) as u8);
                let a_string = format!("{}", stop.color.alpha * alpha);
                let color = format!("rgba({}, {}, {}, {})", r_string, g_string, b_string, a_string);
                grd.add_color_stop(stop.offset as f32, &color).unwrap();
            }
            context.set_stroke_style(&grd);
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
        },
        GradientImageOrColor::Image(image) => {
            let img = image.image;
            let tl_corner = image.top_left_corner;
            let br_corner = image.bottom_right_corner;
            let w = br_corner.0 - tl_corner.0;
            let h = br_corner.1 - tl_corner.1;
            let alpha = image.alpha;
            let document = web_sys::window().unwrap().document().unwrap();
            let canvas2 = document.create_element("canvas").unwrap().dyn_into::<HtmlCanvasElement>().unwrap();
            canvas2.set_width(width as u32);
            canvas2.set_height(height as u32);
            let context2 = canvas2.get_context("2d").unwrap().unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
            context2.set_global_alpha(alpha);
            context2.draw_image_with_html_image_element_and_dw_and_dh(&img, tl_corner.0, tl_corner.1, w, h).unwrap();
            let pattern = context.create_pattern_with_html_canvas_element(&canvas2, "repeat").unwrap().unwrap();
            context.set_stroke_style(&pattern);
            context.set_line_width(stroke_width);
            context.stroke();
        }
    }
}


pub fn render_vector_wasm(
    vec: &VectorFeatures,
    width: u64,
    height: u64,
    context: web_sys::CanvasRenderingContext2d
) {
    let points = vec.points.clone();
    let fill = vec.fill.clone();
    let stroke = vec.stroke.clone();
    let stroke_width = vec.stroke_width;
    let line_cap = vec.line_cap;
    let line_join = vec.line_join;
    draw_context_path_wasm(&context, points.clone());
    apply_fill_wasm(&context, fill, points.clone(), width, height);
    apply_stroke_wasm(&context, stroke, stroke_width, &line_cap, &line_join, points.clone(), width, height);
    for subvec in &vec.subobjects {
        render_vector_wasm(&subvec, width, height, context.clone());
    }
}


pub fn render_all_vectors(
    vecs: &Vec<VectorFeatures>,
    width: u64,
    height: u64,
    context: Option<web_sys::CanvasRenderingContext2d>,
    background: GradientImageOrColor,
    top_left_corner: (f64, f64),
    bottom_right_corner: (f64, f64)
) -> Option<Vec<u8>> {
    let context = context.unwrap();
    context.reset_transform().unwrap();
    let scale_xy = (width as f64 / (bottom_right_corner.0 - top_left_corner.0), height as f64 / (bottom_right_corner.1 - top_left_corner.1));
    context.scale(scale_xy.0, scale_xy.1).unwrap();
    context.translate(-top_left_corner.0, -top_left_corner.1).unwrap();
    context.clear_rect(top_left_corner.0, top_left_corner.1, bottom_right_corner.0 - top_left_corner.0, bottom_right_corner.1 - top_left_corner.1);
    match background {
        GradientImageOrColor::Color(color) => {
            let fill_style = format!("rgba({}, {}, {}, {})", (color.red * 255.0) as u8, (color.green * 255.0) as u8, (color.blue * 255.0) as u8, color.alpha);
            context.set_fill_style(&fill_style.into());
        }
        GradientImageOrColor::LinearGradient(gradient) => {
            let grd = context.create_linear_gradient(gradient.x1, gradient.y1, gradient.x2, gradient.y2);
            for stop in gradient.stops.clone() {
                let r_string = format!("{}", (stop.color.red * 255.0) as u8);
                let g_string = format!("{}", (stop.color.green * 255.0) as u8);
                let b_string = format!("{}", (stop.color.blue * 255.0) as u8);
                let a_string = format!("{}", stop.color.alpha);
                let color = format!("rgba({}, {}, {}, {})", r_string, g_string, b_string, a_string);
                grd.add_color_stop(stop.offset as f32, &color).unwrap();
            }
            context.set_fill_style(&grd);
        },
        GradientImageOrColor::RadialGradient(gradient) => {
            let grd = context.create_radial_gradient(gradient.fx, gradient.fy, 0.0, gradient.cx, gradient.cy, gradient.r).unwrap();
            for stop in gradient.stops.clone() {
                let r_string = format!("{}", (stop.color.red * 255.0) as u8);
                let g_string = format!("{}", (stop.color.green * 255.0) as u8);
                let b_string = format!("{}", (stop.color.blue * 255.0) as u8);
                let a_string = format!("{}", stop.color.alpha);
                let color = format!("rgba({}, {}, {}, {})", r_string, g_string, b_string, a_string);
                grd.add_color_stop(stop.offset as f32, &color).unwrap();
            }
            context.set_fill_style(&grd);
        },
        GradientImageOrColor::Image(image) => {
            let img = image.image;
            let tl_corner = image.top_left_corner;
            let br_corner = image.bottom_right_corner;
            let w = br_corner.0 - tl_corner.0;
            let h = br_corner.1 - tl_corner.1;
            let alpha = image.alpha;
            let document = web_sys::window().unwrap().document().unwrap();
            let canvas2 = document.create_element("canvas").unwrap().dyn_into::<HtmlCanvasElement>().unwrap();
            canvas2.set_width(w as u32);
            canvas2.set_height(h as u32);
            let context2 = canvas2.get_context("2d").unwrap().unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
            context2.set_global_alpha(alpha);
            context2.draw_image_with_html_image_element_and_dw_and_dh(&img, tl_corner.0, tl_corner.1, w, h).unwrap();
            let pattern = context.create_pattern_with_html_canvas_element(&canvas2, "repeat").unwrap().unwrap();
            context.set_fill_style(&pattern);
        }
    };
    context.fill_rect(top_left_corner.0, top_left_corner.1, bottom_right_corner.0 - top_left_corner.0, bottom_right_corner.1 - top_left_corner.1);
    for vec in vecs {
        render_vector_wasm(&vec, width, height, context.clone());
    }
    return None;
}
