use js_sys::{Array, Function};
use wasm_bindgen::prelude::*;

use crate::{colors::Color, objects::wasm_interface::{WasmColor, WasmGradientImageOrColor, WasmVectorObject}};

use super::{sphere::sphere, three_d_axes::{coords_to_point_3d, parametric_line_plot_in_axes_3d, parametric_plot_in_axes_3d, plot_in_axes_3d, point_to_coords_3d, three_d_axes}, three_d_object::{matrix_times_points, cross_product, ensure_valid_three_d_color, get_anchors, get_corner_unit_normal, get_end_anchors, get_end_corner, get_end_corner_unit_normal, get_shaded_color, get_shaded_rgb, get_start_anchors, get_start_corner, get_start_corner_unit_normal, line_as_cubic_bezier_3d, matrix_product, project_points, rot_matrix, rot_matrix_euler, shift_points_3d, transpose_matrix, Camera, LightSource, ThreeDObject}};


#[wasm_bindgen(js_name = rotMatrix)]
pub fn rot_matrix_js(
    angle: f64,
    axis: usize
) -> Array {
    let matrix = rot_matrix(angle, axis);
    let result = Array::new();
    for i in 0..3 {
        let row = Array::new();
        for j in 0..3 {
            row.push(&JsValue::from(matrix[i][j]));
        }
        result.push(&row);
    }
    result
}


#[wasm_bindgen(js_name = matrixProduct)]
pub fn matrix_product_js(
    a: Array,
    b: Array
) -> Array {
    let matrix1 = [
        [
            a.get(0).dyn_into::<Array>().unwrap().get(0).as_f64().unwrap(),
            a.get(0).dyn_into::<Array>().unwrap().get(1).as_f64().unwrap(),
            a.get(0).dyn_into::<Array>().unwrap().get(2).as_f64().unwrap()
        ],
        [
            a.get(1).dyn_into::<Array>().unwrap().get(0).as_f64().unwrap(),
            a.get(1).dyn_into::<Array>().unwrap().get(1).as_f64().unwrap(),
            a.get(1).dyn_into::<Array>().unwrap().get(2).as_f64().unwrap()
        ],
        [
            a.get(2).dyn_into::<Array>().unwrap().get(0).as_f64().unwrap(),
            a.get(2).dyn_into::<Array>().unwrap().get(1).as_f64().unwrap(),
            a.get(2).dyn_into::<Array>().unwrap().get(2).as_f64().unwrap()
        ]
    ];
    let matrix2 = [
        [
            b.get(0).dyn_into::<Array>().unwrap().get(0).as_f64().unwrap(),
            b.get(0).dyn_into::<Array>().unwrap().get(1).as_f64().unwrap(),
            b.get(0).dyn_into::<Array>().unwrap().get(2).as_f64().unwrap()
        ],
        [
            b.get(1).dyn_into::<Array>().unwrap().get(0).as_f64().unwrap(),
            b.get(1).dyn_into::<Array>().unwrap().get(1).as_f64().unwrap(),
            b.get(1).dyn_into::<Array>().unwrap().get(2).as_f64().unwrap()
        ],
        [
            b.get(2).dyn_into::<Array>().unwrap().get(0).as_f64().unwrap(),
            b.get(2).dyn_into::<Array>().unwrap().get(1).as_f64().unwrap(),
            b.get(2).dyn_into::<Array>().unwrap().get(2).as_f64().unwrap()
        ]
    ];
    let matrix = matrix_product(matrix1, matrix2);
    let result = Array::new();
    for i in 0..3 {
        let row = Array::new();
        for j in 0..3 {
            row.push(&JsValue::from(matrix[i][j]));
        }
        result.push(&row);
    }
    result
}


#[wasm_bindgen(js_name = rotMatrixEuler)]
pub fn rot_matrix_euler_js(
    phi: f64,
    theta: f64,
    gamma: f64
) -> Array {
    let matrix = rot_matrix_euler(phi, theta, gamma);
    let result = Array::new();
    for i in 0..3 {
        let row = Array::new();
        for j in 0..3 {
            row.push(&JsValue::from(matrix[i][j]));
        }
        result.push(&row);
    }
    result
}


#[wasm_bindgen(js_name = transposeMatrix)]
pub fn transpose_matrix_js(
    a: Array
) -> Array {
    let matrix = [
        [
            a.get(0).dyn_into::<Array>().unwrap().get(0).as_f64().unwrap(),
            a.get(0).dyn_into::<Array>().unwrap().get(1).as_f64().unwrap(),
            a.get(0).dyn_into::<Array>().unwrap().get(2).as_f64().unwrap()
        ],
        [
            a.get(1).dyn_into::<Array>().unwrap().get(0).as_f64().unwrap(),
            a.get(1).dyn_into::<Array>().unwrap().get(1).as_f64().unwrap(),
            a.get(1).dyn_into::<Array>().unwrap().get(2).as_f64().unwrap()
        ],
        [
            a.get(2).dyn_into::<Array>().unwrap().get(0).as_f64().unwrap(),
            a.get(2).dyn_into::<Array>().unwrap().get(1).as_f64().unwrap(),
            a.get(2).dyn_into::<Array>().unwrap().get(2).as_f64().unwrap()
        ]
    ];
    let matrix_t = transpose_matrix(matrix);
    let result = Array::new();
    for i in 0..3 {
        let row = Array::new();
        for j in 0..3 {
            row.push(&JsValue::from(matrix_t[i][j]));
        }
        result.push(&row);
    }
    result
}


#[wasm_bindgen(js_name = applyMatrix)]
pub fn apply_matrix_js(
    matrix: Array,
    points: Array
) -> Array {
    let matrix = [
        [
            matrix.get(0).dyn_into::<Array>().unwrap().get(0).as_f64().unwrap(),
            matrix.get(0).dyn_into::<Array>().unwrap().get(1).as_f64().unwrap(),
            matrix.get(0).dyn_into::<Array>().unwrap().get(2).as_f64().unwrap()
        ],
        [
            matrix.get(1).dyn_into::<Array>().unwrap().get(0).as_f64().unwrap(),
            matrix.get(1).dyn_into::<Array>().unwrap().get(1).as_f64().unwrap(),
            matrix.get(1).dyn_into::<Array>().unwrap().get(2).as_f64().unwrap()
        ],
        [
            matrix.get(2).dyn_into::<Array>().unwrap().get(0).as_f64().unwrap(),
            matrix.get(2).dyn_into::<Array>().unwrap().get(1).as_f64().unwrap(),
            matrix.get(2).dyn_into::<Array>().unwrap().get(2).as_f64().unwrap()
        ]
    ];
    let points = points.iter().map(
        |point| {
            let point = point.dyn_into::<Array>().unwrap();
            (
                point.get(0).as_f64().unwrap(),
                point.get(1).as_f64().unwrap(),
                point.get(2).as_f64().unwrap()
            )
        }
    ).collect::<Vec<(f64, f64, f64)>>();
    let result = matrix_times_points(matrix, points);
    let result_js = Array::new();
    for i in 0..result.len() {
        let point = Array::new();
        point.push(&JsValue::from(result[i].0));
        point.push(&JsValue::from(result[i].1));
        point.push(&JsValue::from(result[i].2));
        result_js.push(&point);
    }
    result_js
}


#[wasm_bindgen(js_name = shiftPoints3D)]
pub fn shift_points_3d_js(
    points: Array,
    shift: Array
) -> Array {
    let points = points.iter().map(
        |point| {
            let point = point.dyn_into::<Array>().unwrap();
            (
                point.get(0).as_f64().unwrap(),
                point.get(1).as_f64().unwrap(),
                point.get(2).as_f64().unwrap()
            )
        }
    ).collect::<Vec<(f64, f64, f64)>>();
    let shift = (
        shift.get(0).as_f64().unwrap(),
        shift.get(1).as_f64().unwrap(),
        shift.get(2).as_f64().unwrap()
    );
    let result = shift_points_3d(&points, shift);
    let result_js = result.iter().map(
        |point| {
            let point_js = Array::new();
            point_js.push(&JsValue::from(point.0));
            point_js.push(&JsValue::from(point.1));
            point_js.push(&JsValue::from(point.2));
            point_js
        }
    ).collect::<Array>();
    result_js
}


#[wasm_bindgen(js_name = ensureValidThreeDColor)]
pub fn ensure_valid_three_d_color_js(
    color: WasmGradientImageOrColor
) -> WasmGradientImageOrColor {
    return WasmGradientImageOrColor {
        gradient_image_or_color: ensure_valid_three_d_color(color.gradient_image_or_color)
    }
}


#[wasm_bindgen]
#[derive(Clone)]
pub struct WasmLightSource {
    #[wasm_bindgen(skip)]
    pub light_source: LightSource
}


#[wasm_bindgen]
impl WasmLightSource {
    #[wasm_bindgen(constructor)]
    pub fn new(
        position: Array
    ) -> WasmLightSource {
        let position = (
            position.get(0).as_f64().unwrap(),
            position.get(1).as_f64().unwrap(),
            position.get(2).as_f64().unwrap()
        );
        return WasmLightSource {
            light_source: LightSource {
                position
            }
        }
    }
    #[wasm_bindgen(js_name = getPosition)]
    pub fn get_position(&self) -> Array {
        let position = self.light_source.position;
        let position_js = Array::new();
        position_js.push(&JsValue::from(position.0));
        position_js.push(&JsValue::from(position.1));
        position_js.push(&JsValue::from(position.2));
        position_js
    }
    #[wasm_bindgen(js_name = clone)]
    pub fn clone_js(&self) -> WasmLightSource {
        self.clone()
    }
}


#[wasm_bindgen]
#[derive(Clone)]
pub struct WasmCamera {
    #[wasm_bindgen(skip)]
    pub camera: Camera
}


#[wasm_bindgen]
impl WasmCamera {
    #[wasm_bindgen(constructor)]
    pub fn new(
        position: Array,
        rotation: Array,
        focal_distance: f64,
        zoom: f64,
    ) -> WasmCamera {
        let position = (
            position.get(0).as_f64().unwrap(),
            position.get(1).as_f64().unwrap(),
            position.get(2).as_f64().unwrap()
        );
        let rotation = (
            rotation.get(0).as_f64().unwrap(),
            rotation.get(1).as_f64().unwrap(),
            rotation.get(2).as_f64().unwrap()
        );
        return WasmCamera {
            camera: Camera {
                position,
                rotation,
                focal_distance,
                zoom,
            }
        }
    }
    #[wasm_bindgen(js_name = getPosition)]
    pub fn get_position_js(&self) -> Array {
        let position = self.camera.position;
        let position_js = Array::new();
        position_js.push(&JsValue::from(position.0));
        position_js.push(&JsValue::from(position.1));
        position_js.push(&JsValue::from(position.2));
        position_js
    }
    #[wasm_bindgen(js_name = getRotation)]
    pub fn get_rotation_js(&self) -> Array {
        let rotation = self.camera.rotation;
        let rotation_js = Array::new();
        rotation_js.push(&JsValue::from(rotation.0));
        rotation_js.push(&JsValue::from(rotation.1));
        rotation_js.push(&JsValue::from(rotation.2));
        rotation_js
    }
    #[wasm_bindgen(js_name = getFocalDistance)]
    pub fn get_focal_distance_js(&self) -> f64 {
        self.camera.focal_distance
    }
    #[wasm_bindgen(js_name = getZoom)]
    pub fn get_zoom_js(&self) -> f64 {
        self.camera.zoom
    }
    #[wasm_bindgen(js_name = clone)]
    pub fn clone_js(&self) -> WasmCamera {
        self.clone()
    }
}


#[wasm_bindgen(js_name = getShadedRgb)]
pub fn get_shaded_rgb_js(
    color: WasmColor,
    point: Array,
    unit_normal: Array,
    light_source: &WasmLightSource
) -> WasmColor {
    let point = (
        point.get(0).as_f64().unwrap(),
        point.get(1).as_f64().unwrap(),
        point.get(2).as_f64().unwrap()
    );
    let unit_normal = (
        unit_normal.get(0).as_f64().unwrap(),
        unit_normal.get(1).as_f64().unwrap(),
        unit_normal.get(2).as_f64().unwrap()
    );
    let light_source = &light_source.light_source;
    let color = color.color;
    let rgb = get_shaded_rgb(&color, point, unit_normal, light_source);
    let rgb_js = WasmColor {
        color: rgb
    };
    rgb_js
}


#[wasm_bindgen(js_name = getStartCorner)]
pub fn get_start_corner_js(
    points: Array
) -> Array {
    let points = points.iter().map(
        |point| {
            let point = point.dyn_into::<Array>().unwrap();
            (
                point.get(0).as_f64().unwrap(),
                point.get(1).as_f64().unwrap(),
                point.get(2).as_f64().unwrap()
            )
        }
    ).collect::<Vec<(f64, f64, f64)>>();
    let start_corner = get_start_corner(&points);
    let point = Array::of3(
        &JsValue::from(start_corner.0),
        &JsValue::from(start_corner.1),
        &JsValue::from(start_corner.2)
    );
    point
}


#[wasm_bindgen(js_name = getEndCorner)]
pub fn get_end_corner_js(
    points: Array
) -> Array {
    let points = points.iter().map(
        |point| {
            let point = point.dyn_into::<Array>().unwrap();
            (
                point.get(0).as_f64().unwrap(),
                point.get(1).as_f64().unwrap(),
                point.get(2).as_f64().unwrap()
            )
        }
    ).collect::<Vec<(f64, f64, f64)>>();
    let end_corner = get_end_corner(&points);
    let point = Array::of3(
        &JsValue::from(end_corner.0),
        &JsValue::from(end_corner.1),
        &JsValue::from(end_corner.2)
    );
    point
}


#[wasm_bindgen(js_name = crossProduct)]
pub fn cross_product_js(
    a: Array,
    b: Array
) -> Array {
    let a = (
        a.get(0).as_f64().unwrap(),
        a.get(1).as_f64().unwrap(),
        a.get(2).as_f64().unwrap()
    );
    let b = (
        b.get(0).as_f64().unwrap(),
        b.get(1).as_f64().unwrap(),
        b.get(2).as_f64().unwrap()
    );
    let result = cross_product(a, b);
    let result_js = Array::of3(
        &JsValue::from(result.0),
        &JsValue::from(result.1),
        &JsValue::from(result.2)
    );
    result_js
}


#[wasm_bindgen(js_name = getUnitNormal)]
pub fn get_unit_normal_js(
    v1: Array,
    v2: Array
) -> Array {
    let v1 = (
        v1.get(0).as_f64().unwrap(),
        v1.get(1).as_f64().unwrap(),
        v1.get(2).as_f64().unwrap()
    );
    let v2 = (
        v2.get(0).as_f64().unwrap(),
        v2.get(1).as_f64().unwrap(),
        v2.get(2).as_f64().unwrap()
    );
    let normal = cross_product(v1, v2);
    let normal_js = Array::of3(
        &JsValue::from(normal.0),
        &JsValue::from(normal.1),
        &JsValue::from(normal.2)
    );
    normal_js
}


#[wasm_bindgen(js_name = getStartAnchors)]
pub fn get_start_anchors_js(
    points: Array
) -> Array {
    let points = points.iter().map(
        |point| {
            let point = point.dyn_into::<Array>().unwrap();
            (
                point.get(0).as_f64().unwrap(),
                point.get(1).as_f64().unwrap(),
                point.get(2).as_f64().unwrap()
            )
        }
    ).collect::<Vec<(f64, f64, f64)>>();
    let start_anchors = get_start_anchors(&points);
    let start_anchors_js = start_anchors.iter().map(
        |anchor| {
            let anchor_js = Array::of3(
                &JsValue::from(anchor.0),
                &JsValue::from(anchor.1),
                &JsValue::from(anchor.2)
            );
            anchor_js
        }
    ).collect::<Array>();
    start_anchors_js
}


#[wasm_bindgen(js_name = getEndAnchors)]
pub fn get_end_anchors_js(
    points: Array
) -> Array {
    let points = points.iter().map(
        |point| {
            let point = point.dyn_into::<Array>().unwrap();
            (
                point.get(0).as_f64().unwrap(),
                point.get(1).as_f64().unwrap(),
                point.get(2).as_f64().unwrap()
            )
        }
    ).collect::<Vec<(f64, f64, f64)>>();
    let end_anchors = get_end_anchors(&points);
    let end_anchors_js = end_anchors.iter().map(
        |anchor| {
            let anchor_js = Array::of3(
                &JsValue::from(anchor.0),
                &JsValue::from(anchor.1),
                &JsValue::from(anchor.2)
            );
            anchor_js
        }
    ).collect::<Array>();
    end_anchors_js
}


#[wasm_bindgen(js_name = getAnchors)]
pub fn get_anchors_js(
    points: Array
) -> Array {
    let points = points.iter().map(
        |point| {
            let point = point.dyn_into::<Array>().unwrap();
            (
                point.get(0).as_f64().unwrap(),
                point.get(1).as_f64().unwrap(),
                point.get(2).as_f64().unwrap()
            )
        }
    ).collect::<Vec<(f64, f64, f64)>>();
    let anchors = get_anchors(&points);
    let anchors_js = anchors.iter().map(
        |anchor| {
            let anchor_js = Array::of3(
                &JsValue::from(anchor.0),
                &JsValue::from(anchor.1),
                &JsValue::from(anchor.2)
            );
            anchor_js
        }
    ).collect::<Array>();
    anchors_js
}


#[wasm_bindgen(js_name = getCornerUnitNormal)]
pub fn get_corner_unit_normal_js(
    points: Array,
    index: usize
) -> Array {
    let points = points.iter().map(
        |point| {
            let point = point.dyn_into::<Array>().unwrap();
            (
                point.get(0).as_f64().unwrap(),
                point.get(1).as_f64().unwrap(),
                point.get(2).as_f64().unwrap()
            )
        }
    ).collect::<Vec<(f64, f64, f64)>>();
    let corner_unit_normal = get_corner_unit_normal(&points, index);
    let normal_js = Array::of3(
        &JsValue::from(corner_unit_normal.0),
        &JsValue::from(corner_unit_normal.1),
        &JsValue::from(corner_unit_normal.2)
    );
    normal_js
}


#[wasm_bindgen(js_name = getStartCornerUnitNormal)]
pub fn get_start_corner_unit_normal_js(
    points: Array
) -> Array {
    let points = points.iter().map(
        |point| {
            let point = point.dyn_into::<Array>().unwrap();
            (
                point.get(0).as_f64().unwrap(),
                point.get(1).as_f64().unwrap(),
                point.get(2).as_f64().unwrap()
            )
        }
    ).collect::<Vec<(f64, f64, f64)>>();
    let corner_unit_normal = get_start_corner_unit_normal(&points);
    let normal_js = Array::of3(
        &JsValue::from(corner_unit_normal.0),
        &JsValue::from(corner_unit_normal.1),
        &JsValue::from(corner_unit_normal.2)
    );
    normal_js
}


#[wasm_bindgen(js_name = getEndCornerUnitNormal)]
pub fn get_end_corner_unit_normal_js(
    points: Array
) -> Array {
    let points = points.iter().map(
        |point| {
            let point = point.dyn_into::<Array>().unwrap();
            (
                point.get(0).as_f64().unwrap(),
                point.get(1).as_f64().unwrap(),
                point.get(2).as_f64().unwrap()
            )
        }
    ).collect::<Vec<(f64, f64, f64)>>();
    let corner_unit_normal = get_end_corner_unit_normal(&points);
    let normal_js = Array::of3(
        &JsValue::from(corner_unit_normal.0),
        &JsValue::from(corner_unit_normal.1),
        &JsValue::from(corner_unit_normal.2)
    );
    normal_js
}


#[wasm_bindgen(js_name = getShadedColor)]
pub fn get_shaded_color_js(
    color: &WasmGradientImageOrColor,
    points: Array,
    light_source: &WasmLightSource,
    camera: &WasmCamera
) -> WasmGradientImageOrColor {
    let points = points.iter().map(
        |point| {
            let point = point.dyn_into::<Array>().unwrap();
            (
                point.get(0).as_f64().unwrap(),
                point.get(1).as_f64().unwrap(),
                point.get(2).as_f64().unwrap()
            )
        }
    ).collect::<Vec<(f64, f64, f64)>>();
    let light_source = &light_source.light_source;
    let camera = &camera.camera;
    let color = &color.gradient_image_or_color;
    let color = get_shaded_color(color, &points, light_source, camera);
    let rgb_js = WasmGradientImageOrColor {
        gradient_image_or_color: color
    };
    rgb_js
}


#[wasm_bindgen(js_name = projectPoints)]
pub fn project_points_js(
    points: Array,
    camera: &WasmCamera
) -> Array {
    let points = points.iter().map(
        |point| {
            let point = point.dyn_into::<Array>().unwrap();
            (
                point.get(0).as_f64().unwrap(),
                point.get(1).as_f64().unwrap(),
                point.get(2).as_f64().unwrap()
            )
        }
    ).collect::<Vec<(f64, f64, f64)>>();
    let camera = &camera.camera;
    let result = project_points(&points, camera);
    let result_js = result.iter().map(
        |point| {
            let point_js = Array::new();
            point_js.push(&JsValue::from(point.0));
            point_js.push(&JsValue::from(point.1));
            point_js
        }
    ).collect::<Array>();
    result_js
}


#[wasm_bindgen(js_name = lineAsCubicBezier3D)]
pub fn line_as_cubic_bezier_3d_js(
    point1: Array,
    point2: Array
) -> Array {
    let point1 = (
        point1.get(0).as_f64().unwrap(),
        point1.get(1).as_f64().unwrap(),
        point1.get(2).as_f64().unwrap()
    );
    let point2 = (
        point2.get(0).as_f64().unwrap(),
        point2.get(1).as_f64().unwrap(),
        point2.get(2).as_f64().unwrap()
    );
    let result = line_as_cubic_bezier_3d(point1, point2);
    let result_js = result.iter().map(
        |point| {
            let point_js = Array::of3(
                &JsValue::from(point.0),
                &JsValue::from(point.1),
                &JsValue::from(point.2)
            );
            point_js
        }
    ).collect::<Array>();
    result_js
}


#[wasm_bindgen]
#[derive(Clone)]
pub struct WasmThreeDObject {
    #[wasm_bindgen(skip)]
    pub three_d_object: ThreeDObject
}


#[wasm_bindgen]
impl WasmThreeDObject {
    #[wasm_bindgen(constructor)]
    pub fn new(
        points: Array,
        subobjects: Vec<WasmThreeDObject>,
        fill: WasmGradientImageOrColor,
        stroke: WasmGradientImageOrColor,
        stroke_width: f64,
        index: usize
    ) -> WasmThreeDObject {
        let points = points.iter().map(
            |point| {
                let point = point.dyn_into::<Array>().unwrap();
                (
                    point.get(0).as_f64().unwrap(),
                    point.get(1).as_f64().unwrap(),
                    point.get(2).as_f64().unwrap()
                )
            }
        ).collect::<Vec<(f64, f64, f64)>>();
        let subobjects = subobjects.iter().map(
            |face| {
                face.three_d_object.clone()
            }
        ).collect::<Vec<ThreeDObject>>();
        WasmThreeDObject {
            three_d_object: ThreeDObject::new(
                points,
                subobjects,
                fill.gradient_image_or_color,
                stroke.gradient_image_or_color,
                stroke_width,
                index
            )
        }
    }
    #[wasm_bindgen(js_name = getPoints)]
    pub fn get_points(&self) -> Array {
        let points = &self.three_d_object.points;
        let points_js = points.iter().map(
            |point| {
                let point_js = Array::of3(
                    &JsValue::from(point.0),
                    &JsValue::from(point.1),
                    &JsValue::from(point.2)
                );
                point_js
            }
        ).collect::<Array>();
        points_js
    }
    #[wasm_bindgen(js_name = getSubobjects)]
    pub fn get_subobjects(&self) -> Vec<WasmThreeDObject> {
        let faces = &self.three_d_object.subobjects;
        let faces_js = faces.iter().map(
            |face| {
                WasmThreeDObject {
                    three_d_object: face.clone()
                }
            }
        ).collect::<Vec<WasmThreeDObject>>();
        faces_js
    }
    #[wasm_bindgen(js_name = getFill)]
    pub fn get_fill(&self) -> WasmGradientImageOrColor {
        return WasmGradientImageOrColor {
            gradient_image_or_color: self.three_d_object.fill.clone()
        }
    }
    #[wasm_bindgen(js_name = getStroke)]
    pub fn get_stroke(&self) -> WasmGradientImageOrColor {
        return WasmGradientImageOrColor {
            gradient_image_or_color: self.three_d_object.stroke.clone()
        }
    }
    #[wasm_bindgen(js_name = getStrokeWidth)]
    pub fn get_stroke_width(&self) -> f64 {
        self.three_d_object.stroke_width
    }
    #[wasm_bindgen(js_name = getAnchorsAndHandles)]
    pub fn get_anchors_and_handles(&self) -> Array {
        let (a1, h1, h2, a2) = self.three_d_object.get_anchors_and_handles();
        let result = Array::of4(
            &a1.iter().map(
                |anchor| {
                    Array::of3(
                        &JsValue::from(anchor.0),
                        &JsValue::from(anchor.1),
                        &JsValue::from(anchor.2)
                    )
                }
            ).collect::<Array>(),
            &h1.iter().map(
                |handle| {
                    Array::of3(
                        &JsValue::from(handle.0),
                        &JsValue::from(handle.1),
                        &JsValue::from(handle.2)
                    )
                }
            ).collect::<Array>(),
            &h2.iter().map(
                |handle| {
                    Array::of3(
                        &JsValue::from(handle.0),
                        &JsValue::from(handle.1),
                        &JsValue::from(handle.2)
                    )
                }
            ).collect::<Array>(),
            &a2.iter().map(
                |anchor| {
                    Array::of3(
                        &JsValue::from(anchor.0),
                        &JsValue::from(anchor.1),
                        &JsValue::from(anchor.2)
                    )
                }
            ).collect::<Array>()
        );
        result
    }
    #[wasm_bindgen(js_name = setAnchorsAndHandles)]
    pub fn set_anchors_and_handles(&self, anchors_and_handles: Array) -> WasmThreeDObject {
        let a1 = anchors_and_handles.get(0).dyn_into::<Array>().unwrap().iter().map(
            |anchor| {
                let anchor = anchor.dyn_into::<Array>().unwrap();
                (
                    anchor.get(0).as_f64().unwrap(),
                    anchor.get(1).as_f64().unwrap(),
                    anchor.get(2).as_f64().unwrap()
                )
            }
        ).collect::<Vec<(f64, f64, f64)>>();
        let h1 = anchors_and_handles.get(1).dyn_into::<Array>().unwrap().iter().map(
            |handle| {
                let handle = handle.dyn_into::<Array>().unwrap();
                (
                    handle.get(0).as_f64().unwrap(),
                    handle.get(1).as_f64().unwrap(),
                    handle.get(2).as_f64().unwrap()
                )
            }
        ).collect::<Vec<(f64, f64, f64)>>();
        let h2 = anchors_and_handles.get(2).dyn_into::<Array>().unwrap().iter().map(
            |handle| {
                let handle = handle.dyn_into::<Array>().unwrap();
                (
                    handle.get(0).as_f64().unwrap(),
                    handle.get(1).as_f64().unwrap(),
                    handle.get(2).as_f64().unwrap()
                )
            }
        ).collect::<Vec<(f64, f64, f64)>>();
        let a2 = anchors_and_handles.get(3).dyn_into::<Array>().unwrap().iter().map(
            |anchor| {
                let anchor = anchor.dyn_into::<Array>().unwrap();
                (
                    anchor.get(0).as_f64().unwrap(),
                    anchor.get(1).as_f64().unwrap(),
                    anchor.get(2).as_f64().unwrap()
                )
            }
        ).collect::<Vec<(f64, f64, f64)>>();
        return WasmThreeDObject {
            three_d_object: self.three_d_object.set_anchors_and_handles((a1, h1, h2, a2))
        }
    }
    #[wasm_bindgen(js_name = scaleHandleToAnchorDistances)]
    pub fn scale_handle_to_anchor_distances(&self, factor: f64, recursive: bool) -> WasmThreeDObject {
        return WasmThreeDObject {
            three_d_object: self.three_d_object.scale_handle_to_anchor_distances(factor, recursive)
        }
    } 
    #[wasm_bindgen(js_name = setPoints)]
    pub fn set_points(&self, points: Array) -> WasmThreeDObject {
        let points = points.iter().map(
            |point| {
                let point = point.dyn_into::<Array>().unwrap();
                (
                    point.get(0).as_f64().unwrap(),
                    point.get(1).as_f64().unwrap(),
                    point.get(2).as_f64().unwrap()
                )
            }
        ).collect::<Vec<(f64, f64, f64)>>();
        return WasmThreeDObject {
            three_d_object: self.three_d_object.set_points(points)
        }
    }
    #[wasm_bindgen(js_name = setSubobjects)]
    pub fn set_subobjects(&self, subobjects: Vec<WasmThreeDObject>) -> WasmThreeDObject {
        let subobjects = subobjects.iter().map(
            |face| {
                face.three_d_object.clone()
            }
        ).collect::<Vec<ThreeDObject>>();
        return WasmThreeDObject {
            three_d_object: self.three_d_object.set_subobjects(subobjects)
        }
    }
    #[wasm_bindgen(js_name = setFill)]
    pub fn set_fill(&self, fill: WasmGradientImageOrColor) -> WasmThreeDObject {
        return WasmThreeDObject {
            three_d_object: self.three_d_object.set_fill(fill.gradient_image_or_color)
        }
    }
    #[wasm_bindgen(js_name = setStroke)]
    pub fn set_stroke(&self, stroke: WasmGradientImageOrColor) -> WasmThreeDObject {
        return WasmThreeDObject {
            three_d_object: self.three_d_object.set_stroke(stroke.gradient_image_or_color)
        }
    }
    #[wasm_bindgen(js_name = setStrokeWidth)]
    pub fn set_stroke_width(&self, stroke_width: f64) -> WasmThreeDObject {
        return WasmThreeDObject {
            three_d_object: self.three_d_object.set_stroke_width(stroke_width)
        }
    }
    #[wasm_bindgen(js_name = scale)]
    pub fn scale(&self, factor: f64, recursive: bool) -> WasmThreeDObject {
        return WasmThreeDObject {
            three_d_object: self.three_d_object.scale(factor, recursive)
        }
    }
    #[wasm_bindgen(js_name = stretch)]
    pub fn stretch(&self, factor: Array, recursive: bool) -> WasmThreeDObject {
        let factor = (
            factor.get(0).as_f64().unwrap(),
            factor.get(1).as_f64().unwrap(),
            factor.get(2).as_f64().unwrap()
        );
        return WasmThreeDObject {
            three_d_object: self.three_d_object.stretch(factor, recursive)
        }
    }
    #[wasm_bindgen(js_name = shift)]
    pub fn shift(&self, shift: Array, recursive: bool) -> WasmThreeDObject {
        let shift = (
            shift.get(0).as_f64().unwrap(),
            shift.get(1).as_f64().unwrap(),
            shift.get(2).as_f64().unwrap()
        );
        return WasmThreeDObject {
            three_d_object: self.three_d_object.shift(shift, recursive)
        }
    }
    #[wasm_bindgen(js_name = rotateX)]
    pub fn rotate_x(&self, angle: f64, recursive: bool) -> WasmThreeDObject {
        return WasmThreeDObject {
            three_d_object: self.three_d_object.rotate_x(angle, recursive)
        }
    }
    #[wasm_bindgen(js_name = rotateY)]
    pub fn rotate_y(&self, angle: f64, recursive: bool) -> WasmThreeDObject {
        return WasmThreeDObject {
            three_d_object: self.three_d_object.rotate_y(angle, recursive)
        }
    }
    #[wasm_bindgen(js_name = rotateZ)]
    pub fn rotate_z(&self, angle: f64, recursive: bool) -> WasmThreeDObject {
        return WasmThreeDObject {
            three_d_object: self.three_d_object.rotate_z(angle, recursive)
        }
    }
    #[wasm_bindgen(js_name = projectAndShade)]
    pub fn project_and_shade(&self, camera: &WasmCamera, light_source: &WasmLightSource) -> WasmVectorObject {
        let light_source = &light_source.light_source;
        let camera = &camera.camera;
        return WasmVectorObject {
            native_vec_features: self.three_d_object.project_and_shade(camera, light_source)
        }
    }
    #[wasm_bindgen(js_name = applyFunction)]
    pub fn apply_function(&self, f: Function, recursive: bool) -> WasmThreeDObject {
        return WasmThreeDObject {
            three_d_object: self.three_d_object.apply_function(&|x: f64, y: f64, z: f64| -> (f64, f64, f64) {
                let result = f.call3(&JsValue::NULL, &JsValue::from(x), &JsValue::from(y), &JsValue::from(z)).unwrap();
                let result = result.dyn_into::<Array>().unwrap();
                (
                    result.get(0).as_f64().unwrap(),
                    result.get(1).as_f64().unwrap(),
                    result.get(2).as_f64().unwrap()
                )
            }, recursive)
        }
    }
    #[wasm_bindgen(js_name = fromUvFunction)]
    pub fn from_uv_function(
        uv_function: Function,
        u_range: Array,
        v_range: Array,
        u_segments: usize,
        v_segments: usize,
        fills: Vec<WasmColor>,
        strokes: Vec<WasmColor>,
        stroke_width: f64,
        index: Option<usize>
    ) -> WasmThreeDObject {
        let u_range = (
            u_range.get(0).as_f64().unwrap(),
            u_range.get(1).as_f64().unwrap()
        );
        let v_range = (
            v_range.get(0).as_f64().unwrap(),
            v_range.get(1).as_f64().unwrap()
        );
        let uv_function = move |u: f64, v: f64| -> (f64, f64, f64) {
            let result = uv_function.call2(&JsValue::NULL, &JsValue::from_f64(u), &JsValue::from_f64(v)).unwrap();
            let result = result.dyn_into::<Array>().unwrap();
            (
                result.get(0).as_f64().unwrap(),
                result.get(1).as_f64().unwrap(),
                result.get(2).as_f64().unwrap()
            )
        };
        let fills = fills.iter().map(
            |fill| {
                fill.color.clone()
            }
        ).collect::<Vec<Color>>();
        let strokes = strokes.iter().map(
            |stroke| {
                stroke.color.clone()
            }
        ).collect::<Vec<Color>>();
        return WasmThreeDObject {
            three_d_object: ThreeDObject::from_uv_function(
                &uv_function,
                u_range,
                v_range,
                u_segments,
                v_segments,
                fills,
                strokes,
                stroke_width,
                index
            )
        }
    }
    #[wasm_bindgen(js_name = getBoundingBox)]
    pub fn get_bounding_box(&self) -> Array {
        let bounding_box = self.three_d_object.get_bounding_box();
        let mins = bounding_box.0;
        let maxs = bounding_box.1;
        let min_js = Array::of3(
            &JsValue::from(mins.0),
            &JsValue::from(mins.1),
            &JsValue::from(mins.2)
        );
        let max_js = Array::of3(
            &JsValue::from(maxs.0),
            &JsValue::from(maxs.1),
            &JsValue::from(maxs.2)
        );
        let bounding_box_js = Array::of2(&min_js, &max_js);
        bounding_box_js
    }
    #[wasm_bindgen(js_name = getCenter)]
    pub fn get_center(&self) -> Array {
        let center = self.three_d_object.get_center();
        let center_js = Array::of3(
            &JsValue::from(center.0),
            &JsValue::from(center.1),
            &JsValue::from(center.2)
        );
        center_js
    }
    #[wasm_bindgen(js_name = moveTo)]
    pub fn move_to(&self, point: Array, recursive: bool) -> WasmThreeDObject {
        let point = (
            point.get(0).as_f64().unwrap(),
            point.get(1).as_f64().unwrap(),
            point.get(2).as_f64().unwrap()
        );
        return WasmThreeDObject {
            three_d_object: self.three_d_object.move_to(point, recursive)
        }
    }
    #[wasm_bindgen(js_name = clone)]
    pub fn clone_js(&self) -> WasmThreeDObject {
        self.clone()
    }
    #[wasm_bindgen(js_name = getIndex)]
    pub fn get_index(&self) -> usize {
        self.three_d_object.get_index()
    }
    #[wasm_bindgen(js_name = setIndex)]
    pub fn set_index(&self, index: usize) -> WasmThreeDObject {
        return WasmThreeDObject {
            three_d_object: self.three_d_object.set_index(index)
        }
    }
    #[wasm_bindgen(js_name = fromVectorObject)]
    pub fn from_vector_object(
        vector_object: &WasmVectorObject,
    ) -> WasmThreeDObject {
        return WasmThreeDObject {
            three_d_object: ThreeDObject::from_vector_object(&vector_object.native_vec_features)
        }
    }
    #[wasm_bindgen(js_name = getSubobjectsRecursively)]
    pub fn get_subobjects_recursively(&self) -> Vec<WasmThreeDObject> {
        let subobjects = self.three_d_object.get_subobjects_recursively();
        let subobjects_js = subobjects.iter().map(
            |face| {
                WasmThreeDObject {
                    three_d_object: face.clone()
                }
            }
        ).collect::<Vec<WasmThreeDObject>>();
        subobjects_js
    }
}


#[wasm_bindgen(js_name = threeDAxes)]
pub fn three_d_axes_js(
    x_min: f64,
    x_max: f64,
    x_step: f64,
    y_min: f64,
    y_max: f64,
    y_step: f64,
    z_min: f64,
    z_max: f64,
    z_step: f64,
    center: Array,
    x_length: Option<f64>,
    y_length: Option<f64>,
    z_length: Option<f64>,
    color: Option<WasmColor>,
    stroke_width: Option<f64>,
    add_x_ticks: Option<bool>,
    add_y_ticks: Option<bool>,
    add_z_ticks: Option<bool>,
    x_tick_size: Option<f64>,
    y_tick_size: Option<f64>,
    z_tick_size: Option<f64>,
    add_x_tip: Option<bool>,
    add_y_tip: Option<bool>,
    add_z_tip: Option<bool>,
    n_pieces: Option<usize>,
    index: Option<usize>
) -> WasmThreeDObject {
    return WasmThreeDObject {
        three_d_object: three_d_axes(
            x_min,
            x_max,
            x_step,
            y_min,
            y_max,
            y_step,
            z_min,
            z_max,
            z_step,
            (
                center.get(0).as_f64().unwrap(),
                center.get(1).as_f64().unwrap(),
                center.get(2).as_f64().unwrap()
            ),
            x_length,
            y_length,
            z_length,
            color.map(|color| (color.color.red, color.color.green, color.color.blue, color.color.alpha)),
            stroke_width,
            add_x_ticks,
            add_y_ticks,
            add_z_ticks,
            x_tick_size,
            y_tick_size,
            z_tick_size,
            add_x_tip,
            add_y_tip,
            add_z_tip,
            n_pieces,
            index
        )
    }
}


#[wasm_bindgen(js_name = coordsToPoint3D)]
pub fn coords_to_point_js(
    axes: &WasmThreeDObject,
    coords: Array,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    z_min: f64,
    z_max: f64
) -> Array {
    let coords = (
        coords.get(0).as_f64().unwrap(),
        coords.get(1).as_f64().unwrap(),
        coords.get(2).as_f64().unwrap()
    );
    let point = coords_to_point_3d(&axes.three_d_object, coords, x_min, x_max, y_min, y_max, z_min, z_max);
    let point_js = Array::of3(
        &JsValue::from(point.0),
        &JsValue::from(point.1),
        &JsValue::from(point.2)
    );
    point_js
}


#[wasm_bindgen(js_name = pointToCoords3D)]
pub fn point_to_coords_js(
    axes: &WasmThreeDObject,
    point: Array,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    z_min: f64,
    z_max: f64
) -> Array {
    let point = (
        point.get(0).as_f64().unwrap(),
        point.get(1).as_f64().unwrap(),
        point.get(2).as_f64().unwrap()
    );
    let coords = point_to_coords_3d(&axes.three_d_object, point, x_min, x_max, y_min, y_max, z_min, z_max);
    let coords_js = Array::of3(
        &JsValue::from(coords.0),
        &JsValue::from(coords.1),
        &JsValue::from(coords.2)
    );
    coords_js
}


#[wasm_bindgen(js_name = parametricPlotInAxes3D)]
pub fn parametric_plot_in_axes_js(
    axes: &WasmThreeDObject,
    f: Function,
    u_min: f64,
    u_max: f64,
    v_min: f64,
    v_max: f64,
    u_segments: usize,
    v_segments: usize,
    fills: Vec<WasmColor>,
    strokes: Vec<WasmColor>,
    stroke_width: f64,
    index: Option<usize>
) -> WasmThreeDObject {
    let f = move |u: f64, v: f64| -> (f64, f64, f64) {
        let result = f.call2(&JsValue::NULL, &JsValue::from_f64(u), &JsValue::from_f64(v)).unwrap();
        let result = result.dyn_into::<Array>().unwrap();
        (
            result.get(0).as_f64().unwrap(),
            result.get(1).as_f64().unwrap(),
            result.get(2).as_f64().unwrap()
        )
    };
    let fills = fills.iter().map(
        |fill| {
            fill.color.clone()
        }
    ).collect::<Vec<Color>>();
    let strokes = strokes.iter().map(
        |stroke| {
            stroke.color.clone()
        }
    ).collect::<Vec<Color>>();
    return WasmThreeDObject {
        three_d_object: parametric_plot_in_axes_3d(
            &axes.three_d_object,
            &f,
            u_min,
            u_max,
            v_min,
            v_max,
            u_segments,
            v_segments,
            fills,
            strokes,
            stroke_width,
            index
        )
    }
}


#[wasm_bindgen(js_name = plotInAxes3D)]
pub fn plot_in_axes_3d_js(
    axes: &WasmThreeDObject,
    f: Function,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    u_segments: usize,
    v_segments: usize,
    fills: Vec<WasmColor>,
    strokes: Vec<WasmColor>,
    stroke_width: f64,
    index: Option<usize>
) -> WasmThreeDObject {
    let f = move |x: f64, y: f64| -> f64 {
        f.call2(&JsValue::NULL, &JsValue::from_f64(x), &JsValue::from_f64(y)).unwrap().as_f64().unwrap()
    };
    return WasmThreeDObject {
        three_d_object: plot_in_axes_3d(
            &axes.three_d_object,
            &f,
            x_min,
            x_max,
            y_min,
            y_max,
            u_segments,
            v_segments,
            fills.iter().map(|fill| fill.color.clone()).collect::<Vec<Color>>(),
            strokes.iter().map(|stroke| stroke.color.clone()).collect::<Vec<Color>>(),
            stroke_width,
            index
        )
    }
}


#[wasm_bindgen(js_name = parametricLinePlotInAxes3D)]
pub fn parametric_line_plot_in_axes_3d_js(
    axes: &WasmThreeDObject,
    f: Function,
    u_min: f64,
    u_max: f64,
    u_segments: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    z_min: f64,
    z_max: f64,
    color: WasmColor,
    stroke_width: f64,
    index: Option<usize>
) -> WasmThreeDObject {
    let f = move |u: f64| -> (f64, f64, f64) {
        let result = f.call1(&JsValue::NULL, &JsValue::from_f64(u)).unwrap();
        let result = result.dyn_into::<Array>().unwrap();
        (
            result.get(0).as_f64().unwrap(),
            result.get(1).as_f64().unwrap(),
            result.get(2).as_f64().unwrap()
        )
    };
    return WasmThreeDObject {
        three_d_object: parametric_line_plot_in_axes_3d(
            &axes.three_d_object,
            &f,
            u_min,
            u_max,
            u_segments,
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
            color.color,
            stroke_width,
            index
        )
    }
}


#[wasm_bindgen(js_name = sphere)]
pub fn sphere_js(
    center: Array,
    radius: f64,
    u_segments: usize,
    v_segments: usize,
    fill_colors: Vec<WasmColor>,
    stroke_colors: Vec<WasmColor>,
    stroke_width: f64,
    index: Option<usize>
) -> WasmThreeDObject {
    return WasmThreeDObject {
        three_d_object: sphere(
            (
                center.get(0).as_f64().unwrap(),
                center.get(1).as_f64().unwrap(),
                center.get(2).as_f64().unwrap()
            ),
            radius,
            u_segments,
            v_segments,
            fill_colors.iter().map(|fill| fill.color.clone()).collect::<Vec<Color>>(),
            stroke_colors.iter().map(|stroke| stroke.color.clone()).collect::<Vec<Color>>(),
            stroke_width,
            index
        )
    }
}