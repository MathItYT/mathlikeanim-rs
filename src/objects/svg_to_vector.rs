use core::str;

use lightningcss::properties::transform::Transform;
use lightningcss::traits::Parse;
use svg::node::element::path::{Command, Data};
use svg::node::element::tag::Type;
use svg::node::Value;
use svg::parser::Event;
use svg::node::element::path::Position;
use lightningcss::values::color::CssColor;

use crate::objects::geometry::arc::circle;
use crate::objects::geometry::poly::rectangle;
use crate::utils::{consider_points_equals, line_as_cubic_bezier, quadratic_bezier_as_cubic_bezier};
use crate::objects::vector_object::{VectorFeatures, VectorObject};

use crate::colors::{Color, GradientImageOrColor};
use super::geometry::poly::polygon;


fn parse_color(color: &str) -> CssColor {
    let color = CssColor::parse_string(color);
    return color.unwrap().to_rgb().unwrap();
}


fn parse_path(attributes: &std::collections::HashMap<String, Value>, index: usize) -> VectorFeatures {
    let data = attributes.get("d").unwrap();
    let data = Data::parse(data).unwrap();
    let mut points = Vec::new();
    let mut last_move: Option<(f64, f64)> = None;
    let mut curve_start = None;
    for command in data.iter() {
        match command {
            &Command::Move(ref abs, ref params) => {
                let mut x = params[0] as f64;
                let mut y = params[1] as f64;
                match abs {
                    &Position::Relative => {
                        if last_move.is_some() {
                            x += last_move.unwrap().0;
                            y += last_move.unwrap().1;
                        }
                    },
                    _ => {}
                }
                last_move = Some((x, y));
                if curve_start.is_none() {
                    curve_start = last_move;
                }
            },
            &Command::Line(ref abs, ref params) => {
                let mut x = params[0] as f64;
                let mut y = params[1] as f64;
                match abs {
                    &Position::Relative => {
                        x += last_move.unwrap().0;
                        y += last_move.unwrap().1;
                    },
                    _ => {}
                }
                points.extend(line_as_cubic_bezier(last_move.unwrap(), (x, y)));
                last_move = Some((x, y));
            },
            &Command::Close => {
                if !consider_points_equals(last_move.unwrap(), curve_start.unwrap()) {
                    points.extend(line_as_cubic_bezier(last_move.unwrap(), curve_start.unwrap()));
                    last_move = curve_start;
                }
                curve_start = None;
            },
            &Command::CubicCurve(ref abs, ref params) => {
                let mut x1 = params[0] as f64;
                let mut y1 = params[1] as f64;
                let mut x2 = params[2] as f64;
                let mut y2 = params[3] as f64;
                let mut x = params[4] as f64;
                let mut y = params[5] as f64;
                match abs {
                    &Position::Relative => {
                        x1 += last_move.unwrap().0;
                        y1 += last_move.unwrap().1;
                        last_move = Some((x1, y1));
                        x2 += last_move.unwrap().0;
                        y2 += last_move.unwrap().1;
                        last_move = Some((x2, y2));
                        x += last_move.unwrap().0;
                        y += last_move.unwrap().1;
                    },
                    _ => {}
                }
                points.push(last_move.unwrap());
                points.push((x1, y1));
                points.push((x2, y2));
                points.push((x, y));
                last_move = Some((x, y));
            },
            &Command::QuadraticCurve(ref abs, ref params) => {
                let mut x1 = params[0] as f64;
                let mut y1 = params[1] as f64;
                let mut x = params[2] as f64;
                let mut y = params[3] as f64;
                match abs {
                    &Position::Relative => {
                        x1 += last_move.unwrap().0;
                        y1 += last_move.unwrap().1;
                        last_move = Some((x1, y1));
                        x += last_move.unwrap().0;
                        y += last_move.unwrap().1;
                    },
                    _ => {}
                }
                points.extend(quadratic_bezier_as_cubic_bezier(last_move.unwrap(), (x1, y1), (x, y)));
                last_move = Some((x, y));
            },
            &Command::HorizontalLine(ref abs, ref params) => {
                let mut x = params[0] as f64;
                match abs {
                    &Position::Relative => {
                        x += last_move.unwrap().0;
                    },
                    _ => {}
                }
                points.extend(line_as_cubic_bezier(last_move.unwrap(), (x, last_move.unwrap().1)));
                last_move = Some((x, last_move.unwrap().1));
            },
            &Command::VerticalLine(ref abs, ref params) => {
                let mut y = params[0] as f64;
                match abs {
                    &Position::Relative => {
                        y += last_move.unwrap().1;
                    },
                    _ => {}
                }
                points.extend(line_as_cubic_bezier(last_move.unwrap(), (last_move.unwrap().0, y)));
                last_move = Some((last_move.unwrap().0, y));
            },
            &Command::SmoothCubicCurve(ref abs, ref params) => {
                let mut x2 = params[0] as f64;
                let mut y2 = params[1] as f64;
                let mut x = params[2] as f64;
                let mut y = params[3] as f64;
                match abs {
                    &Position::Relative => {
                        x2 += last_move.unwrap().0;
                        y2 += last_move.unwrap().1;
                        last_move = Some((x2, y2));
                        x += last_move.unwrap().0;
                        y += last_move.unwrap().1;
                    },
                    _ => {}
                }
                let last = points.last().unwrap();
                let x1 = 2.0 * last.0 - points[points.len() - 2].0;
                let y1 = 2.0 * last.1 - points[points.len() - 2].1;
                points.push(last_move.unwrap());
                points.push((x1, y1));
                points.push((x2, y2));
                points.push((x, y));
                last_move = Some((x, y));
            },
            _ => {}
        }
    }
    let fill_color = attributes.get("fill").map(|fill| {
        if fill.to_string().as_str() == "none" {
            return (0.0, 0.0, 0.0, 0.0);
        }
        let color = parse_color(fill.to_string().as_str());
        let opacity = attributes.get("fill-opacity").map(|opacity| {
            opacity.parse().unwrap()
        }).unwrap_or(-1.0);
        match color {
            CssColor::RGBA(ref rgba) => {
                (rgba.red_f32() as f64, rgba.green_f32() as f64, rgba.blue_f32() as f64, if opacity == -1.0 { rgba.alpha_f32() as f64 } else { opacity })
            }
            _ => (0.0, 0.0, 0.0, 1.0),
        }
    }).unwrap_or((0.0, 0.0, 0.0, 1.0));
    let stroke_color = attributes.get("stroke").map(|stroke| {
        if stroke.to_string().as_str() == "none" {
            return (0.0, 0.0, 0.0, 0.0);
        }
        let color = parse_color(stroke.to_string().as_str());
        let opacity = attributes.get("stroke-opacity").map(|opacity| {
            opacity.parse().unwrap()
        }).unwrap_or(-1.0);
        match color {
            CssColor::RGBA(ref rgba) => {
                (rgba.red_f32() as f64, rgba.green_f32() as f64, rgba.blue_f32() as f64, if opacity == -1.0 { rgba.alpha_f32() as f64 } else { opacity })
            }
            _ => (0.0, 0.0, 0.0, 1.0),
        }
    }).unwrap_or((0.0, 0.0, 0.0, 1.0));
    let stroke_width = attributes.get("stroke-width").map(|width| {
        width.parse().unwrap()
    }).unwrap_or(0.0);
    let line_cap = attributes.get("stroke-linecap").map(|cap| {
        cap.to_string()
    }).unwrap_or("butt".to_string());
    let line_join = attributes.get("stroke-linejoin").map(|join| {
        join.to_string()
    }).unwrap_or("miter".to_string());
    let line_cap = match line_cap.as_str() {
        "butt" => "butt",
        "square" => "square",
        "round" => "round",
        _ => "butt",
    };
    let line_join = match line_join.as_str() {
        "miter" => "miter",
        "bevel" => "bevel",
        "round" => "round",
        _ => "miter",
    };
    let transform_attr = attributes.get("transform").map(|transform| {
        transform.to_string()
    });
    if transform_attr.is_some() {
        let transform = Transform::parse_string(transform_attr.unwrap().replace(", ", " ").replace(" ", ",").as_str()).unwrap();
        let matrix = transform.to_matrix().unwrap().to_matrix2d().unwrap();
        let mut new_points = Vec::new();
        for point in points {
            let new_x = matrix.a as f64 * point.0 + matrix.c as f64 * point.1 + matrix.e as f64;
            let new_y = matrix.b as f64 * point.0 + matrix.d as f64 * point.1 + matrix.f as f64;
            new_points.push((new_x, new_y));
        }
        points = new_points;
    }
    let vec_obj = VectorFeatures {
        points: points,
        fill: GradientImageOrColor::Color(Color {
            red: fill_color.0,
            green: fill_color.1,
            blue: fill_color.2,
            alpha: fill_color.3,
        }),
        stroke: GradientImageOrColor::Color(Color {
            red: stroke_color.0,
            green: stroke_color.1,
            blue: stroke_color.2,
            alpha: stroke_color.3,
        }),
        stroke_width: stroke_width,
        line_cap: line_cap,
        line_join: line_join,
        subobjects: vec![],
        index: index,
    };
    return vec_obj;
}


fn parse_rect(
    attributes: &std::collections::HashMap<String, Value>,
    index: usize
) -> VectorFeatures {
    let x = attributes.get("x").map(|x| {
        x.parse().unwrap()
    }).unwrap_or(0.0);
    let y = attributes.get("y").map(|y| {
        y.parse().unwrap()
    }).unwrap_or(0.0);
    let width = attributes.get("width").map(|width| {
        width.parse().unwrap()
    }).unwrap_or(0.0);
    let height = attributes.get("height").map(|height| {
        height.parse().unwrap()
    }).unwrap_or(0.0);
    let fill_color = attributes.get("fill").map(|fill| {
        if fill.to_string().as_str() == "none" {
            return (0.0, 0.0, 0.0, 0.0);
        }
        let color = parse_color(fill.to_string().as_str());
        let opacity = attributes.get("fill-opacity").map(|opacity| {
            opacity.parse().unwrap()
        }).unwrap_or(-1.0);
        match color {
            CssColor::RGBA(ref rgba) => {
                (rgba.red_f32() as f64, rgba.green_f32() as f64, rgba.blue_f32() as f64, if opacity == -1.0 { rgba.alpha_f32() as f64 } else { opacity })
            }
            _ => (0.0, 0.0, 0.0, 1.0),
        }
    }).unwrap_or((0.0, 0.0, 0.0, 1.0));
    let stroke_color = attributes.get("stroke").map(|stroke| {
        if stroke.to_string().as_str() == "none" {
            return (0.0, 0.0, 0.0, 0.0);
        }
        let color = parse_color(stroke.to_string().as_str());
        let opacity = attributes.get("stroke-opacity").map(|opacity| {
            opacity.parse().unwrap()
        }).unwrap_or(-1.0);
        match color {
            CssColor::RGBA(ref rgba) => {
                (rgba.red_f32() as f64, rgba.green_f32() as f64, rgba.blue_f32() as f64, if opacity == -1.0 { rgba.alpha_f32() as f64 } else { opacity })
            }
            _ => (0.0, 0.0, 0.0, 1.0),
        }
    }).unwrap_or((0.0, 0.0, 0.0, 1.0));
    let stroke_width = attributes.get("stroke-width").map(|width| {
        width.parse().unwrap()
    }).unwrap_or(0.0);
    let line_cap = attributes.get("stroke-linecap").map(|cap| {
        cap.to_string()
    }).unwrap_or("butt".to_string());
    let line_join = attributes.get("stroke-linejoin").map(|join| {
        join.to_string()
    }).unwrap_or("miter".to_string());
    let line_cap = match line_cap.as_str() {
        "butt" => "butt",
        "square" => "square",
        "round" => "round",
        _ => "butt",
    };
    let line_join = match line_join.as_str() {
        "miter" => "miter",
        "bevel" => "bevel",
        "round" => "round",
        _ => "miter",
    };
    let center = (x + width / 2.0, y + height / 2.0);
    let vec_obj = rectangle(
        center,
        width,
        height,
        Some(stroke_color),
        Some(fill_color),
        Some(stroke_width),
        Some(line_cap),
        Some(line_join),
        Some(index)
    );
    return vec_obj;
}


fn parse_circle(
    attributes: &std::collections::HashMap<String, Value>,
    index: usize
) -> VectorFeatures {
    let cx = attributes.get("cx").map(|cx| {
        cx.parse().unwrap()
    }).unwrap_or(0.0);
    let cy = attributes.get("cy").map(|cy| {
        cy.parse().unwrap()
    }).unwrap_or(0.0);
    let r = attributes.get("r").map(|r| {
        r.parse().unwrap()
    }).unwrap_or(0.0);
    let fill_color = attributes.get("fill").map(|fill| {
        if fill.to_string().as_str() == "none" {
            return (0.0, 0.0, 0.0, 0.0);
        }
        let color = parse_color(fill.to_string().as_str());
        let opacity = attributes.get("fill-opacity").map(|opacity| {
            opacity.parse().unwrap()
        }).unwrap_or(-1.0);
        match color {
            CssColor::RGBA(ref rgba) => {
                (rgba.red_f32() as f64, rgba.green_f32() as f64, rgba.blue_f32() as f64, if opacity == -1.0 { rgba.alpha_f32() as f64 } else { opacity })
            }
            _ => (0.0, 0.0, 0.0, 1.0),
        }
    }).unwrap_or((0.0, 0.0, 0.0, 1.0));
    let stroke_color = attributes.get("stroke").map(|stroke| {
        if stroke.to_string().as_str() == "none" {
            return (0.0, 0.0, 0.0, 0.0);
        }
        let color = parse_color(stroke.to_string().as_str());
        let opacity = attributes.get("stroke-opacity").map(|opacity| {
            opacity.parse().unwrap()
        }).unwrap_or(-1.0);
        match color {
            CssColor::RGBA(ref rgba) => {
                (rgba.red_f32() as f64, rgba.green_f32() as f64, rgba.blue_f32() as f64, if opacity == -1.0 { rgba.alpha_f32() as f64 } else { opacity })
            }
            _ => (0.0, 0.0, 0.0, 1.0),
        }
    }).unwrap_or((0.0, 0.0, 0.0, 1.0));
    let stroke_width = attributes.get("stroke-width").map(|width| {
        width.parse().unwrap()
    }).unwrap_or(0.0);
    let line_cap = attributes.get("stroke-linecap").map(|cap| {
        cap.to_string()
    }).unwrap_or("butt".to_string());
    let line_join = attributes.get("stroke-linejoin").map(|join| {
        join.to_string()
    }).unwrap_or("miter".to_string());
    let line_cap = match line_cap.as_str() {
        "butt" => "butt",
        "square" => "square",
        "round" => "round",
        _ => "butt",
    };
    let line_join = match line_join.as_str() {
        "miter" => "miter",
        "bevel" => "bevel",
        "round" => "round",
        _ => "miter",
    };
    let vec_obj = circle(
        (cx, cy),
        r,
        None,
        Some(stroke_color),
        Some(fill_color),
        Some(stroke_width),
        Some(line_cap),
        Some(line_join),
        Some(index)
    );
    return vec_obj;
}


fn parse_polygon(
    attributes: &std::collections::HashMap<String, Value>,
    index: usize
) -> VectorFeatures {
    let points = attributes.get("points").map(|points| {
        let points = points.to_string();
        let points = points.split(" ");
        let points = points.map(|point| {
            let point = point.split(",");
            let mut point = point.map(|coord| {
                coord.parse().unwrap()
            });
            (point.next().unwrap(), point.next().unwrap())
        });
        let mut points = points.collect::<Vec<(f64, f64)>>();
        points.push(points[0]);
        points
    }).unwrap_or(vec![]);
    let fill_color = attributes.get("fill").map(|fill| {
        if fill.to_string().as_str() == "none" {
            return (0.0, 0.0, 0.0, 0.0);
        }
        let color = parse_color(fill.to_string().as_str());
        let opacity = attributes.get("fill-opacity").map(|opacity| {
            opacity.parse().unwrap()
        }).unwrap_or(-1.0);
        match color {
            CssColor::RGBA(ref rgba) => {
                (rgba.red_f32() as f64, rgba.green_f32() as f64, rgba.blue_f32() as f64, if opacity == -1.0 { rgba.alpha_f32() as f64 } else { opacity })
            }
            _ => (0.0, 0.0, 0.0, 1.0),
        }
    }).unwrap_or((0.0, 0.0, 0.0, 1.0));
    let stroke_color = attributes.get("stroke").map(|stroke| {
        if stroke.to_string().as_str() == "none" {
            return (0.0, 0.0, 0.0, 0.0);
        }
        let color = parse_color(stroke.to_string().as_str());
        let opacity = attributes.get("stroke-opacity").map(|opacity| {
            opacity.parse().unwrap()
        }).unwrap_or(-1.0);
        match color {
            CssColor::RGBA(ref rgba) => {
                (rgba.red_f32() as f64, rgba.green_f32() as f64, rgba.blue_f32() as f64, if opacity == -1.0 { rgba.alpha_f32() as f64 } else { opacity })
            }
            _ => (0.0, 0.0, 0.0, 1.0),
        }
    }).unwrap_or((0.0, 0.0, 0.0, 1.0));
    let stroke_width = attributes.get("stroke-width").map(|width| {
        width.parse().unwrap()
    }).unwrap_or(0.0);
    let line_cap = attributes.get("stroke-linecap").map(|cap| {
        cap.to_string()
    }).unwrap_or("butt".to_string());
    let line_join = attributes.get("stroke-linejoin").map(|join| {
        join.to_string()
    }).unwrap_or("miter".to_string());
    let line_cap = match line_cap.as_str() {
        "butt" => "butt",
        "square" => "square",
        "round" => "round",
        _ => "butt",
    };
    let line_join = match line_join.as_str() {
        "miter" => "miter",
        "bevel" => "bevel",
        "round" => "round",
        _ => "miter",
    };
    let polygon = polygon(
        points,
        Some(stroke_color),
        Some(fill_color),
        Some(stroke_width),
        Some(line_cap),
        Some(line_join),
        Some(index)
    );
    return polygon;
}


pub fn svg_to_vector(svg: &str) -> VectorFeatures {
    let mut id_vec_obj_map = std::collections::HashMap::new();
    let mut subobjects = Vec::new();
    let mut subobjects_indices = Vec::new();
    let mut vec_objs_with_no_id = Vec::new();
    let mut fill = Vec::new();
    let mut stroke = Vec::new();
    let mut sw = Vec::new();
    let mut lc = Vec::new();
    let mut lj = Vec::new();
    let mut index = 1 as usize;
    for event in svg::read(svg).unwrap() {
        match event {
            Event::Tag("defs", _, _) => {},
            Event::Tag("g", Type::Start, attributes) => {
                let fill_cur = attributes.get("fill").map(|fill| {
                    if fill.to_string().as_str() == "none" {
                        return (0.0, 0.0, 0.0, 0.0);
                    }
                    let color = parse_color(fill.to_string().as_str());
                    let opacity = attributes.get("fill-opacity").map(|opacity| {
                        opacity.parse().unwrap()
                    }).unwrap_or(-1.0);
                    match color {
                        CssColor::RGBA(ref rgba) => {
                            (rgba.red_f32() as f64, rgba.green_f32() as f64, rgba.blue_f32() as f64, if opacity == -1.0 { rgba.alpha_f32() as f64 } else { opacity })
                        }
                        _ => (0.0, 0.0, 0.0, 1.0),
                    }
                });
                if fill_cur.is_some() {
                    fill.push(fill_cur.unwrap());
                } else if fill.len() > 0 {
                    fill.push(fill.last().unwrap().clone());
                }
                let stroke_cur = attributes.get("stroke").map(|stroke| {
                    if stroke.to_string().as_str() == "none" {
                        return (0.0, 0.0, 0.0, 0.0);
                    }
                    let color = parse_color(stroke.to_string().as_str());
                    let opacity = attributes.get("stroke-opacity").map(|opacity| {
                        opacity.parse().unwrap()
                    }).unwrap_or(-1.0);
                    match color {
                        CssColor::RGBA(ref rgba) => {
                            (rgba.red_f32() as f64, rgba.green_f32() as f64, rgba.blue_f32() as f64, if opacity == -1.0 { rgba.alpha_f32() as f64 } else { opacity })
                        }
                        _ => (0.0, 0.0, 0.0, 1.0),
                    }
                });
                if stroke_cur.is_some() {
                    stroke.push(stroke_cur.unwrap());
                } else if stroke.len() > 0 {
                    stroke.push(stroke.last().unwrap().clone());
                }
                let sw_cur = attributes.get("stroke-width").map(|width| {
                    width.parse::<f64>().unwrap()
                });
                if sw_cur.is_some() {
                    sw.push(sw_cur.unwrap());
                } else if sw.len() > 0 {
                    sw.push(sw.last().unwrap().clone());
                }
                let lc_cur = attributes.get("stroke-linecap").map(|cap| {
                    let cap = cap.to_string();
                    match cap.as_str() {
                        "butt" => "butt",
                        "square" => "square",
                        "round" => "round",
                        _ => "butt",
                    }
                });
                if lc_cur.is_some() {
                    lc.push(lc_cur.unwrap());
                } else if lc.len() > 0 {
                    lc.push(lc.last().unwrap());
                }
                let lj_cur = attributes.get("stroke-linejoin").map(|join| {
                    let join = join.to_string();
                    match join.as_str() {
                        "miter" => "miter",
                        "bevel" => "bevel",
                        "round" => "round",
                        _ => "miter",
                    }
                });
                if lj_cur.is_some() {
                    lj.push(lj_cur.unwrap());
                } else if lj.len() > 0 {
                    lj.push(lj.last().unwrap());
                }
            }
            Event::Tag("g", Type::End, _) => {
                if fill.len() > 0 {
                    fill.pop();
                }
                if stroke.len() > 0 {
                    stroke.pop();
                }
                if sw.len() > 0 {
                    sw.pop();
                }
                if lc.len() > 0 {
                    lc.pop();
                }
                if lj.len() > 0 {
                    lj.pop();
                }
            }
            Event::Tag("svg", _, _) => {},
            Event::Tag("path", _, attributes) => {
                let vec_obj = parse_path(&attributes, index);
                let id = attributes.get("id").map(|id| {
                    id.to_string()
                });
                if id.is_some() {
                    id_vec_obj_map.insert(id, vec_obj.clone());
                } else {
                    vec_objs_with_no_id.push(vec_obj.clone());
                }
                index += 1;
            },
            Event::Tag("rect", _, attributes) => {
                let vec_obj = parse_rect(&attributes, index);
                let id = attributes.get("id").map(|id| {
                    id.to_string()
                });
                if id.is_some() {
                    id_vec_obj_map.insert(id, vec_obj.clone());
                } else {
                    vec_objs_with_no_id.push(vec_obj.clone());
                }
                index += 1;
            }
            Event::Tag("circle", _, attributes) => {
                let vec_obj = parse_circle(&attributes, index);
                let id = attributes.get("id").map(|id| {
                    id.to_string()
                });
                if id.is_some() {
                    id_vec_obj_map.insert(id, vec_obj.clone());
                } else {
                    vec_objs_with_no_id.push(vec_obj.clone());
                }
                index += 1;
            }
            Event::Tag("polygon", _, attributes) => {
                let vec_obj = parse_polygon(&attributes, index);
                let id = attributes.get("id").map(|id| {
                    id.to_string()
                });
                if id.is_some() {
                    id_vec_obj_map.insert(id, vec_obj.clone());
                } else {
                    vec_objs_with_no_id.push(vec_obj.clone());
                }
                index += 1;
            }
            Event::Tag("use", _, attributes) => {
                let x_link_href = attributes.get("xlink:href").map(
                    |xlink_href| {
                        xlink_href[1..].to_string()
                    }
                );
                if x_link_href.is_none() {
                    continue;
                }
                let x = attributes.get("x").map(|x| {
                    x.parse().unwrap()
                }).unwrap_or(0.0);
                let y = attributes.get("y").map(|y| {
                    y.parse().unwrap()
                }).unwrap_or(0.0);
                let vec_obj = id_vec_obj_map.get(&x_link_href);
                if vec_obj.is_none() {
                    println!("Warning: no object with id: {:?}", x_link_href);
                    continue;
                }
                let mut vec_obj = vec_obj.unwrap().clone();
                let fill_color = fill.last();
                let stroke_color = stroke.last();
                let stroke_width = sw.last().unwrap_or(&vec_obj.stroke_width).clone();
                #[allow(suspicious_double_ref_op)]
                let line_cap = lc.last().unwrap_or(&vec_obj.line_cap).clone();
                #[allow(suspicious_double_ref_op)]
                let line_join = lj.last().unwrap_or(&vec_obj.line_join).clone();
                vec_obj = vec_obj.shift((x, y), false);
                if fill_color.is_some() {
                    vec_obj = vec_obj.set_fill(GradientImageOrColor::Color(Color {
                        red: fill_color.unwrap().0,
                        green: fill_color.unwrap().1,
                        blue: fill_color.unwrap().2,
                        alpha: fill_color.unwrap().3,
                    }), false);
                }
                if stroke_color.is_some() {
                    vec_obj = vec_obj.set_stroke(GradientImageOrColor::Color(Color {
                        red: stroke_color.unwrap().0,
                        green: stroke_color.unwrap().1,
                        blue: stroke_color.unwrap().2,
                        alpha: stroke_color.unwrap().3,
                    }), false);
                }
                vec_obj = vec_obj.set_stroke_width(stroke_width, false);
                vec_obj = vec_obj.set_line_cap(line_cap, false);
                vec_obj = vec_obj.set_line_join(line_join, false);
                if subobjects_indices.contains(&vec_obj.index) {
                    let mut i = vec_obj.index;
                    while subobjects_indices.contains(&i) || i == 0 {
                        i += 1;
                    }
                    vec_obj.index = i;
                }
                subobjects_indices.push(vec_obj.index);
                subobjects.push(vec_obj);
            }
            Event::Tag(tag, _, _) => {
                println!("Warning: unsupported tag: {:?}", tag);
            },
            Event::Text(..) => {},
            Event::Comment(..) => {},
            Event::Instruction(..) => {},
            Event::Declaration(..) => {},
            Event::Error(..) => {
                panic!("Error while parsing SVG");
            },
        }
    }
    if vec_objs_with_no_id.len() > 0 {
        for vec_obj in vec_objs_with_no_id {
            subobjects.push(vec_obj);
        }
    }
    return VectorFeatures {
        points: vec![],
        fill: GradientImageOrColor::Color(Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 0.0,
        }),
        stroke: GradientImageOrColor::Color(Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 0.0,
        }),
        stroke_width: 0.0,
        line_cap: "butt",
        line_join: "miter",
        subobjects: subobjects,
        index: 0,
    };
}