use std::future::Future;
use std::pin::Pin;

use crate::colors::{Color, GradientImageOrColor};
use crate::objects::vector_object::{
    generate_cubic_bezier_tuples, VectorObject
};
use crate::objects::vector_object::generate_subpaths;

use crate::svg_scene::SVGScene;
use crate::utils::consider_points_equals;
use js_sys::{Function, Map, Promise};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, HtmlImageElement};


pub fn draw_context_path_wasm(
    context: &web_sys::CanvasRenderingContext2d,
    points: &Vec<(f64, f64)>
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
    vec: &VectorObject,
    document: &web_sys::Document,
    count: &Vec<usize>
) -> (String, String) {
    let mut def_string = "".to_string();
    let mut use_string = "".to_string();
    let mut id_end = String::new();
    for i in count.iter() {
        id_end.push_str(i.to_string().as_str());
        id_end.push_str("_");
    }
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
                let id = format!("lgradient_{}", id_end);
                grd.set_attribute("id", &id).unwrap();
                grd.set_attribute("x1", &gradient.x1.to_string()).unwrap();
                grd.set_attribute("y1", &gradient.y1.to_string()).unwrap();
                grd.set_attribute("x2", &gradient.x2.to_string()).unwrap();
                grd.set_attribute("y2", &gradient.y2.to_string()).unwrap();
                grd.set_attribute("gradientUnits", "userSpaceOnUse").unwrap();
                for stop in &gradient.stops {
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
                let id = format!("rgradient_{}", id_end);
                grd.set_attribute("id", &id).unwrap();
                grd.set_attribute("cx", &gradient.cx.to_string()).unwrap();
                grd.set_attribute("cy", &gradient.cy.to_string()).unwrap();
                grd.set_attribute("r", &gradient.r.to_string()).unwrap();
                grd.set_attribute("fx", &gradient.fx.to_string()).unwrap();
                grd.set_attribute("fy", &gradient.fy.to_string()).unwrap();
                grd.set_attribute("gradientUnits", "userSpaceOnUse").unwrap();
                for stop in &gradient.stops {
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
                let img = HtmlImageElement::new().unwrap();
                img.set_src(format!("data:{};base64,{}", image.mime_type, image.image_base64).as_str());
                let top_left_corner = image.top_left_corner;
                let bottom_right_corner = image.bottom_right_corner;
                let alpha = image.alpha;
                let x = top_left_corner.0.to_string();
                let y = top_left_corner.1.to_string();
                let width = (bottom_right_corner.0 - top_left_corner.0).to_string();
                let height = (bottom_right_corner.1 - top_left_corner.1).to_string();
                let pattern_id = format!("image_{}", id_end);
                let pattern = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "pattern").unwrap();
                pattern.set_attribute("id", &pattern_id).unwrap();
                pattern.set_attribute("x", &x).unwrap();
                pattern.set_attribute("y", &y).unwrap();
                pattern.set_attribute("width", &width).unwrap();
                pattern.set_attribute("height", &height).unwrap();
                pattern.set_attribute("patternUnits", "userSpaceOnUse").unwrap();
                pattern.set_attribute("viewBox", format!("{} {} {} {}", top_left_corner.0, top_left_corner.1, bottom_right_corner.0 - top_left_corner.0, bottom_right_corner.1 - top_left_corner.1).as_str()).unwrap();
                let img_element = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "image").unwrap();
                img_element.set_attribute("x", &x).unwrap();
                img_element.set_attribute("y", &y).unwrap();
                img_element.set_attribute("width", &width).unwrap();
                img_element.set_attribute("height", &height).unwrap();
                img_element.set_attribute("href", &img.src()).unwrap();
                img_element.set_attribute("opacity", &alpha.to_string()).unwrap();
                img_element.set_attribute("preserveAspectRatio", "none").unwrap();
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
                let id = format!("lgradient_{}_stroke", id_end);
                grd.set_attribute("id", &id).unwrap();
                grd.set_attribute("x1", &gradient.x1.to_string()).unwrap();
                grd.set_attribute("y1", &gradient.y1.to_string()).unwrap();
                grd.set_attribute("x2", &gradient.x2.to_string()).unwrap();
                grd.set_attribute("y2", &gradient.y2.to_string()).unwrap();
                grd.set_attribute("gradientUnits", "userSpaceOnUse").unwrap();
                for stop in &gradient.stops {
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
                let id = format!("rgradient_{}_stroke", id_end);
                grd.set_attribute("id", &id).unwrap();
                grd.set_attribute("cx", &gradient.cx.to_string()).unwrap();
                grd.set_attribute("cy", &gradient.cy.to_string()).unwrap();
                grd.set_attribute("r", &gradient.r.to_string()).unwrap();
                grd.set_attribute("fx", &gradient.fx.to_string()).unwrap();
                grd.set_attribute("fy", &gradient.fy.to_string()).unwrap();
                grd.set_attribute("gradientUnits", "userSpaceOnUse").unwrap();
                for stop in &gradient.stops {
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
                let img = HtmlImageElement::new().unwrap();
                img.set_src(format!("data:{};base64,{}", image.mime_type, image.image_base64).as_str());
                let top_left_corner = image.top_left_corner;
                let bottom_right_corner = image.bottom_right_corner;
                let alpha = image.alpha;
                let x = top_left_corner.0.to_string();
                let y = top_left_corner.1.to_string();
                let width = (bottom_right_corner.0 - top_left_corner.0).to_string();
                let height = (bottom_right_corner.1 - top_left_corner.1).to_string();
                let pattern_id = format!("image_{}_stroke", id_end);
                let pattern = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "pattern").unwrap();
                pattern.set_attribute("id", &pattern_id).unwrap();
                pattern.set_attribute("x", &x).unwrap();
                pattern.set_attribute("y", &y).unwrap();
                pattern.set_attribute("width", &width).unwrap();
                pattern.set_attribute("height", &height).unwrap();
                pattern.set_attribute("patternUnits", "userSpaceOnUse").unwrap();
                pattern.set_attribute("viewBox", format!("{} {} {} {}", top_left_corner.0, top_left_corner.1, bottom_right_corner.0 - top_left_corner.0, bottom_right_corner.1 - top_left_corner.1).as_str()).unwrap();
                let img_element = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "image").unwrap();
                img_element.set_attribute("x", &x).unwrap();
                img_element.set_attribute("y", &y).unwrap();
                img_element.set_attribute("width", &width).unwrap();
                img_element.set_attribute("height", &height).unwrap();
                img_element.set_attribute("href", &img.src()).unwrap();
                img_element.set_attribute("opacity", &alpha.to_string()).unwrap();
                img_element.set_attribute("preserveAspectRatio", "none").unwrap();
                pattern.append_child(&img_element).unwrap();
                def_string.push_str((pattern.outer_html() + "\n").as_str());
                stroke = format!("url(#{})", pattern_id);
            },
        }
        let path_id = format!("path_{}", id_end);
        let d = get_d_string_from_points(&vec.points);
        let path = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path").unwrap();
        path.set_attribute("id", &path_id).unwrap();
        path.set_attribute("d", &d).unwrap();
        path.set_attribute("fill", &fill).unwrap();
        path.set_attribute("stroke", &stroke).unwrap();
        path.set_attribute("stroke-width", &vec.stroke_width.to_string()).unwrap();
        path.set_attribute("stroke-linecap", &vec.line_cap).unwrap();
        path.set_attribute("stroke-linejoin", &vec.line_join).unwrap();
        path.set_attribute("fill-rule", &vec.fill_rule).unwrap();
        use_string.push_str((path.outer_html() + "\n").as_str());
    }
    let mut subcount = count.clone();
    let mut i = 0;
    for subvec in &vec.subobjects {
        subcount.push(i);
        let (subdef_string, subuse_string) = vec_to_def_and_use_string(subvec, document, &subcount);
        def_string.push_str(&subdef_string);
        use_string.push_str(&subuse_string);
        i += 1;
        subcount.pop();
    }
    return (def_string, use_string);
}


pub async fn render_all_vectors_svg(
    svg_scene: &mut SVGScene
) {
    let width = svg_scene.width;
    let height = svg_scene.height;
    let top_left_corner = svg_scene.top_left_corner;
    let bottom_right_corner = svg_scene.bottom_right_corner;
    let div = svg_scene.div_container.as_ref().unwrap();
    let document = web_sys::window().unwrap().document().unwrap();
    div.set_inner_html("");
    let mut svg = format!("<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"{} {} {} {}\">", width, height, top_left_corner.0, top_left_corner.1, bottom_right_corner.0 - top_left_corner.0, bottom_right_corner.1 - top_left_corner.1);
    let mut defs = "<defs>\n".to_string();
    let mut use_strings = "".to_string();
    #[allow(unused_assignments)]
    let mut rec_fill = "".to_string();
    match &svg_scene.background {
        GradientImageOrColor::Color(color) => {
            rec_fill = format!("rgba({}, {}, {}, {})", (color.red * 255.0) as u8, (color.green * 255.0) as u8, (color.blue * 255.0) as u8, color.alpha);
        },
        GradientImageOrColor::LinearGradient(gradient) => {
            let alpha = gradient.alpha;
            let grd = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "linearGradient").unwrap();
            let id = "lgradient_background";
            grd.set_attribute("id", &id).unwrap();
            grd.set_attribute("x1", &gradient.x1.to_string()).unwrap();
            grd.set_attribute("y1", &gradient.y1.to_string()).unwrap();
            grd.set_attribute("x2", &gradient.x2.to_string()).unwrap();
            grd.set_attribute("y2", &gradient.y2.to_string()).unwrap();
            grd.set_attribute("gradientUnits", "userSpaceOnUse").unwrap();
            for stop in &gradient.stops {
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
            let id = "rgradient_background";
            grd.set_attribute("id", &id).unwrap();
            grd.set_attribute("cx", &gradient.cx.to_string()).unwrap();
            grd.set_attribute("cy", &gradient.cy.to_string()).unwrap();
            grd.set_attribute("r", &gradient.r.to_string()).unwrap();
            grd.set_attribute("fx", &gradient.fx.to_string()).unwrap();
            grd.set_attribute("fy", &gradient.fy.to_string()).unwrap();
            grd.set_attribute("gradientUnits", "userSpaceOnUse").unwrap();
            for stop in &gradient.stops {
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
            let img = HtmlImageElement::new().unwrap();
            img.set_src(format!("data:{};base64,{}", image.mime_type, image.image_base64).as_str());
            let top_left_corner = image.top_left_corner;
            let bottom_right_corner = image.bottom_right_corner;
            let alpha = image.alpha;
            let x = top_left_corner.0.to_string();
            let y = top_left_corner.1.to_string();
            let width = (bottom_right_corner.0 - top_left_corner.0).to_string();
            let height = (bottom_right_corner.1 - top_left_corner.1).to_string();
            let pattern_id = "image_background";
            let pattern = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "pattern").unwrap();
            pattern.set_attribute("id", &pattern_id).unwrap();
            pattern.set_attribute("x", &x).unwrap();
            pattern.set_attribute("y", &y).unwrap();
            pattern.set_attribute("width", &width).unwrap();
            pattern.set_attribute("height", &height).unwrap();
            pattern.set_attribute("patternUnits", "userSpaceOnUse").unwrap();
            pattern.set_attribute("viewBox", format!("{} {} {} {}", top_left_corner.0, top_left_corner.1, bottom_right_corner.0 - top_left_corner.0, bottom_right_corner.1 - top_left_corner.1).as_str()).unwrap();
            let img_element = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "image").unwrap();
            img_element.set_attribute("x", &x).unwrap();
            img_element.set_attribute("y", &y).unwrap();
            img_element.set_attribute("width", &width).unwrap();
            img_element.set_attribute("height", &height).unwrap();
            img_element.set_attribute("href", &img.src()).unwrap();
            img_element.set_attribute("opacity", &alpha.to_string()).unwrap();
            img_element.set_attribute("preserveAspectRatio", "none").unwrap();
            img_element.set_attribute("preserveAspectRatio", "none").unwrap();
            pattern.append_child(&img_element).unwrap();
            defs.push_str((pattern.outer_html() + "\n").as_str());
            rec_fill = format!("url(#{})", pattern_id);
        },
    }
    svg.push_str(format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\"/>\n", top_left_corner.0, top_left_corner.1, bottom_right_corner.0 - top_left_corner.0, bottom_right_corner.1 - top_left_corner.1, rec_fill).as_str());
    for (i, vec) in svg_scene.objects.iter().enumerate() {
        let (def_string, use_string) = vec_to_def_and_use_string(&vec, &document, &vec![i]);
        defs.push_str(&def_string);
        let class = svg_scene.classes.get(&vec.index);
        if class.is_some() {
            use_strings.push_str(format!("<g class=\"{}\">{}</g>\n", class.unwrap(), use_string).as_str());
        } else {
            use_strings.push_str(format!("<g>{}</g>\n", use_string).as_str());
        }
    }
    defs.push_str("</defs>\n");
    svg.push_str(&defs);
    svg.push_str(&use_strings);
    svg.push_str("</svg>");
    div.set_inner_html(&svg);
    svg_scene.on_rendered_js().await;
}


pub fn apply_fill_wasm(
    context: &'static web_sys::CanvasRenderingContext2d,
    fill: &GradientImageOrColor,
    fill_rule: &'static str,
    points: &Vec<(f64, f64)>,
    width: u32,
    height: u32,
    loaded_images: &Map
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
            match fill_rule {
                "nonzero" => {
                    context.fill_with_canvas_winding_rule(web_sys::CanvasWindingRule::Nonzero);
                }
                "evenodd" => {
                    context.fill_with_canvas_winding_rule(web_sys::CanvasWindingRule::Evenodd);
                }
                _ => {
                    context.fill_with_canvas_winding_rule(web_sys::CanvasWindingRule::Nonzero);
                }
            }
        },
        GradientImageOrColor::LinearGradient(gradient) => {
            let alpha = gradient.alpha;
            let grd = context.create_linear_gradient(gradient.x1, gradient.y1, gradient.x2, gradient.y2);
            for stop in &gradient.stops {
                let r_string = format!("{}", (stop.color.red * 255.0) as u8);
                let g_string = format!("{}", (stop.color.green * 255.0) as u8);
                let b_string = format!("{}", (stop.color.blue * 255.0) as u8);
                let a_string = format!("{}", stop.color.alpha * alpha);
                let color = format!("rgba({}, {}, {}, {})", r_string, g_string, b_string, a_string);
                grd.add_color_stop(stop.offset as f32, &color).unwrap();
            }
            context.set_fill_style(&grd);
            match fill_rule {
                "nonzero" => {
                    context.fill_with_canvas_winding_rule(web_sys::CanvasWindingRule::Nonzero);
                }
                "evenodd" => {
                    context.fill_with_canvas_winding_rule(web_sys::CanvasWindingRule::Evenodd);
                }
                _ => {
                    context.fill_with_canvas_winding_rule(web_sys::CanvasWindingRule::Nonzero);
                }
            }
        },
        GradientImageOrColor::RadialGradient(gradient) => {
            let alpha = gradient.alpha;
            let grd = context.create_radial_gradient(gradient.fx, gradient.fy, 0.0, gradient.cx, gradient.cy, gradient.r).unwrap();
            for stop in &gradient.stops {
                let r_string = format!("{}", (stop.color.red * 255.0) as u8);
                let g_string = format!("{}", (stop.color.green * 255.0) as u8);
                let b_string = format!("{}", (stop.color.blue * 255.0) as u8);
                let a_string = format!("{}", stop.color.alpha * alpha);
                let color = format!("rgba({}, {}, {}, {})", r_string, g_string, b_string, a_string);
                grd.add_color_stop(stop.offset as f32, &color).unwrap();
            }
            context.set_fill_style(&grd);
            match fill_rule {
                "nonzero" => {
                    context.fill_with_canvas_winding_rule(web_sys::CanvasWindingRule::Nonzero);
                }
                "evenodd" => {
                    context.fill_with_canvas_winding_rule(web_sys::CanvasWindingRule::Evenodd);
                }
                _ => {
                    context.fill_with_canvas_winding_rule(web_sys::CanvasWindingRule::Nonzero);
                }
            }
        },
        GradientImageOrColor::Image(image) => {
            let top_left_corner = image.top_left_corner;
            let bottom_right_corner = image.bottom_right_corner;
            let x = top_left_corner.0;
            let y = top_left_corner.1;
            let w = bottom_right_corner.0 - top_left_corner.0;
            let h = bottom_right_corner.1 - top_left_corner.1;
            let alpha = image.alpha;
            let src = format!("data:{};base64,{}", image.mime_type, image.image_base64);
            let img = loaded_images.get(&JsValue::from_str(src.as_str())).dyn_into::<web_sys::HtmlImageElement>().unwrap();
            let canvas2 = window().unwrap().document().unwrap().create_element("canvas").unwrap().dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
            canvas2.set_width(width);
            canvas2.set_height(height);
            let context2 = canvas2.get_context("2d").unwrap().unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
            context2.set_global_alpha(alpha);
            context2.draw_image_with_html_image_element_and_dw_and_dh(&img, x, y, w, h).unwrap();
            let pattern = context.create_pattern_with_html_canvas_element(&canvas2, "no-repeat").unwrap().unwrap();
            context.set_fill_style(&pattern);
            match fill_rule {
                "nonzero" => {
                    context.fill_with_canvas_winding_rule(web_sys::CanvasWindingRule::Nonzero);
                }
                "evenodd" => {
                    context.fill_with_canvas_winding_rule(web_sys::CanvasWindingRule::Evenodd);
                }
                _ => {
                    context.fill_with_canvas_winding_rule(web_sys::CanvasWindingRule::Nonzero);
                }
            }
        }
    }
}


pub fn apply_stroke_wasm(
    context: &'static web_sys::CanvasRenderingContext2d,
    stroke: &GradientImageOrColor,
    stroke_width: f64,
    line_cap: &'static str,
    line_join: &'static str,
    points: &Vec<(f64, f64)>,
    width: u32,
    height: u32,
    loaded_images: &Map
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
            context.set_line_cap(line_cap);
            context.set_line_join(line_join);
            context.stroke();
        },
        GradientImageOrColor::LinearGradient(gradient) => {
            let alpha = gradient.alpha;
            let grd = context.create_linear_gradient(gradient.x1, gradient.y1, gradient.x2, gradient.y2);
            for stop in &gradient.stops {
                let r_string = format!("{}", (stop.color.red * 255.0) as u8);
                let g_string = format!("{}", (stop.color.green * 255.0) as u8);
                let b_string = format!("{}", (stop.color.blue * 255.0) as u8);
                let a_string = format!("{}", stop.color.alpha * alpha);
                let color = format!("rgba({}, {}, {}, {})", r_string, g_string, b_string, a_string);
                grd.add_color_stop(stop.offset as f32, &color).unwrap();
            }
            if gradient.x1 == gradient.x2 && gradient.y1 == gradient.y2 {
                let last_color = &gradient.stops[gradient.stops.len() - 1].color;
                let r_string = format!("{}", (last_color.red * 255.0) as u8);
                let g_string = format!("{}", (last_color.green * 255.0) as u8);
                let b_string = format!("{}", (last_color.blue * 255.0) as u8);
                let a_string = format!("{}", last_color.alpha * alpha);
                let color = format!("rgba({}, {}, {}, {})", r_string, g_string, b_string, a_string);
                let color = js_sys::JsString::from(color);
                context.set_stroke_style(&color);
            } else {
                context.set_stroke_style(&grd);
            }
            context.set_line_width(stroke_width);
            context.set_line_cap(line_cap);
            context.set_line_join(line_join);
            context.stroke();
        },
        GradientImageOrColor::RadialGradient(gradient) => {
            let alpha = gradient.alpha;
            let grd = context.create_radial_gradient(gradient.fx, gradient.fy, 0.0, gradient.cx, gradient.cy, gradient.r).unwrap();
            for stop in &gradient.stops {
                let r_string = format!("{}", (stop.color.red * 255.0) as u8);
                let g_string = format!("{}", (stop.color.green * 255.0) as u8);
                let b_string = format!("{}", (stop.color.blue * 255.0) as u8);
                let a_string = format!("{}", stop.color.alpha * alpha);
                let color = format!("rgba({}, {}, {}, {})", r_string, g_string, b_string, a_string);
                grd.add_color_stop(stop.offset as f32, &color).unwrap();
            }
            context.set_stroke_style(&grd);
            context.set_line_width(stroke_width);
            context.set_line_cap(line_cap);
            context.set_line_join(line_join);
            context.stroke();
        },
        GradientImageOrColor::Image(image) => {
            let top_left_corner = image.top_left_corner;
            let bottom_right_corner = image.bottom_right_corner;
            let x = top_left_corner.0;
            let y = top_left_corner.1;
            let w = bottom_right_corner.0 - top_left_corner.0;
            let h = bottom_right_corner.1 - top_left_corner.1;
            let alpha = image.alpha;
            let src = format!("data:{};base64,{}", image.mime_type, image.image_base64);
            let img = loaded_images.get(&JsValue::from_str(src.as_str())).dyn_into::<web_sys::HtmlImageElement>().unwrap();
            let canvas2 = window().unwrap().document().unwrap().create_element("canvas").unwrap().dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
            canvas2.set_width(width);
            canvas2.set_height(height);
            let context2 = canvas2.get_context("2d").unwrap().unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
            context2.set_global_alpha(alpha);
            context2.draw_image_with_html_image_element_and_dw_and_dh(&img, x, y, w, h).unwrap();
            let pattern = context.create_pattern_with_html_canvas_element(&canvas2, "no-repeat").unwrap().unwrap();
            context.set_stroke_style(&pattern);
            context.set_line_width(stroke_width);
            context.set_line_cap(line_cap);
            context.set_line_join(line_join);
            context.stroke();
        }
    }
}


pub fn render_vector_wasm(
    vec: &VectorObject,
    width: u32,
    height: u32,
    context: &'static web_sys::CanvasRenderingContext2d,
    loaded_images: &Map
) {
    let stroke_width = vec.stroke_width;
    let line_cap = vec.line_cap;
    let line_join = vec.line_join;
    draw_context_path_wasm(&context, &vec.points);
    apply_fill_wasm(context, &vec.fill, &vec.fill_rule, &vec.points, width, height, loaded_images);
    apply_stroke_wasm(context, &vec.stroke, stroke_width, line_cap, line_join, &vec.points, width, height, loaded_images);
    for subvec in &vec.subobjects {
        render_vector_wasm(subvec, width, height, &context, loaded_images);
    }
}


pub fn load_images<'a>(
    objects: &'a Vec<VectorObject>,
    background: &'a GradientImageOrColor,
    loaded_images: &'a Map
) -> Pin<Box<dyn Future<Output = ()> + 'a>> {
    Box::pin(async move {
        let mut images_to_load = Vec::new();
        match background {
            GradientImageOrColor::Image(image) => {
                images_to_load.push(image);
            },
            _ => {},
        }
        for vec in objects {
            match &vec.fill {
                GradientImageOrColor::Image(image) => {
                    images_to_load.push(image);
                },
                _ => {},
            }
            match &vec.stroke {
                GradientImageOrColor::Image(image) => {
                    images_to_load.push(image);
                },
                _ => {},
            }
            load_images(&vec.subobjects, &GradientImageOrColor::Color(Color { red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0 }), loaded_images).await;
        }
        for image in images_to_load {
            let src = format!("data:{};base64,{}", image.mime_type, image.image_base64);
            if loaded_images.has(&JsValue::from_str(src.as_str())) {
                continue;
            } else if loaded_images.get(&JsValue::from_str(src.as_str())) == JsValue::NULL {
                continue;
            } else {
                let img = HtmlImageElement::new().unwrap();
                loaded_images.set(&JsValue::from_str(src.as_str()), &JsValue::NULL);
                let promise = Promise::new(&mut |resolve, _| {
                    let closure = Closure::wrap(Box::new(move || {
                        resolve.call1(&JsValue::NULL, &JsValue::NULL).unwrap();
                    }) as Box<dyn Fn()>);
                    img.set_onload(Some(&closure.into_js_value().dyn_into().unwrap()));
                    img.set_src(src.as_str());
                });
                JsFuture::from(promise).await.unwrap();
                loaded_images.set(&JsValue::from_str(src.as_str()), &img);
            }
        }
    })
}


pub async fn render_all_vectors(
    vecs: &Vec<VectorObject>,
    width: u32,
    height: u32,
    context: Option<&'static web_sys::CanvasRenderingContext2d>,
    background: &GradientImageOrColor,
    top_left_corner: (f64, f64),
    bottom_right_corner: (f64, f64),
    loaded_images: &Map,
    on_rendered: &Function
) {
    load_images(vecs, background, loaded_images).await;
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
            context.fill_rect(top_left_corner.0, top_left_corner.1, bottom_right_corner.0 - top_left_corner.0, bottom_right_corner.1 - top_left_corner.1);
        }
        GradientImageOrColor::LinearGradient(gradient) => {
            let grd = context.create_linear_gradient(gradient.x1, gradient.y1, gradient.x2, gradient.y2);
            for stop in &gradient.stops {
                let r_string = format!("{}", (stop.color.red * 255.0) as u8);
                let g_string = format!("{}", (stop.color.green * 255.0) as u8);
                let b_string = format!("{}", (stop.color.blue * 255.0) as u8);
                let a_string = format!("{}", stop.color.alpha);
                let color = format!("rgba({}, {}, {}, {})", r_string, g_string, b_string, a_string);
                grd.add_color_stop(stop.offset as f32, &color).unwrap();
            }
            context.set_fill_style(&grd);
            context.fill_rect(top_left_corner.0, top_left_corner.1, bottom_right_corner.0 - top_left_corner.0, bottom_right_corner.1 - top_left_corner.1);
        },
        GradientImageOrColor::RadialGradient(gradient) => {
            let grd = context.create_radial_gradient(gradient.fx, gradient.fy, 0.0, gradient.cx, gradient.cy, gradient.r).unwrap();
            for stop in &gradient.stops {
                let r_string = format!("{}", (stop.color.red * 255.0) as u8);
                let g_string = format!("{}", (stop.color.green * 255.0) as u8);
                let b_string = format!("{}", (stop.color.blue * 255.0) as u8);
                let a_string = format!("{}", stop.color.alpha);
                let color = format!("rgba({}, {}, {}, {})", r_string, g_string, b_string, a_string);
                grd.add_color_stop(stop.offset as f32, &color).unwrap();
            }
            context.set_fill_style(&grd);
            context.fill_rect(top_left_corner.0, top_left_corner.1, bottom_right_corner.0 - top_left_corner.0, bottom_right_corner.1 - top_left_corner.1);
        },
        GradientImageOrColor::Image(image) => {
            let top_left_corner = image.top_left_corner;
            let bottom_right_corner = image.bottom_right_corner;
            let x = top_left_corner.0;
            let y = top_left_corner.1;
            let w = bottom_right_corner.0 - top_left_corner.0;
            let h = bottom_right_corner.1 - top_left_corner.1;
            let alpha = image.alpha;
            let src = format!("data:{};base64,{}", image.mime_type, image.image_base64);
            let img = loaded_images.get(&JsValue::from_str(src.as_str())).dyn_into::<web_sys::HtmlImageElement>().unwrap();
            let canvas2 = window().unwrap().document().unwrap().create_element("canvas").unwrap().dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
            canvas2.set_width(width);
            canvas2.set_height(height);
            let context2 = canvas2.get_context("2d").unwrap().unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
            context2.set_global_alpha(alpha);
            context2.draw_image_with_html_image_element_and_dw_and_dh(&img, x, y, w, h).unwrap();
            let pattern = context.create_pattern_with_html_canvas_element(&canvas2, "no-repeat").unwrap().unwrap();
            context.set_fill_style(&pattern);
            context.fill_rect(top_left_corner.0, top_left_corner.1, bottom_right_corner.0 - top_left_corner.0, bottom_right_corner.1 - top_left_corner.1);
        }
    }
    for vec in vecs {
        render_vector_wasm(&vec, width, height, &context, &loaded_images);
    }
    on_rendered.call0(&JsValue::NULL).unwrap();
}
