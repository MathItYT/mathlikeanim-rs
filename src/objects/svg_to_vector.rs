use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

use js_sys::Map;
use lightningcss::properties::transform::{Matrix, Transform};
use lightningcss::traits::Parse;
use svg::node::element::path::{Command, Data};
use svg::node::element::tag::Type;
use svg::node::Value;
use svg::parser::Event;
use svg::node::element::path::Position;
use lightningcss::values::color::CssColor;

use crate::objects::geometry::arc::circle;
use crate::objects::geometry::poly::rectangle;
use crate::utils::{consider_points_equals, elliptical_arc_path, line_as_cubic_bezier, quadratic_bezier_as_cubic_bezier};
use crate::utils::log;
use crate::objects::vector_object::VectorObject;
use crate::objects::text_to_vector::text_to_vector;

use crate::colors::{Color, GradientImageOrColor};
use super::geometry::arc::ellipse;
use super::geometry::poly::polygon;


fn parse_color(color: &str) -> CssColor {
    let color = CssColor::parse_string(color);
    return color.unwrap().to_rgb().unwrap();
}


pub fn parse_transform(transform_attr: &str) -> Matrix<f32> {
    let first_parenthesis = transform_attr.find("(").unwrap();
    let last_parenthesis = transform_attr.rfind(")").unwrap();
    let action = transform_attr[..first_parenthesis].replace(" ", "");
    let numbers = transform_attr[first_parenthesis + 1..last_parenthesis].replace(", ", " ").replace(" ", ", ");
    let transform_attr = format!("{}({})", action, numbers);
    log(&transform_attr);
    let transform = Transform::parse_string(transform_attr.as_str()).unwrap();
    let matrix = transform.to_matrix().unwrap().to_matrix2d().unwrap();
    return matrix;
}


pub fn parse_style_attr(style: &str) -> HashMap<String, String> {
    let mut style_map = HashMap::new();
    let style = style.split(";");
    for style in style {
        let style = style.split(":");
        let mut style = style.map(|style| {
            style.trim().to_string()
        });
        let key = style.next().unwrap();
        let value = style.next().unwrap();
        style_map.insert(key, value);
    }
    return style_map;
}


pub fn parse_ellipse(attributes: &std::collections::HashMap<String, Value>, index: usize, fill: &(f64, f64, f64, f64), stroke: &(f64, f64, f64, f64), sw: &f64, lc: &&str, lj: &&str, transforms: &Vec<Vec<Matrix<f32>>>) -> VectorObject {
    let mut transforms = transforms.clone();
    let style = attributes.get("style").map(|style| {
        parse_style_attr(style.to_string().as_str())
    }).unwrap_or(HashMap::new());
    let cx = attributes.get("cx").map(|cx| {
        cx.parse().unwrap()
    }).unwrap_or(style.get("cx").map(|cx| {
        cx.parse().unwrap()
    }).unwrap_or(0.0));
    let cy = attributes.get("cy").map(|cy| {
        cy.parse().unwrap()
    }).unwrap_or(style.get("cy").map(|cy| {
        cy.parse().unwrap()
    }).unwrap_or(0.0));
    let rx = attributes.get("rx").map(|rx| {
        rx.parse().unwrap()
    }).unwrap_or(style.get("rx").map(|rx| {
        rx.parse().unwrap()
    }).unwrap_or(0.0));
    let ry = attributes.get("ry").map(|ry| {
        ry.parse().unwrap()
    }).unwrap_or(style.get("ry").map(|ry| {
        ry.parse().unwrap()
    }).unwrap_or(0.0));
    let fill_color = attributes.get("fill").map(|fill| {
        if fill.to_string().as_str() == "none" {
            return (0.0, 0.0, 0.0, 0.0);
        }
        if fill.to_string().as_str() == "currentColor" {
            return (0.0, 0.0, 0.0, 1.0);
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
    }).unwrap_or(style.get("fill").map(|fill| {
        if fill.as_str() == "none" {
            return (0.0, 0.0, 0.0, 0.0);
        }
        if fill.as_str() == "currentColor" {
            return (0.0, 0.0, 0.0, 1.0);
        }
        let color = parse_color(fill.as_str());
        let opacity = style.get("fill-opacity").map(|opacity| {
            opacity.parse().unwrap()
        }).unwrap_or(-1.0);
        match color {
            CssColor::RGBA(ref rgba) => {
                (rgba.red_f32() as f64, rgba.green_f32() as f64, rgba.blue_f32() as f64, if opacity == -1.0 { rgba.alpha_f32() as f64 } else { opacity })
            }
            _ => (0.0, 0.0, 0.0, 1.0),
        }
    }).unwrap_or(fill.clone()));
    let stroke_color = attributes.get("stroke").map(|stroke| {
        if stroke.to_string().as_str() == "none" {
            return (0.0, 0.0, 0.0, 0.0);
        }
        if stroke.to_string().as_str() == "currentColor" {
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
    }).unwrap_or(style.get("stroke").map(|stroke| {
        if stroke.as_str() == "none" {
            return (0.0, 0.0, 0.0, 0.0);
        }
        if stroke.as_str() == "currentColor" {
            return (0.0, 0.0, 0.0, 0.0);
        }
        let color = parse_color(stroke.as_str());
        let opacity = style.get("stroke-opacity").map(|opacity| {
            opacity.parse().unwrap()
        }).unwrap_or(-1.0);
        match color {
            CssColor::RGBA(ref rgba) => {
                (rgba.red_f32() as f64, rgba.green_f32() as f64, rgba.blue_f32() as f64, if opacity == -1.0 { rgba.alpha_f32() as f64 } else { opacity })
            }
            _ => (0.0, 0.0, 0.0, 1.0),
        }
    }).unwrap_or(stroke.clone()));
    let stroke_width = attributes.get("stroke-width").map(|width| {
        width.parse().unwrap()
    }).unwrap_or(style.get("stroke-width").map(|width| {
        width.parse().unwrap()
    }).unwrap_or(*sw));
    let line_cap = attributes.get("stroke-linecap").map(|cap| {
        cap.to_string()
    }).unwrap_or(style.get("stroke-linecap").map(|cap| {
        cap.to_string()
    }).unwrap_or(lc.to_string()));
    let line_join = attributes.get("stroke-linejoin").map(|join| {
        join.to_string()
    }).unwrap_or(style.get("stroke-linejoin").map(|join| {
        join.to_string()
    }).unwrap_or(lj.to_string()));
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
    let mut vec_obj = ellipse(
        (cx, cy),
        rx,
        ry,
        None,
        Some(stroke_color),
        Some(fill_color),
        Some(stroke_width),
        Some(line_cap),
        Some(line_join),
        Some(index)
    );
    let mut points = vec_obj.points.clone();
    let mut transform = attributes.get("transform").map(|transform| {
        transform.to_string()
    });
    if transform.is_none() {
        transform = style.get("transform").cloned();
    }
    if transform.is_some() {
        transforms.push(vec![parse_transform(transform.unwrap().as_str())]);
    }
    for transform in transforms.iter().rev() {
        for matrix in transform.iter().rev() {
            let new_points = points.iter().map(|point| {
                let new_x = matrix.a as f64 * point.0 + matrix.c as f64 * point.1 + matrix.e as f64;
                let new_y = matrix.b as f64 * point.0 + matrix.d as f64 * point.1 + matrix.f as f64;
                (new_x, new_y)
            }).collect::<Vec<(f64, f64)>>();
            points = new_points;
        }
    }
    vec_obj.points = points;
    return vec_obj;
}


pub fn parse_path(attributes: &std::collections::HashMap<String, Value>, index: usize, fill: &(f64, f64, f64, f64), stroke: &(f64, f64, f64, f64), sw: &f64, lc: &&str, lj: &&str, transforms: &Vec<Vec<Matrix<f32>>>) -> VectorObject {
    let mut transforms = transforms.clone();
    let style = attributes.get("style").map(|style| {
        parse_style_attr(style.to_string().as_str())
    }).unwrap_or(HashMap::new());
    let data = attributes.get("d").map(|d| {
        d.to_string()
    }).unwrap_or(style.get("d").map(|d| {
        d.to_string()
    }).unwrap_or("".to_string()));
    let data = Data::parse(data.as_str()).unwrap();
    let mut points = Vec::new();
    let mut last_move: Option<(f64, f64)> = None;
    let mut curve_start = None;
    let mut last_quadratic_curve = None;
    let mut last_cubic_curve = None;
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
                last_quadratic_curve = None;
                last_cubic_curve = None;
            },
            &Command::Close => {
                if !consider_points_equals(last_move.unwrap(), curve_start.unwrap()) {
                    points.extend(line_as_cubic_bezier(last_move.unwrap(), curve_start.unwrap()));
                    last_move = curve_start;
                }
                curve_start = None;
                last_quadratic_curve = None;
                last_cubic_curve = None;
            },
            &Command::CubicCurve(ref abs, ref params) => {
                let mut x1 = params[0] as f64;
                let mut y1 = params[1] as f64;
                let mut x2 = params[2] as f64;
                let mut y2 = params[3] as f64;
                let mut x = params[4] as f64;
                let mut y = params[5] as f64;
                let last = last_move.unwrap();
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
                points.push(last);
                points.push((x1, y1));
                points.push((x2, y2));
                points.push((x, y));
                last_move = Some((x, y));
                last_quadratic_curve = None;
                last_cubic_curve = Some((x2, y2));
            },
            &Command::QuadraticCurve(ref abs, ref params) => {
                let mut x1 = params[0] as f64;
                let mut y1 = params[1] as f64;
                let mut x = params[2] as f64;
                let mut y = params[3] as f64;
                let last = last_move.unwrap();
                match abs {
                    &Position::Relative => {
                        x1 += last_move.unwrap().0;
                        y1 += last_move.unwrap().1;
                        x += last_move.unwrap().0;
                        y += last_move.unwrap().1;
                    },
                    _ => {}
                }
                points.extend(quadratic_bezier_as_cubic_bezier(last, (x1, y1), (x, y)));
                last_move = Some((x, y));
                last_quadratic_curve = Some((x1, y1));
                last_cubic_curve = None;
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
                last_quadratic_curve = None;
                last_cubic_curve = None;
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
                last_quadratic_curve = None;
                last_cubic_curve = None;
            },
            &Command::SmoothCubicCurve(ref abs, ref params) => {
                let mut x2 = params[0] as f64;
                let mut y2 = params[1] as f64;
                let mut x = params[2] as f64;
                let mut y = params[3] as f64;
                let last = last_move.unwrap();
                match abs {
                    &Position::Relative => {
                        x2 += last_move.unwrap().0;
                        y2 += last_move.unwrap().1;
                        x += last_move.unwrap().0;
                        y += last_move.unwrap().1;
                    },
                    _ => {}
                }
                let x1 = if last_cubic_curve.is_some() {
                    2.0 * last.0 - last_cubic_curve.unwrap().0
                } else {
                    last.0
                };
                let y1 = if last_cubic_curve.is_some() {
                    2.0 * last.1 - last_cubic_curve.unwrap().1
                } else {
                    last.1
                };
                points.push(last);
                points.push((x1, y1));
                points.push((x2, y2));
                points.push((x, y));
                last_move = Some((x, y));
                last_cubic_curve = Some((x2, y2));
                last_quadratic_curve = None;
            },
            &Command::SmoothQuadraticCurve(ref abs, ref params) => {
                let last = last_move.unwrap();
                let x1 = if last_quadratic_curve.is_some() {
                    2.0 * last.0 - last_quadratic_curve.unwrap().0
                } else {
                    last.0
                };
                let y1 = if last_quadratic_curve.is_some() {
                    2.0 * last.1 - last_quadratic_curve.unwrap().1
                } else {
                    last.1
                };
                let mut x = params[0] as f64;
                let mut y = params[1] as f64;
                match abs {
                    &Position::Relative => {
                        x += last_move.unwrap().0;
                        y += last_move.unwrap().1;
                    },
                    _ => {}
                }
                let cubic = quadratic_bezier_as_cubic_bezier(last, (x1, y1), (x, y));
                points.extend(cubic);
                last_move = Some((x, y));
                last_cubic_curve = None;
                last_quadratic_curve = Some((x1, y1));
            },
            &Command::EllipticalArc(ref abs, ref params) => {
                let rx = params[0] as f64;
                let ry = params[1] as f64;
                let rotation = params[2] as f64;
                let large_arc = params[3] == 1.0;
                let sweep = params[4] == 1.0;
                let mut x = params[5] as f64;
                let mut y = params[6] as f64;
                let last = last_move.unwrap();
                match abs {
                    &Position::Relative => {
                        x += last.0;
                        y += last.1;
                    },
                    _ => {}
                }
                let arc = elliptical_arc_path(last, rx, ry, rotation, large_arc, sweep, x, y);
                points.extend(arc);
                last_move = Some((x, y));
                last_cubic_curve = None;
                last_quadratic_curve = None;
            },
        }
    }
    let fill_color = attributes.get("fill").map(|fill| {
        if fill.to_string().as_str() == "none" {
            return (0.0, 0.0, 0.0, 0.0);
        }
        if fill.to_string().as_str() == "currentColor" {
            return (0.0, 0.0, 0.0, 1.0);
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
    }).unwrap_or(style.get("fill").map(|fill| {
        if fill.as_str() == "none" {
            return (0.0, 0.0, 0.0, 0.0);
        }
        if fill.as_str() == "currentColor" {
            return (0.0, 0.0, 0.0, 1.0);
        }
        let color = parse_color(fill.as_str());
        let opacity = style.get("fill-opacity").map(|opacity| {
            opacity.parse().unwrap()
        }).unwrap_or(-1.0);
        match color {
            CssColor::RGBA(ref rgba) => {
                (rgba.red_f32() as f64, rgba.green_f32() as f64, rgba.blue_f32() as f64, if opacity == -1.0 { rgba.alpha_f32() as f64 } else { opacity })
            }
            _ => (0.0, 0.0, 0.0, 1.0),
        }
    }).unwrap_or(fill.clone()));
    let fill_rule = attributes.get("fill-rule").map(|rule| {
        rule.to_string()
    }).unwrap_or("nonzero".to_string());
    let stroke_color = attributes.get("stroke").map(|stroke| {
        if stroke.to_string().as_str() == "none" {
            return (0.0, 0.0, 0.0, 0.0);
        }
        if stroke.to_string().as_str() == "currentColor" {
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
    }).unwrap_or(style.get("stroke").map(|stroke| {
        if stroke.as_str() == "none" {
            return (0.0, 0.0, 0.0, 0.0);
        }
        if stroke.as_str() == "currentColor" {
            return (0.0, 0.0, 0.0, 0.0);
        }
        let color = parse_color(stroke.as_str());
        let opacity = style.get("stroke-opacity").map(|opacity| {
            opacity.parse().unwrap()
        }).unwrap_or(-1.0);
        match color {
            CssColor::RGBA(ref rgba) => {
                (rgba.red_f32() as f64, rgba.green_f32() as f64, rgba.blue_f32() as f64, if opacity == -1.0 { rgba.alpha_f32() as f64 } else { opacity })
            }
            _ => (0.0, 0.0, 0.0, 1.0),
        }
    }).unwrap_or(stroke.clone()));
    let stroke_width = attributes.get("stroke-width").map(|width| {
        width.parse().unwrap()
    }).unwrap_or(style.get("stroke-width").map(|width| {
        width.parse().unwrap()
    }).unwrap_or(*sw));
    let line_cap = attributes.get("stroke-linecap").map(|cap| {
        cap.to_string()
    }).unwrap_or(style.get("stroke-linecap").map(|cap| {
        cap.to_string()
    }).unwrap_or(lc.to_string()));
    let line_join = attributes.get("stroke-linejoin").map(|join| {
        join.to_string()
    }).unwrap_or(style.get("stroke-linejoin").map(|join| {
        join.to_string()
    }).unwrap_or(lj.to_string()));
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
    let mut transform = attributes.get("transform").map(|transform| {
        transform.to_string()
    });
    if transform.is_none() {
        transform = style.get("transform").cloned();
    }
    if transform.is_some() {
        transforms.push(vec![parse_transform(transform.unwrap().as_str())]);
    }
    for transform in transforms.iter().rev() {
        for matrix in transform.iter().rev() {
            let new_points = points.iter().map(|point| {
                let new_x = matrix.a as f64 * point.0 + matrix.c as f64 * point.1 + matrix.e as f64;
                let new_y = matrix.b as f64 * point.0 + matrix.d as f64 * point.1 + matrix.f as f64;
                (new_x, new_y)
            }).collect::<Vec<(f64, f64)>>();
            points = new_points;
        }
    }
    let vec_obj = VectorObject {
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
        fill_rule: match fill_rule.as_str() {
            "evenodd" => "evenodd",
            _ => "nonzero",
        },
        index: index,
    };
    return vec_obj;
}


fn parse_rect(
    attributes: &std::collections::HashMap<String, Value>,
    index: usize,
    fill: &(f64, f64, f64, f64),
    stroke: &(f64, f64, f64, f64),
    sw: &f64,
    lc: &&str,
    lj: &&str,
    transforms: &Vec<Vec<Matrix<f32>>>
) -> VectorObject {
    let mut transforms = transforms.clone();
    let style = attributes.get("style").map(|style| {
        parse_style_attr(style.to_string().as_str())
    }).unwrap_or(HashMap::new());
    let x = attributes.get("x").map(|x| {
        x.parse().unwrap()
    }).unwrap_or(style.get("x").map(|x| {
        x.parse().unwrap()
    }).unwrap_or(0.0));
    let y = attributes.get("y").map(|y| {
        y.parse().unwrap()
    }).unwrap_or(style.get("y").map(|y| {
        y.parse().unwrap()
    }).unwrap_or(0.0));
    let width = attributes.get("width").map(|width| {
        width.parse().unwrap()
    }).unwrap_or(style.get("width").map(|width| {
        width.parse().unwrap()
    }).unwrap_or(0.0));
    let height = attributes.get("height").map(|height| {
        height.parse().unwrap()
    }).unwrap_or(style.get("height").map(|height| {
        height.parse().unwrap()
    }).unwrap_or(0.0));
    let fill_color = attributes.get("fill").map(|fill| {
        if fill.to_string().as_str() == "none" {
            return (0.0, 0.0, 0.0, 0.0);
        }
        if fill.to_string().as_str() == "currentColor" {
            return (0.0, 0.0, 0.0, 1.0);
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
    }).unwrap_or(style.get("fill").map(|fill| {
        if fill.as_str() == "none" {
            return (0.0, 0.0, 0.0, 0.0);
        }
        if fill.as_str() == "currentColor" {
            return (0.0, 0.0, 0.0, 1.0);
        }
        let color = parse_color(fill.as_str());
        let opacity = style.get("fill-opacity").map(|opacity| {
            opacity.parse().unwrap()
        }).unwrap_or(-1.0);
        match color {
            CssColor::RGBA(ref rgba) => {
                (rgba.red_f32() as f64, rgba.green_f32() as f64, rgba.blue_f32() as f64, if opacity == -1.0 { rgba.alpha_f32() as f64 } else { opacity })
            }
            _ => (0.0, 0.0, 0.0, 1.0),
        }
    }).unwrap_or(fill.clone()));
    let stroke_color = attributes.get("stroke").map(|stroke| {
        if stroke.to_string().as_str() == "none" {
            return (0.0, 0.0, 0.0, 0.0);
        }
        if stroke.to_string().as_str() == "currentColor" {
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
    }).unwrap_or(style.get("stroke").map(|stroke| {
        if stroke.as_str() == "none" {
            return (0.0, 0.0, 0.0, 0.0);
        }
        if stroke.as_str() == "currentColor" {
            return (0.0, 0.0, 0.0, 0.0);
        }
        let color = parse_color(stroke.as_str());
        let opacity = style.get("stroke-opacity").map(|opacity| {
            opacity.parse().unwrap()
        }).unwrap_or(-1.0);
        match color {
            CssColor::RGBA(ref rgba) => {
                (rgba.red_f32() as f64, rgba.green_f32() as f64, rgba.blue_f32() as f64, if opacity == -1.0 { rgba.alpha_f32() as f64 } else { opacity })
            }
            _ => (0.0, 0.0, 0.0, 1.0),
        }
    }).unwrap_or(stroke.clone()));
    let stroke_width = attributes.get("stroke-width").map(|width| {
        width.parse().unwrap()
    }).unwrap_or(style.get("stroke-width").map(|width| {
        width.parse().unwrap()
    }).unwrap_or(*sw));
    let line_cap = attributes.get("stroke-linecap").map(|cap| {
        cap.to_string()
    }).unwrap_or(style.get("stroke-linecap").map(|cap| {
        cap.to_string()
    }).unwrap_or(lc.to_string()));
    let line_join = attributes.get("stroke-linejoin").map(|join| {
        join.to_string()
    }).unwrap_or(style.get("stroke-linejoin").map(|join| {
        join.to_string()
    }).unwrap_or(lj.to_string()));
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
    let mut vec_obj = rectangle(
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
    let mut points = vec_obj.points.clone();
    let mut transform = attributes.get("transform").map(|transform| {
        transform.to_string()
    });
    if transform.is_none() {
        transform = style.get("transform").cloned();
    }
    if transform.is_some() {
        transforms.push(vec![parse_transform(transform.unwrap().as_str())]);
    }
    for transform in transforms.iter().rev() {
        for matrix in transform.iter().rev() {
            let new_points = points.iter().map(|point| {
                let new_x = matrix.a as f64 * point.0 + matrix.c as f64 * point.1 + matrix.e as f64;
                let new_y = matrix.b as f64 * point.0 + matrix.d as f64 * point.1 + matrix.f as f64;
                (new_x, new_y)
            }).collect::<Vec<(f64, f64)>>();
            points = new_points;
        }
    }
    vec_obj.points = points;
    return vec_obj;
}


fn parse_circle(
    attributes: &std::collections::HashMap<String, Value>,
    index: usize,
    fill: &(f64, f64, f64, f64),
    stroke: &(f64, f64, f64, f64),
    sw: &f64,
    lc: &&str,
    lj: &&str,
    transforms: &Vec<Vec<Matrix<f32>>>
) -> VectorObject {
    let mut transforms = transforms.clone();
    let style = attributes.get("style").map(|style| {
        parse_style_attr(style.to_string().as_str())
    }).unwrap_or(HashMap::new());
    let cx = attributes.get("cx").map(|cx| {
        cx.parse().unwrap()
    }).unwrap_or(style.get("cx").map(|cx| {
        cx.parse().unwrap()
    }).unwrap_or(0.0));
    let cy = attributes.get("cy").map(|cy| {
        cy.parse().unwrap()
    }).unwrap_or(style.get("cy").map(|cy| {
        cy.parse().unwrap()
    }).unwrap_or(0.0));
    let r = attributes.get("r").map(|r| {
        r.parse().unwrap()
    }).unwrap_or(style.get("r").map(|r| {
        r.parse().unwrap()
    }).unwrap_or(0.0));
    let fill_color = attributes.get("fill").map(|fill| {
        if fill.to_string().as_str() == "none" {
            return (0.0, 0.0, 0.0, 0.0);
        }
        if fill.to_string().as_str() == "currentColor" {
            return (0.0, 0.0, 0.0, 1.0);
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
    }).unwrap_or(style.get("fill").map(|fill| {
        if fill.as_str() == "none" {
            return (0.0, 0.0, 0.0, 0.0);
        }
        if fill.as_str() == "currentColor" {
            return (0.0, 0.0, 0.0, 1.0);
        }
        let color = parse_color(fill.as_str());
        let opacity = style.get("fill-opacity").map(|opacity| {
            opacity.parse().unwrap()
        }).unwrap_or(-1.0);
        match color {
            CssColor::RGBA(ref rgba) => {
                (rgba.red_f32() as f64, rgba.green_f32() as f64, rgba.blue_f32() as f64, if opacity == -1.0 { rgba.alpha_f32() as f64 } else { opacity })
            }
            _ => (0.0, 0.0, 0.0, 1.0),
        }
    }).unwrap_or(fill.clone()));
    let stroke_color = attributes.get("stroke").map(|stroke| {
        if stroke.to_string().as_str() == "none" {
            return (0.0, 0.0, 0.0, 0.0);
        }
        if stroke.to_string().as_str() == "currentColor" {
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
    }).unwrap_or(style.get("stroke").map(|stroke| {
        if stroke.as_str() == "none" {
            return (0.0, 0.0, 0.0, 0.0);
        }
        if stroke.as_str() == "currentColor" {
            return (0.0, 0.0, 0.0, 0.0);
        }
        let color = parse_color(stroke.as_str());
        let opacity = style.get("stroke-opacity").map(|opacity| {
            opacity.parse().unwrap()
        }).unwrap_or(-1.0);
        match color {
            CssColor::RGBA(ref rgba) => {
                (rgba.red_f32() as f64, rgba.green_f32() as f64, rgba.blue_f32() as f64, if opacity == -1.0 { rgba.alpha_f32() as f64 } else { opacity })
            }
            _ => (0.0, 0.0, 0.0, 1.0),
        }
    }).unwrap_or(stroke.clone()));
    let stroke_width = attributes.get("stroke-width").map(|width| {
        width.parse().unwrap()
    }).unwrap_or(style.get("stroke-width").map(|width| {
        width.parse().unwrap()
    }).unwrap_or(*sw));
    let line_cap = attributes.get("stroke-linecap").map(|cap| {
        cap.to_string()
    }).unwrap_or(style.get("stroke-linecap").map(|cap| {
        cap.to_string()
    }).unwrap_or(lc.to_string()));
    let line_join = attributes.get("stroke-linejoin").map(|join| {
        join.to_string()
    }).unwrap_or(style.get("stroke-linejoin").map(|join| {
        join.to_string()
    }).unwrap_or(lj.to_string()));
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
    let mut vec_obj = circle(
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
    let mut points = vec_obj.points.clone();
    let mut transform = attributes.get("transform").map(|transform| {
        transform.to_string()
    });
    if transform.is_none() {
        transform = style.get("transform").cloned();
    }
    if transform.is_some() {
        transforms.push(vec![parse_transform(transform.unwrap().as_str())]);
    }
    for transform in transforms.iter().rev() {
        for matrix in transform.iter().rev() {
            let new_points = points.iter().map(|point| {
                let new_x = matrix.a as f64 * point.0 + matrix.c as f64 * point.1 + matrix.e as f64;
                let new_y = matrix.b as f64 * point.0 + matrix.d as f64 * point.1 + matrix.f as f64;
                (new_x, new_y)
            }).collect::<Vec<(f64, f64)>>();
            points = new_points;
        }
    }
    vec_obj.points = points;
    return vec_obj;
}


fn parse_polygon(
    attributes: &std::collections::HashMap<String, Value>,
    index: usize,
    fill: &(f64, f64, f64, f64),
    stroke: &(f64, f64, f64, f64),
    sw: &f64,
    lc: &&str,
    lj: &&str,
    transforms: &Vec<Vec<Matrix<f32>>>
) -> VectorObject {
    let style = attributes.get("style").map(|style| {
        parse_style_attr(style.to_string().as_str())
    }).unwrap_or(HashMap::new());
    let mut transforms = transforms.clone();
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
        if fill.to_string().as_str() == "currentColor" {
            return (0.0, 0.0, 0.0, 1.0);
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
    }).unwrap_or(style.get("fill").map(|fill| {
        if fill.as_str() == "none" {
            return (0.0, 0.0, 0.0, 0.0);
        }
        if fill.as_str() == "currentColor" {
            return (0.0, 0.0, 0.0, 1.0);
        }
        let color = parse_color(fill.as_str());
        let opacity = style.get("fill-opacity").map(|opacity| {
            opacity.parse().unwrap()
        }).unwrap_or(-1.0);
        match color {
            CssColor::RGBA(ref rgba) => {
                (rgba.red_f32() as f64, rgba.green_f32() as f64, rgba.blue_f32() as f64, if opacity == -1.0 { rgba.alpha_f32() as f64 } else { opacity })
            }
            _ => (0.0, 0.0, 0.0, 1.0),
        }
    }).unwrap_or(fill.clone()));
    let stroke_color = attributes.get("stroke").map(|stroke| {
        if stroke.to_string().as_str() == "none" {
            return (0.0, 0.0, 0.0, 0.0);
        }
        if stroke.to_string().as_str() == "currentColor" {
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
    }).unwrap_or(style.get("stroke").map(|stroke| {
        if stroke.as_str() == "none" {
            return (0.0, 0.0, 0.0, 0.0);
        }
        if stroke.as_str() == "currentColor" {
            return (0.0, 0.0, 0.0, 0.0);
        }
        let color = parse_color(stroke.as_str());
        let opacity = style.get("stroke-opacity").map(|opacity| {
            opacity.parse().unwrap()
        }).unwrap_or(-1.0);
        match color {
            CssColor::RGBA(ref rgba) => {
                (rgba.red_f32() as f64, rgba.green_f32() as f64, rgba.blue_f32() as f64, if opacity == -1.0 { rgba.alpha_f32() as f64 } else { opacity })
            }
            _ => (0.0, 0.0, 0.0, 1.0),
        }
    }).unwrap_or(stroke.clone()));
    let stroke_width = attributes.get("stroke-width").map(|width| {
        width.parse().unwrap()
    }).unwrap_or(style.get("stroke-width").map(|width| {
        width.parse().unwrap()
    }).unwrap_or(*sw));
    let line_cap = attributes.get("stroke-linecap").map(|cap| {
        cap.to_string()
    }).unwrap_or(style.get("stroke-linecap").map(|cap| {
        cap.to_string()
    }).unwrap_or(lc.to_string()));
    let line_join = attributes.get("stroke-linejoin").map(|join| {
        join.to_string()
    }).unwrap_or(style.get("stroke-linejoin").map(|join| {
        join.to_string()
    }).unwrap_or(lj.to_string()));
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
    let mut polygon = polygon(
        points,
        Some(stroke_color),
        Some(fill_color),
        Some(stroke_width),
        Some(line_cap),
        Some(line_join),
        Some(index)
    );
    let mut transform = attributes.get("transform").map(|transform| {
        transform.to_string()
    });
    if transform.is_none() {
        transform = style.get("transform").cloned();
    }
    if transform.is_some() {
        transforms.push(vec![parse_transform(transform.unwrap().as_str())]);
    }
    let mut points = polygon.points.clone();
    for transform in transforms.iter().rev() {
        for matrix in transform.iter().rev() {
            let new_points = points.iter().map(|point| {
                let new_x = matrix.a as f64 * point.0 + matrix.c as f64 * point.1 + matrix.e as f64;
                let new_y = matrix.b as f64 * point.0 + matrix.d as f64 * point.1 + matrix.f as f64;
                (new_x, new_y)
            }).collect::<Vec<(f64, f64)>>();
            points = new_points;
        }
    }
    polygon.points = points;
    return polygon;
}


pub fn svg_to_vector_pin<'a>(svg: &'a str, font_array_buffers: Option<Map>) -> Pin<Box<dyn Future<Output = VectorObject> + 'a>> {
    Box::pin(async move {
        let mut text_fill = Vec::new();
        let mut text_stroke = Vec::new();
        let mut fw = Vec::new();
        let mut fstyle = Vec::new();
        let mut text_stroke_width = Vec::new();
        let mut text_line_cap = Vec::new();
        let mut text_line_join = Vec::new();
        let mut text_applied_transforms = Vec::new();
        let mut text = Vec::new();
        let mut x_text = Vec::new();
        let mut y_text = Vec::new();
        let mut fs_text = Vec::new();
        let mut ff_text = Vec::new();
        let mut id_vec_obj_map = std::collections::HashMap::new();
        let mut subobjects = Vec::new();
        let mut subobjects_indices = Vec::new();
        let mut fill = Vec::new();
        let mut applied_fill = Vec::new();
        let mut stroke = Vec::new();
        let mut applied_stroke = Vec::new();
        let mut sw = Vec::new();
        let mut applied_sw = Vec::new();
        let mut lc = Vec::new();
        let mut applied_lc = Vec::new();
        let mut lj = Vec::new();
        let mut applied_lj = Vec::new();
        let mut transforms = Vec::new();
        let mut applied_transforms = Vec::new();
        let mut index = 1 as usize;
        let mut in_defs = false;
        for event in svg::read(svg).unwrap() {
            match event {
                Event::Tag("defs", Type::Start, _) => {
                    in_defs = true;
                },
                Event::Tag("defs", Type::End, _) => {
                    in_defs = false;
                },
                Event::Tag("g", Type::Start, attributes) => {
                    let fill_cur = attributes.get("fill").map(|fill| {
                        if fill.to_string().as_str() == "none" {
                            return (0.0, 0.0, 0.0, 0.0);
                        }
                        if fill.to_string().as_str() == "currentColor" {
                            return (0.0, 0.0, 0.0, 1.0);
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
                    applied_fill.push(fill_cur.is_some());
                    if fill_cur.is_some() {
                        fill.push(fill_cur.unwrap());
                    }
                    let stroke_cur = attributes.get("stroke").map(|stroke| {
                        if stroke.to_string().as_str() == "none" {
                            return (0.0, 0.0, 0.0, 0.0);
                        }
                        if stroke.to_string().as_str() == "currentColor" {
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
                    applied_stroke.push(stroke_cur.is_some());
                    if stroke_cur.is_some() {
                        stroke.push(stroke_cur.unwrap());
                    }
                    let sw_cur = attributes.get("stroke-width").map(|width| {
                        width.parse::<f64>().unwrap()
                    });
                    applied_sw.push(sw_cur.is_some());
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
                    applied_lc.push(lc_cur.is_some());
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
                    applied_lj.push(lj_cur.is_some());
                    if lj_cur.is_some() {
                        lj.push(lj_cur.unwrap());
                    }
                    let transform_attr = attributes.get("transform").map(|transform| {
                        transform.to_string()
                    });
                    applied_transforms.push(transform_attr.is_some());
                    if transform_attr.is_some() {
                        transforms.push(vec![parse_transform(transform_attr.unwrap().as_str())]);
                    }
                }
                Event::Tag("g", Type::End, _) => {
                    if fill.len() > 0 && applied_fill.pop().unwrap() {
                        fill.pop();
                    }
                    if stroke.len() > 0 && applied_stroke.pop().unwrap() {
                        stroke.pop();
                    }
                    if sw.len() > 0 && applied_sw.pop().unwrap() {
                        sw.pop();
                    }
                    if lc.len() > 0 && applied_lc.pop().unwrap() {
                        lc.pop();
                    }
                    if lj.len() > 0 && applied_lj.pop().unwrap() {
                        lj.pop();
                    }
                    if transforms.len() > 0 && applied_transforms.pop().unwrap() {
                        transforms.pop();
                    }
                },
                Event::Tag("ellipse", _, attributes) => {
                    let fill_color = fill.last().unwrap_or(&(0.0, 0.0, 0.0, 1.0));
                    let stroke_color = stroke.last().unwrap_or(&(0.0, 0.0, 0.0, 0.0));
                    let stroke_width = sw.last().unwrap_or(&0.0).clone();
                    let line_cap = lc.last().unwrap_or(&&"butt");
                    let line_join = lj.last().unwrap_or(&&"miter");
                    let vec_obj = parse_ellipse(&attributes, index, fill_color, stroke_color, &stroke_width, &line_cap, &line_join, &transforms);
                    let id = attributes.get("id").map(|id| {
                        id.to_string()
                    });
                    if id.is_some() && in_defs {
                        id_vec_obj_map.insert(id, vec_obj.clone());
                    } else {
                        subobjects.push(vec_obj.clone());
                        subobjects_indices.push(index);
                    }
                    index += 1;
                },
                Event::Tag("svg", _, _) => {},
                Event::Tag("path", _, attributes) => {
                    // Apply transforms and fill/stroke/line_cap/line_join/stroke_width
                    let fill_color = fill.last().unwrap_or(&(0.0, 0.0, 0.0, 1.0));
                    let stroke_color = stroke.last().unwrap_or(&(0.0, 0.0, 0.0, 0.0));
                    let stroke_width = sw.last().unwrap_or(&0.0).clone();
                    let line_cap = lc.last().unwrap_or(&&"butt");
                    let line_join = lj.last().unwrap_or(&&"miter");
                    let vec_obj = parse_path(&attributes, index, fill_color, stroke_color, &stroke_width, &line_cap, &line_join, &transforms);
                    let id = attributes.get("id").map(|id| {
                        id.to_string()
                    });
                    if id.is_some() && in_defs {
                        id_vec_obj_map.insert(id, vec_obj.clone());
                    } else {
                        subobjects.push(vec_obj.clone());
                        subobjects_indices.push(index);
                    }
                    index += 1;
                },
                Event::Tag("rect", _, attributes) => {
                    let fill_color = fill.last().unwrap_or(&(0.0, 0.0, 0.0, 1.0));
                    let stroke_color = stroke.last().unwrap_or(&(0.0, 0.0, 0.0, 0.0));
                    let stroke_width = sw.last().unwrap_or(&0.0).clone();
                    let line_cap = lc.last().unwrap_or(&&"butt");
                    let line_join = lj.last().unwrap_or(&&"miter");
                    let vec_obj = parse_rect(&attributes, index, fill_color, stroke_color, &stroke_width, &line_cap, &line_join, &transforms);
                    let id = attributes.get("id").map(|id| {
                        id.to_string()
                    });
                    if id.is_some() && in_defs {
                        id_vec_obj_map.insert(id, vec_obj.clone());
                    } else {
                        subobjects.push(vec_obj.clone());
                        subobjects_indices.push(index);
                    }
                    index += 1;
                }
                Event::Tag("circle", _, attributes) => {
                    let fill_color = fill.last().unwrap_or(&(0.0, 0.0, 0.0, 1.0));
                    let stroke_color = stroke.last().unwrap_or(&(0.0, 0.0, 0.0, 0.0));
                    let stroke_width = sw.last().unwrap_or(&0.0).clone();
                    let line_cap = lc.last().unwrap_or(&&"butt");
                    let line_join = lj.last().unwrap_or(&&"miter");
                    let vec_obj = parse_circle(&attributes, index, fill_color, stroke_color, &stroke_width, &line_cap, &line_join, &transforms);
                    let id = attributes.get("id").map(|id| {
                        id.to_string()
                    });
                    if id.is_some() && in_defs {
                        id_vec_obj_map.insert(id, vec_obj.clone());
                    } else {
                        subobjects.push(vec_obj.clone());
                        subobjects_indices.push(index);
                    }
                    index += 1;
                }
                Event::Tag("polygon", _, attributes) => {
                    let fill_color = fill.last().unwrap_or(&(0.0, 0.0, 0.0, 1.0));
                    let stroke_color = stroke.last().unwrap_or(&(0.0, 0.0, 0.0, 0.0));
                    let stroke_width = sw.last().unwrap_or(&0.0).clone();
                    let line_cap = lc.last().unwrap_or(&&"butt");
                    let line_join = lj.last().unwrap_or(&&"miter");
                    let vec_obj = parse_polygon(&attributes, index, fill_color, stroke_color, &stroke_width, &line_cap, &line_join, &transforms);
                    let id = attributes.get("id").map(|id| {
                        id.to_string()
                    });
                    if id.is_some() && in_defs {
                        id_vec_obj_map.insert(id, vec_obj.clone());
                    } else {
                        subobjects.push(vec_obj.clone());
                        subobjects_indices.push(index);
                    }
                    index += 1;
                }
                Event::Tag("use", _, attributes) => {
                    let id = attributes.get("id").map(|id| {
                        id.to_string()
                    });
                    let x_link_href = attributes.get("xlink:href").map(
                        |xlink_href| {
                            xlink_href[1..].to_string()
                        }
                    );
                    let href = attributes.get("href").map(
                        |href| {
                            href[1..].to_string()
                        }
                    );
                    if x_link_href.is_none() && href.is_none() {
                        continue;
                    }
                    let x = attributes.get("x").map(|x| {
                        x.parse().unwrap()
                    }).unwrap_or(0.0);
                    let y = attributes.get("y").map(|y| {
                        y.parse().unwrap()
                    }).unwrap_or(0.0);
                    let transform = attributes.get("transform").map(|transform| {
                        transform.to_string()
                    });
                    let vec_obj = if href.is_some() {
                        id_vec_obj_map.get(&href).unwrap()
                    } else if x_link_href.is_some() {
                        id_vec_obj_map.get(&x_link_href).unwrap()
                    } else {
                        log(&format!("Warning: no object with id: {:?}", x_link_href.clone().unwrap_or(href.clone().unwrap())));
                        continue;
                    };
                    let mut vec_obj = vec_obj.clone();
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
                    let mut points = vec_obj.points.clone();
                    let mut new_points = Vec::new();
                    let mut has_transform = false;
                    if transform.is_some() {
                        transforms.push(vec![parse_transform(transform.unwrap().as_str())]);
                        has_transform = true;
                    }
                    for transform in transforms.iter().rev() {
                        for matrix in transform.iter().rev() {
                            for point in points.clone() {
                                let new_x = matrix.a as f64 * point.0 + matrix.c as f64 * point.1 + matrix.e as f64;
                                let new_y = matrix.b as f64 * point.0 + matrix.d as f64 * point.1 + matrix.f as f64;
                                new_points.push((new_x, new_y));
                            }
                            points = new_points.clone();
                            new_points = Vec::new();
                        }
                    }
                    vec_obj.points = points;
                    if in_defs && id.is_some() {
                        id_vec_obj_map.insert(id, vec_obj.clone());
                    } else {
                        subobjects.push(vec_obj.clone());
                    }
                    index += 1;
                    if has_transform {
                        transforms.pop();
                    }
                }
                Event::Tag("text", Type::Start, attributes) => {
                    let style = attributes.get("style").map(|style| {
                        parse_style_attr(style.to_string().as_str())
                    }).unwrap_or(HashMap::new());
                    let fill_for_text = attributes.get("fill").map(|fill| {
                        if fill.to_string().as_str() == "none" {
                            return (0.0, 0.0, 0.0, 0.0);
                        }
                        if fill.to_string().as_str() == "currentColor" {
                            return (0.0, 0.0, 0.0, 1.0);
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
                    // }).unwrap_or(*fill.last().unwrap_or(&(0.0, 0.0, 0.0, 1.0))).clone();
                    }).unwrap_or(style.get("fill").map(|fill| {
                        if fill.as_str() == "none" {
                            return (0.0, 0.0, 0.0, 0.0);
                        }
                        if fill.as_str() == "currentColor" {
                            return (0.0, 0.0, 0.0, 1.0);
                        }
                        let color = parse_color(fill.as_str());
                        let opacity = style.get("fill-opacity").map(|opacity| {
                            opacity.parse().unwrap()
                        }).unwrap_or(-1.0);
                        match color {
                            CssColor::RGBA(ref rgba) => {
                                (rgba.red_f32() as f64, rgba.green_f32() as f64, rgba.blue_f32() as f64, if opacity == -1.0 { rgba.alpha_f32() as f64 } else { opacity })
                            }
                            _ => (0.0, 0.0, 0.0, 1.0),
                        }
                    }).unwrap_or(*fill.last().unwrap_or(&(0.0, 0.0, 0.0, 1.0))).clone());
                    let stroke_for_text = attributes.get("stroke").map(|stroke| {
                        if stroke.to_string().as_str() == "none" {
                            return (0.0, 0.0, 0.0, 0.0);
                        }
                        if stroke.to_string().as_str() == "currentColor" {
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
                    }).unwrap_or(style.get("stroke").map(|stroke| {
                        if stroke.as_str() == "none" {
                            return (0.0, 0.0, 0.0, 0.0);
                        }
                        if stroke.as_str() == "currentColor" {
                            return (0.0, 0.0, 0.0, 0.0);
                        }
                        let color = parse_color(stroke.as_str());
                        let opacity = style.get("stroke-opacity").map(|opacity| {
                            opacity.parse().unwrap()
                        }).unwrap_or(-1.0);
                        match color {
                            CssColor::RGBA(ref rgba) => {
                                (rgba.red_f32() as f64, rgba.green_f32() as f64, rgba.blue_f32() as f64, if opacity == -1.0 { rgba.alpha_f32() as f64 } else { opacity })
                            }
                            _ => (0.0, 0.0, 0.0, 1.0),
                        }
                    }).unwrap_or(*stroke.last().unwrap_or(&(0.0, 0.0, 0.0, 0.0))).clone());
                    let stroke_width_for_text = attributes.get("stroke-width").map(|width| {
                        width.parse().unwrap()
                    }).unwrap_or(style.get("stroke-width").map(|width| {
                        width.parse().unwrap()
                    }).unwrap_or(*sw.last().unwrap_or(&0.0)).clone());
                    let line_cap_for_text = attributes.get("stroke-linecap").map(|cap| {
                        cap.to_string()
                    }).unwrap_or(style.get("stroke-linecap").map(|cap| {
                        cap.to_string()
                    }).unwrap_or(lc.last().unwrap_or(&&"butt").to_string()).clone());
                    let line_join_for_text = attributes.get("stroke-linejoin").map(|join| {
                        join.to_string()
                    }).unwrap_or(style.get("stroke-linejoin").map(|join| {
                        join.to_string()
                    }).unwrap_or(lj.last().unwrap_or(&&"miter").to_string()).clone());
                    let mut x: Option<f64> = attributes.get("x").map(|x| {
                        x.parse().unwrap()
                    });
                    if x.is_none() {
                        x = style.get("x").map(|x| {
                            x.parse().unwrap()
                        });
                    }
                    let mut y: Option<f64> = attributes.get("y").map(|y| {
                        y.parse().unwrap()
                    });
                    if y.is_none() {
                        y = style.get("y").map(|y| {
                            y.parse().unwrap()
                        });
                    }
                    let font_weight = attributes.get("font-weight").map(|font_weight| {
                        font_weight.to_string()
                    }).unwrap_or(style.get("font-weight").map(|font_weight| {
                        font_weight.to_string()
                    }).unwrap_or("normal".to_string()));
                    let mut font_style = attributes.get("font-style").map(|font_style| {
                        font_style.to_string()
                    });
                    if font_style.is_none() {
                        font_style = style.get("font-style").map(|font_style| {
                            font_style.to_string()
                        });
                    }
                    let mut font_family = attributes.get("font-family").map(|font_family| {
                        font_family.to_string()
                    });
                    if font_family.is_none() {
                        font_family = style.get("font-family").map(|font_family| {
                            font_family.to_string()
                        });
                    }
                    let mut font_size: Option<f64> = attributes.get("font-size").map(|font_size| {
                        if font_size.ends_with("px") {
                            let font_size = font_size[..font_size.len() - 2].parse().unwrap();
                            return font_size;
                        }
                        let font_size = font_size.parse().unwrap();
                        font_size
                    });
                    if font_size.is_none() {
                        font_size = style.get("font-size").map(|font_size| {
                            if font_size.ends_with("px") {
                                let font_size = font_size[..font_size.len() - 2].parse().unwrap();
                                return font_size;
                            }
                            let font_size = font_size.parse().unwrap();
                            font_size
                        });
                    }
                    let mut transf = attributes.get("transform").map(|transform| {
                        transform.to_string()
                    });
                    if transf.is_none() {
                        transf = style.get("transform").cloned();
                    }
                    x_text.push(x);
                    y_text.push(y);
                    fs_text.push(font_size);
                    ff_text.push(font_family);
                    fw.push(font_weight);
                    fstyle.push(font_style);
                    text.push(true);
                    text_fill.push(fill_for_text);
                    text_stroke.push(stroke_for_text);
                    text_stroke_width.push(stroke_width_for_text);
                    text_line_cap.push(line_cap_for_text);
                    text_line_join.push(line_join_for_text);
                    text_applied_transforms.push(transf.is_some());
                    if transf.is_some() {
                        transforms.push(vec![parse_transform(transf.unwrap().as_str())]);
                    }
                },
                Event::Tag("text", Type::End, _) => {
                    if transforms.len() > 0 && text_applied_transforms.pop().unwrap() {
                        transforms.pop();
                    }
                    text.pop();
                    x_text.pop();
                    y_text.pop();
                    fs_text.pop();
                    ff_text.pop();
                    text.pop();
                    text_fill.pop();
                    text_stroke.pop();
                    text_stroke_width.pop();
                    text_line_cap.pop();
                    text_line_join.pop();
                },
                Event::Tag("tspan", Type::Start, attributes) => {
                    let style = attributes.get("style").map(|style| {
                        parse_style_attr(style.to_string().as_str())
                    }).unwrap_or(HashMap::new());
                    let fill_for_text = attributes.get("fill").map(|fill| {
                        if fill.to_string().as_str() == "none" {
                            return (0.0, 0.0, 0.0, 0.0);
                        }
                        if fill.to_string().as_str() == "currentColor" {
                            return (0.0, 0.0, 0.0, 1.0);
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
                    }).unwrap_or(style.get("fill").map(|fill| {
                        if fill.as_str() == "none" {
                            return (0.0, 0.0, 0.0, 0.0);
                        }
                        if fill.as_str() == "currentColor" {
                            return (0.0, 0.0, 0.0, 1.0);
                        }
                        let color = parse_color(fill.as_str());
                        let opacity = style.get("fill-opacity").map(|opacity| {
                            opacity.parse().unwrap()
                        }).unwrap_or(-1.0);
                        match color {
                            CssColor::RGBA(ref rgba) => {
                                (rgba.red_f32() as f64, rgba.green_f32() as f64, rgba.blue_f32() as f64, if opacity == -1.0 { rgba.alpha_f32() as f64 } else { opacity })
                            }
                            _ => (0.0, 0.0, 0.0, 1.0),
                        }
                    }).unwrap_or(*fill.last().unwrap_or(&(0.0, 0.0, 0.0, 1.0))).clone());
                    let stroke_for_text = attributes.get("stroke").map(|stroke| {
                        if stroke.to_string().as_str() == "none" {
                            return (0.0, 0.0, 0.0, 0.0);
                        }
                        if stroke.to_string().as_str() == "currentColor" {
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
                    }).unwrap_or(style.get("stroke").map(|stroke| {
                        if stroke.as_str() == "none" {
                            return (0.0, 0.0, 0.0, 0.0);
                        }
                        if stroke.as_str() == "currentColor" {
                            return (0.0, 0.0, 0.0, 0.0);
                        }
                        let color = parse_color(stroke.as_str());
                        let opacity = style.get("stroke-opacity").map(|opacity| {
                            opacity.parse().unwrap()
                        }).unwrap_or(-1.0);
                        match color {
                            CssColor::RGBA(ref rgba) => {
                                (rgba.red_f32() as f64, rgba.green_f32() as f64, rgba.blue_f32() as f64, if opacity == -1.0 { rgba.alpha_f32() as f64 } else { opacity })
                            }
                            _ => (0.0, 0.0, 0.0, 1.0),
                        }
                    }).unwrap_or(*stroke.last().unwrap_or(&(0.0, 0.0, 0.0, 0.0))).clone());
                    let stroke_width_for_text = attributes.get("stroke-width").map(|width| {
                        width.parse().unwrap()
                    }).unwrap_or(style.get("stroke-width").map(|width| {
                        width.parse().unwrap()
                    }).unwrap_or(*sw.last().unwrap_or(&0.0)).clone());
                    let line_cap_for_text = attributes.get("stroke-linecap").map(|cap| {
                        cap.to_string()
                    }).unwrap_or(style.get("stroke-linecap").map(|cap| {
                        cap.to_string()
                    }).unwrap_or(lc.last().unwrap_or(&&"butt").to_string()).clone());
                    let line_join_for_text = attributes.get("stroke-linejoin").map(|join| {
                        join.to_string()
                    }).unwrap_or(style.get("stroke-linejoin").map(|join| {
                        join.to_string()
                    }).unwrap_or(lj.last().unwrap_or(&&"miter").to_string()).clone());
                    let mut x: Option<f64> = attributes.get("x").map(|x| {
                        x.parse().unwrap()
                    });
                    if x.is_none() {
                        x = style.get("x").map(|x| {
                            x.parse().unwrap()
                        });
                    }
                    let mut y: Option<f64> = attributes.get("y").map(|y| {
                        y.parse().unwrap()
                    });
                    if y.is_none() {
                        y = style.get("y").map(|y| {
                            y.parse().unwrap()
                        });
                    }
                    let font_weight = attributes.get("font-weight").map(|font_weight| {
                        font_weight.to_string()
                    }).unwrap_or(style.get("font-weight").map(|font_weight| {
                        font_weight.to_string()
                    }).unwrap_or("normal".to_string()));
                    let mut font_style = attributes.get("font-style").map(|font_style| {
                        font_style.to_string()
                    });
                    if font_style.is_none() {
                        font_style = style.get("font-style").map(|font_style| {
                            font_style.to_string()
                        });
                    }
                    let mut font_family = attributes.get("font-family").map(|font_family| {
                        font_family.to_string()
                    });
                    if font_family.is_none() {
                        font_family = style.get("font-family").map(|font_family| {
                            font_family.to_string()
                        });
                    }
                    let mut font_size: Option<f64> = attributes.get("font-size").map(|font_size| {
                        if font_size.ends_with("px") {
                            let font_size = font_size[..font_size.len() - 2].parse().unwrap();
                            return font_size;
                        }
                        let font_size = font_size.parse().unwrap();
                        font_size
                    });
                    if font_size.is_none() {
                        font_size = style.get("font-size").map(|font_size| {
                            if font_size.ends_with("px") {
                                let font_size = font_size[..font_size.len() - 2].parse().unwrap();
                                return font_size;
                            }
                            let font_size = font_size.parse().unwrap();
                            font_size
                        });
                    }
                    let transf = attributes.get("transform").map(|transform| {
                        Some(transform.to_string())
                    }).unwrap_or(style.get("transform").map(|transform| {
                        Some(transform.to_string())
                    }).unwrap_or(None));
                    x_text.push(x);
                    y_text.push(y);
                    fs_text.push(font_size);
                    ff_text.push(font_family);
                    fw.push(font_weight);
                    fstyle.push(font_style);
                    text.push(true);
                    text_fill.push(fill_for_text);
                    text_stroke.push(stroke_for_text);
                    text_stroke_width.push(stroke_width_for_text);
                    text_line_cap.push(line_cap_for_text);
                    text_line_join.push(line_join_for_text);
                    text_applied_transforms.push(transf.is_some());
                    if transf.is_some() {
                        transforms.push(vec![parse_transform(transf.unwrap().as_str())]);
                    }
                },
                Event::Tag("tspan", Type::End, _) => {
                    if transforms.len() > 0 && text_applied_transforms.pop().unwrap() {
                        transforms.pop();
                    }
                    text.pop();
                    x_text.pop();
                    y_text.pop();
                    fs_text.pop();
                    ff_text.pop();
                    fw.pop();
                    fstyle.pop();
                    text.pop();
                    text_fill.pop();
                    text_stroke.pop();
                    text_stroke_width.pop();
                    text_line_cap.pop();
                    text_line_join.pop();
                },
                Event::Tag("style", Type::Start, _) => {
                    text.push(false);
                },
                Event::Tag("style", Type::End, _) => {
                    text.pop();
                },
                Event::Tag(tag, _, _) => {
                    log(&format!("Warning: unsupported tag: {:?}", tag));
                },
                Event::Text(text_content) => {
                    if text.len() == 0 {
                        continue;
                    }
                    if !text.last().unwrap() {
                        continue;
                    }
                    if text_content.len() == 0 {
                        continue;
                    }
                    let x = x_text.last().unwrap();
                    let y = y_text.last().unwrap();
                    let font_size = fs_text.last().unwrap();
                    let font_family = ff_text.last().unwrap();
                    let font_weight = fw.last().unwrap();
                    let font_style = fstyle.last().unwrap();
                    let mut vec_obj = text_to_vector(text_content.to_string(), font_array_buffers.clone().unwrap(), font_family.clone(), Some(font_weight.clone()), font_style.clone(), *x, *y, font_size.clone()).await.native_vec_features;
                    let fill_color = text_fill.last().unwrap();
                    let stroke_color = text_stroke.last().unwrap();
                    let stroke_width = text_stroke_width.last().unwrap();
                    let line_cap = text_line_cap.last().unwrap().clone();
                    let line_join = text_line_join.last().unwrap().clone();
                    vec_obj = vec_obj.set_fill(GradientImageOrColor::Color(Color {
                        red: fill_color.0,
                        green: fill_color.1,
                        blue: fill_color.2,
                        alpha: fill_color.3,
                    }), true);
                    vec_obj = vec_obj.set_stroke(GradientImageOrColor::Color(Color {
                        red: stroke_color.0,
                        green: stroke_color.1,
                        blue: stroke_color.2,
                        alpha: stroke_color.3,
                    }), true);
                    vec_obj = vec_obj.set_stroke_width(*stroke_width, true);
                    vec_obj = vec_obj.set_line_cap(match line_cap.as_str() {
                        "butt" => "butt",
                        "square" => "square",
                        "round" => "round",
                        _ => "butt",
                    }, true);
                    vec_obj = vec_obj.set_line_join(match line_join.as_str() {
                        "miter" => "miter",
                        "bevel" => "bevel",
                        "round" => "round",
                        _ => "miter",
                    }, true);
                    let subobjs = vec_obj.subobjects.clone();
                    for (i, subobject) in subobjs.iter().enumerate() {
                        let mut points = subobject.points.clone();
                        let mut new_points = Vec::new();
                        for transform in transforms.iter().rev() {
                            for matrix in transform.iter().rev() {
                                for point in points.clone() {
                                    let new_x = matrix.a as f64 * point.0 + matrix.c as f64 * point.1 + matrix.e as f64;
                                    let new_y = matrix.b as f64 * point.0 + matrix.d as f64 * point.1 + matrix.f as f64;
                                    new_points.push((new_x, new_y));
                                }
                                points = new_points.clone();
                                new_points = Vec::new();
                            }
                        }
                        vec_obj.subobjects[i] = subobject.set_points(points);
                    }
                    subobjects.push(vec_obj);
                    subobjects_indices.push(index);
                }
                Event::Comment(..) => {},
                Event::Instruction(..) => {},
                Event::Declaration(..) => {},
                Event::Error(..) => {
                    log("Error while parsing SVG");
                },
            }
        }
        return VectorObject {
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
            fill_rule: "nonzero",
            index: 0,
        };
    })
}