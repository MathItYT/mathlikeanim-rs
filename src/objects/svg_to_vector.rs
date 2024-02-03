use core::str;

use lightningcss::traits::Parse;
use lightningcss::values::length::LengthValue;
use lightningcss::values::percentage::{DimensionPercentage, NumberOrPercentage};
use svg::node::element::path::{Command, Data};
use svg::node::element::tag::Type;
use svg::node::Value;
use svg::parser::Event;
use svg::node::element::path::Position;
use lightningcss::{values::color::CssColor, properties::transform::Transform};

use crate::utils::{consider_points_equals, line_as_cubic_bezier, quadratic_bezier_as_cubic_bezier};
use crate::objects::vector_object::{VectorFeatures, VectorObject};


fn parse_color(color: &str) -> CssColor {
    let color = CssColor::parse_string(color);
    return color.unwrap().to_rgb().unwrap();
}


fn parse_path(attributes: &std::collections::HashMap<String, Value>, index: usize) -> VectorFeatures {
    let data = attributes.get("d").unwrap();
    let data = Data::parse(data).unwrap();
    let mut points = Vec::new();
    let mut last_move = None;
    let mut curve_start = None;
    for command in data.iter() {
        match command {
            &Command::Move(ref _abs, ref params) => {
                last_move = Some((params[0] as f64, params[1] as f64));
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
                        x2 += last_move.unwrap().0;
                        y2 += last_move.unwrap().1;
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
            _ => {
                println!("Warning: unsupported command: {:?}", command);
            }
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
    let vec_obj = VectorFeatures {
        points: points,
        fill_color: fill_color,
        stroke_color: stroke_color,
        stroke_width: stroke_width,
        line_cap: line_cap,
        line_join: line_join,
        subobjects: vec![],
        index: index,
    };
    return vec_obj;
}


pub fn svg_to_vector(svg: &str) -> VectorFeatures {
    let mut id_vec_obj_map = std::collections::HashMap::new();
    let mut subobjects = Vec::new();
    let mut subobjects_indices = Vec::new();
    let mut vec_objs_with_no_id = Vec::new();
    let mut current_id = None;
    let mut fill = Vec::new();
    let mut stroke = Vec::new();
    let mut sw = Vec::new();
    let mut lc = Vec::new();
    let mut lj = Vec::new();
    let mut index = 1 as usize;
    let mut shft = Vec::new();
    let mut scl = Vec::new();
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
                }
                let sw_cur = attributes.get("stroke-width").map(|width| {
                    width.parse::<f64>().unwrap()
                });
                if sw_cur.is_some() {
                    sw.push(sw_cur.unwrap());
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
                }
                let transform_cur = attributes.get("transform").map(|transform| {
                    let transform_cur = Transform::parse_string(transform.to_string().replace(", ", ",").replace(" ", ",").as_str()).unwrap();
                    match transform_cur {
                        Transform::Translate(x, y) => {
                            let mut x_num = 0.0;
                            let mut y_num = 0.0;
                            match x {
                                DimensionPercentage::Dimension(x_val) => {
                                    match x_val {
                                        LengthValue::Px(x_numbr) => {
                                            x_num = x_numbr as f64;
                                        }
                                        _ => {
                                            println!("Warning: unsupported length value: {:?}", x_val);
                                        }
                                    }
                                }
                                _ => {
                                    println!("Warning: unsupported length value: {:?}", x);
                                }
                            }
                            match y {
                                DimensionPercentage::Dimension(y_val) => {
                                    match y_val {
                                        LengthValue::Px(y_numbr) => {
                                            y_num = y_numbr as f64;
                                        }
                                        _ => {
                                            println!("Warning: unsupported length value: {:?}", y_val);
                                        }
                                    }
                                }
                                _ => {
                                    println!("Warning: unsupported length value: {:?}", y);
                                }
                            }
                            ((x_num, y_num), "translate")
                        }
                        Transform::Scale(x, y) => {
                            let mut x_num = 1.0;
                            let mut y_num = 1.0;
                            match x {
                                NumberOrPercentage::Number(x_val) => {
                                    x_num = x_val as f64;
                                }
                                _ => {
                                    println!("Warning: unsupported length value: {:?}", x);
                                }
                            }
                            match y {
                                NumberOrPercentage::Number(y_val) => {
                                    y_num = y_val as f64;
                                }
                                _ => {
                                    println!("Warning: unsupported length value: {:?}", y);
                                }
                            }
                            ((x_num, y_num), "scale")
                        }
                        _ => {
                            ((0.0, 0.0), "translate")
                        }
                    }
                });
                if transform_cur.is_some() {
                    if transform_cur.unwrap().1 == "translate" {
                        shft.push(transform_cur.unwrap().0);
                    } else if transform_cur.unwrap().1 == "scale" {
                        scl.push(transform_cur.unwrap().0);
                    }
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
                if shft.len() > 0 {
                    shft.pop();
                }
                if scl.len() > 0 {
                    scl.pop();
                }
            }
            Event::Tag("svg", _, _) => {},
            Event::Tag("symbol", _, attributes) => {
                let id = attributes.get("id").map(|id| {
                    id.to_string()
                });
                current_id = id;
            }
            Event::Tag("path", _, attributes) => {
                let vec_obj = parse_path(&attributes, index);
                if current_id.is_some() {
                    id_vec_obj_map.insert(current_id.clone(), vec_obj.clone());
                } else {
                    let id = attributes.get("id").map(|id| {
                        id.to_string()
                    });
                    if id.is_some() {
                        id_vec_obj_map.insert(id, vec_obj.clone());
                    } else {
                        vec_objs_with_no_id.push(vec_obj.clone());
                    }
                }
                current_id = None;
                index += 1;
            },
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
                let fill_color = fill.last().unwrap_or(&(0.0, 0.0, 0.0, 1.0)).clone();
                let stroke_color = stroke.last().unwrap_or(&(0.0, 0.0, 0.0, 1.0)).clone();
                let stroke_width = sw.last().unwrap_or(&0.0).clone();
                let line_cap = lc.last().unwrap_or(&"butt");
                let line_join = lj.last().unwrap_or(&"miter");
                let shift_vec = shft.last().unwrap_or(&(0.0, 0.0)).clone();
                let scale_vec = scl.last().unwrap_or(&(1.0, 1.0)).clone();
                vec_obj = vec_obj.shift((x, y), false);
                vec_obj = vec_obj.set_fill_color(fill_color, false);
                vec_obj = vec_obj.set_stroke_color(stroke_color, false);
                vec_obj = vec_obj.set_stroke_width(stroke_width, false);
                vec_obj = vec_obj.set_line_cap(line_cap, false);
                vec_obj = vec_obj.set_line_join(line_join, false);
                vec_obj = vec_obj.shift(shift_vec, false);
                vec_obj = vec_obj.stretch(scale_vec, false);
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
        fill_color: (0.0, 0.0, 0.0, 1.0),
        stroke_color: (0.0, 0.0, 0.0, 1.0),
        stroke_width: 0.0,
        line_cap: "butt",
        line_join: "miter",
        subobjects: subobjects,
        index: 0,
    };
}