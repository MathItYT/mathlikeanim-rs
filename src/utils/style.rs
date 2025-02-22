use std::rc::Rc;

use usvg::{Opacity, Paint};
use wasm_bindgen::prelude::*;

use crate::utils::interpolation::{inverse_lerp, lerp};

use super::{console::log, point2d::Point2D};

/// A color with red, green, blue, and alpha components.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct Color {
    /// The red component of the color.
    pub red: u8,
    /// The green component of the color.
    pub green: u8,
    /// The blue component of the color.
    pub blue: u8,
    /// The alpha component of the color.
    pub alpha: f32,
}

impl Default for Color {
    fn default() -> Self {
        Color {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 0.0,
        }
    }
}

#[wasm_bindgen]
impl Color {
    /// Creates a new @type {Color} with the given red, green, blue, and alpha components.
    #[wasm_bindgen(constructor, return_description = "A new color.")]
    pub fn new(
        #[wasm_bindgen(param_description = "The red component of the color.")]
        red: u8,
        #[wasm_bindgen(param_description = "The green component of the color.")]
        green: u8,
        #[wasm_bindgen(param_description = "The blue component of the color.")]
        blue: u8,
        #[wasm_bindgen(param_description = "The alpha component of the color.")]
        alpha: f32
    ) -> Color {
        Color {
            red,
            green,
            blue,
            alpha,
        }
    }
    /// Returns the default color, which is a fully transparent black.
    #[wasm_bindgen(return_description = "The default color.")]
    pub fn default_color() -> Color {
        Color::default()
    }
    /// Linearly interpolates between two @type {Color}s given a progress value.
    #[wasm_bindgen]
    pub fn lerp(
        #[wasm_bindgen(param_description = "The start color.")]
        color1: &Color,
        #[wasm_bindgen(param_description = "The end color.")]
        color2: &Color,
        #[wasm_bindgen(param_description = "The progress value.")]
        t: f32
    ) -> Color {
        let red = lerp(color1.red as f32, color2.red as f32, t) as u8;
        let green = lerp(color1.green as f32, color2.green as f32, t) as u8;
        let blue = lerp(color1.blue as f32, color2.blue as f32, t) as u8;
        let alpha = lerp(color1.alpha, color2.alpha, t);
        Color {
            red,
            green,
            blue,
            alpha,
        }
    }
}

/// A color stop for a gradient with a color and a position.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct ColorStop {
    /// The color of the stop.
    pub color: Color,
    /// The position of the stop.
    pub position: f32,
}

impl Default for ColorStop {
    fn default() -> Self {
        ColorStop {
            color: Color::default(),
            position: 0.0,
        }
    }
}

#[wasm_bindgen]
impl ColorStop {
    /// Creates a new @type {ColorStop} with the given color and position.
    #[wasm_bindgen(constructor, return_description = "A new color stop.")]
    pub fn new(
        #[wasm_bindgen(param_description = "The color of the stop.")]
        color: Color,
        #[wasm_bindgen(param_description = "The position of the stop.")]
        position: f32
    ) -> ColorStop {
        ColorStop {
            color,
            position,
        }
    }
}

/// A linear gradient with a start and end point and color stops.
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct LinearGradient {
    /// The start point of the gradient.
    pub p1: Point2D,
    /// The end point of the gradient.
    pub p2: Point2D,
    /// The color stops of the gradient.
    color_stops: Rc<Vec<ColorStop>>,
}

impl Default for LinearGradient {
    fn default() -> Self {
        LinearGradient {
            p1: Point2D::default(),
            p2: Point2D::default(),
            color_stops: Rc::new(vec![]),
        }
    }
}

#[wasm_bindgen]
impl LinearGradient {
    /// Creates a new @type {LinearGradient} with the given start point, end point, and @type {ColorStop}s.
    #[wasm_bindgen(constructor, return_description = "A new linear gradient.")]
    pub fn new(
        #[wasm_bindgen(param_description = "The start point of the gradient.")]
        p1: Point2D,
        #[wasm_bindgen(param_description = "The end point of the gradient.")]
        p2: Point2D,
        #[wasm_bindgen(param_description = "The color stops of the gradient.")]
        color_stops: Vec<ColorStop>
    ) -> LinearGradient {
        LinearGradient {
            p1,
            p2,
            color_stops: Rc::new(color_stops),
        }
    }
    /// Returns the default @type {LinearGradient}, which is a gradient from the start to the end the same color.
    #[wasm_bindgen(return_description = "The default linear gradient.")]
    pub fn single_color_gradient(
        #[wasm_bindgen(param_description = "The start point of the gradient.")]
        p1: Point2D,
        #[wasm_bindgen(param_description = "The end point of the gradient.")]
        p2: Point2D,
        #[wasm_bindgen(param_description = "The color of the gradient.")]
        color: Color
    ) -> LinearGradient {
        LinearGradient {
            p1,
            p2,
            color_stops: Rc::new(vec![ColorStop { color, position: 0.0 }, ColorStop { color, position: 1.0 }]),
        }
    }
    /// Returns the default @type {LinearGradient}, which is a gradient from the origin to the origin with no @type {ColorStop}s.
    #[wasm_bindgen(return_description = "The default linear gradient.")]
    pub fn default_linear_gradient() -> LinearGradient {
        LinearGradient::default()
    }
    /// Gets the @type {ColorStop}s of the gradient.
    #[wasm_bindgen(getter, return_description = "The color stops of the gradient.")]
    pub fn color_stops(&self) -> Vec<ColorStop> {
        self.color_stops.to_vec()
    }
    /// Sets the @type {ColorStop}s of the gradient.
    #[wasm_bindgen(setter)]
    pub fn set_color_stops(
        &mut self,
        #[wasm_bindgen(param_description = "The color stops of the gradient.")]
        color_stops: Vec<ColorStop>
    ) {
        self.color_stops = Rc::new(color_stops);
    }
    /// Gets the @type {Color} at a given offset along the gradient.
    #[wasm_bindgen(return_description = "The color at the given offset.")]
    pub fn color_at_offset(
        &self,
        #[wasm_bindgen(param_description = "The offset to get the color at.")]
        position: f32
    ) -> Color {
        let mut the_stops = Rc::clone(&self.color_stops);
        let stops = Rc::make_mut(&mut the_stops);
        stops.sort_by(|a, b| a.position.partial_cmp(&b.position).unwrap());
        for i in 0..stops.len() {
            if position < stops[i].position {
                if i == 0 {
                    return stops[i].color;
                }
                let t = inverse_lerp(stops[i - 1].position, stops[i].position, position);
                return Color::lerp(&stops[i - 1].color, &stops[i].color, t);
            }
        }
        stops[stops.len() - 1].color
    }
    /// Gets the @type {Color} at a given point along the gradient.
    #[wasm_bindgen(return_description = "The color at the given point.")]
    pub fn color_at(
        &self,
        #[wasm_bindgen(param_description = "The point to get the color at.")]
        p: Point2D
    ) -> Color {
        let dp = self.p2 - self.p1;
        let dt = dp.normalized();
        let normal = Point2D::new(-dt.y, dt.x);
        let d = (p.x - self.p1.x) * normal.x + (p.y - self.p1.y) * normal.y;
        let t = d / dp.magnitude();
        self.color_at_offset(t)
    }
    /// Linearly interpolates between two @type {LinearGradient}s given a progress value.
    #[wasm_bindgen(return_description = "The interpolated linear gradient.")]
    pub fn lerp(
        #[wasm_bindgen(param_description = "The first linear gradient.")]
        gradient1: &LinearGradient,
        #[wasm_bindgen(param_description = "The second linear gradient.")]
        gradient2: &LinearGradient,
        #[wasm_bindgen(param_description = "The progress value.")]
        t: f32
    ) -> LinearGradient {
        let p1 = Point2D::lerp(&gradient1.p1, &gradient2.p1, t);
        let p2 = Point2D::lerp(&gradient1.p2, &gradient2.p2, t);
        let length = gradient1.color_stops.len().max(gradient2.color_stops.len());
        let mut color_stops = vec![];
        for i in 0..length {
            let color1 = if i < gradient1.color_stops.len() {
                gradient1.color_stops[i].color
            } else {
                Color::default()
            };
            let color2 = if i < gradient2.color_stops.len() {
                gradient2.color_stops[i].color
            } else {
                Color::default()
            };
            let color = Color::lerp(&color1, &color2, t);
            let position = if i < gradient1.color_stops.len() {
                lerp(gradient1.color_stops[i].position, gradient2.color_stops[i].position, t)
            } else {
                lerp(0.0, gradient2.color_stops[i].position, t)
            };
            color_stops.push(ColorStop { color, position });
        }
        LinearGradient {
            p1,
            p2,
            color_stops: Rc::new(color_stops),
        }
    }
}

/// A radial gradient with two circles and color stops.
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct RadialGradient {
    /// The focal point of the gradient.
    pub f: Point2D,
    /// The center point of the gradient.
    pub c: Point2D,
    /// The radius of the gradient.
    pub r: f32,
    /// The color stops of the gradient.
    color_stops: Rc<Vec<ColorStop>>,
}

impl Default for RadialGradient {
    fn default() -> Self {
        RadialGradient {
            f: Point2D::default(),
            c: Point2D::default(),
            r: 0.0,
            color_stops: Rc::new(vec![]),
        }
    }
}

#[wasm_bindgen]
impl RadialGradient {
    /// Creates a new @type {RadialGradient} with the given focal point, center point, radius, and @type {ColorStop}s.
    #[wasm_bindgen(constructor, return_description = "A new radial gradient.")]
    pub fn new(
        #[wasm_bindgen(param_description = "The focal point of the gradient.")]
        f: Point2D,
        #[wasm_bindgen(param_description = "The center of the gradient.")]
        c: Point2D,
        #[wasm_bindgen(param_description = "The radius of the gradient.")]
        r: f32,
        #[wasm_bindgen(param_description = "The color stops of the gradient.")]
        color_stops: Vec<ColorStop>
    ) -> RadialGradient {
        RadialGradient {
            f,
            c,
            r,
            color_stops: Rc::new(color_stops),
        }
    }
    /// Returns the default @type {RadialGradient}, which is a gradient from the focal point to the center with the same color.
    #[wasm_bindgen(return_description = "The default radial gradient.")]
    pub fn default_radial_gradient() -> RadialGradient {
        RadialGradient::default()
    }
    /// Gets the @type {ColorStop}s of the gradient.
    #[wasm_bindgen(getter, return_description = "The color stops of the gradient.")]
    pub fn color_stops(&self) -> Vec<ColorStop> {
        self.color_stops.to_vec()
    }
    /// Sets the @type {ColorStop}s of the gradient.
    #[wasm_bindgen(setter)]
    pub fn set_color_stops(
        &mut self,
        #[wasm_bindgen(param_description = "The color stops of the gradient.")]
        color_stops: Vec<ColorStop>
    ) {
        self.color_stops = Rc::new(color_stops);
    }
    /// Gets the @type {Color} at a given offset along the gradient.
    #[wasm_bindgen(return_description = "The color at the given offset.")]
    pub fn color_at_offset(
        &self,
        #[wasm_bindgen(param_description = "The offset to get the color at.")]
        position: f32
    ) -> Color {
        let mut stops = Rc::clone(&self.color_stops);
        let stops = Rc::make_mut(&mut stops);
        stops.sort_by(|a, b| a.position.partial_cmp(&b.position).unwrap());
        for i in 0..stops.len() {
            if position < stops[i].position {
                if i == 0 {
                    return stops[i].color;
                }
                let t = inverse_lerp(stops[i - 1].position, stops[i].position, position);
                return Color::lerp(&stops[i - 1].color, &stops[i].color, t);
            }
        }
        stops[stops.len() - 1].color
    }
    /// Gets the @type {Color} at a given point along the gradient.
    #[wasm_bindgen(return_description = "The color at the given point.")]
    pub fn color_at(
        &self,
        #[wasm_bindgen(param_description = "The point to get the color at.")]
        p: Point2D
    ) -> Color {
        let dp = self.c - self.f;
        let dt = dp.normalized();
        let normal = Point2D::new(-dt.y, dt.x);
        let d = (p.x - self.f.x) * normal.x + (p.y - self.f.y) * normal.y;
        let t = d / dp.magnitude();
        self.color_at_offset(t)
    }
    /// Returns a single color radial gradient.
    #[wasm_bindgen(return_description = "A single color radial gradient.")]
    pub fn single_color_gradient(
        #[wasm_bindgen(param_description = "The focal point of the gradient.")]
        f: Point2D,
        #[wasm_bindgen(param_description = "The center of the gradient.")]
        c: Point2D,
        #[wasm_bindgen(param_description = "The radius of the gradient.")]
        r: f32,
        #[wasm_bindgen(param_description = "The color of the gradient.")]
        color: Color
    ) -> RadialGradient {
        RadialGradient {
            f,
            c,
            r,
            color_stops: Rc::new(vec![ColorStop { color, position: 0.0 }, ColorStop { color, position: 1.0 }]),
        }
    }
    /// Linearly interpolates between two @type {RadialGradient}s given a progress value.
    #[wasm_bindgen(return_description = "The interpolated radial gradient.")]
    pub fn lerp(
        #[wasm_bindgen(param_description = "The first radial gradient.")]
        gradient1: &RadialGradient,
        #[wasm_bindgen(param_description = "The second radial gradient.")]
        gradient2: &RadialGradient,
        #[wasm_bindgen(param_description = "The progress value.")]
        t: f32
    ) -> RadialGradient {
        let f = Point2D::lerp(&gradient1.f, &gradient2.f, t);
        let c = Point2D::lerp(&gradient1.c, &gradient2.c, t);
        let r = lerp(gradient1.r, gradient2.r, t);
        let length = gradient1.color_stops.len().max(gradient2.color_stops.len());
        let mut color_stops = vec![];
        for i in 0..length {
            let color1 = if i < gradient1.color_stops.len() {
                gradient1.color_stops[i].color
            } else {
                Color::default()
            };
            let color2 = if i < gradient2.color_stops.len() {
                gradient2.color_stops[i].color
            } else {
                Color::default()
            };
            let color = Color::lerp(&color1, &color2, t);
            let position = if i < gradient1.color_stops.len() {
                lerp(gradient1.color_stops[i].position, gradient2.color_stops[i].position, t)
            } else {
                lerp(0.0, gradient2.color_stops[i].position, t)
            };
            color_stops.push(ColorStop { color, position });
        }
        RadialGradient {
            f,
            c,
            r,
            color_stops: Rc::new(color_stops),
        }
    }
}

/// An image bitmap with pixel data.
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct ImageBitmap {
    /// The x coordinate of the bitmap.
    pub x: f32,
    /// The y coordinate of the bitmap.
    pub y: f32,
    /// The width of the bitmap.
    pub width: f32,
    /// The height of the bitmap.
    pub height: f32,
    /// The pixel data of the bitmap.
    data: Rc<Vec<u8>>,
}

impl Default for ImageBitmap {
    fn default() -> Self {
        ImageBitmap {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            data: Rc::new(vec![]),
        }
    }
}

#[wasm_bindgen]
impl ImageBitmap {
    /// Creates a new @type {ImageBitmap} with the given x, y, width, height, and pixel data.
    #[wasm_bindgen(constructor, return_description = "A new image bitmap.")]
    pub fn new(
        #[wasm_bindgen(param_description = "The x coordinate of the bitmap.")]
        x: f32,
        #[wasm_bindgen(param_description = "The y coordinate of the bitmap.")]
        y: f32,
        #[wasm_bindgen(param_description = "The width of the bitmap.")]
        width: f32,
        #[wasm_bindgen(param_description = "The height of the bitmap.")]
        height: f32,
        #[wasm_bindgen(param_description = "The pixel data of the bitmap.")]
        data: Vec<u8>
    ) -> ImageBitmap {
        ImageBitmap {
            x,
            y,
            width,
            height,
            data: Rc::new(data),
        }
    }
    /// Gets the pixel data of the bitmap.
    #[wasm_bindgen(getter, return_description = "The pixel data of the bitmap.")]
    pub fn data(&self) -> Vec<u8> {
        self.data.to_vec()
    }
    /// Sets the pixel data of the bitmap.
    #[wasm_bindgen(setter)]
    pub fn set_data(
        &mut self,
        #[wasm_bindgen(param_description = "The pixel data of the bitmap.")]
        data: Vec<u8>
    ) {
        self.data = Rc::new(data);
    }
    /// Returns the default @type {ImageBitmap}, which is an empty bitmap.
    #[wasm_bindgen(return_description = "The default image bitmap.")]
    pub fn default_image_bitmap() -> ImageBitmap {
        ImageBitmap::default()
    }
    /// Gets the color of a @type {Point2D} in the bitmap.
    #[wasm_bindgen(return_description = "The color of the pixel.")]
    pub fn get_pixel(
        &self,
        #[wasm_bindgen(param_description = "The point to get the pixel color at.")]
        p: Point2D
    ) -> Color {
        let x = p.x as usize;
        let y = p.y as usize;
        let index = ((y - self.y as usize) * self.width as usize + (x - self.x as usize)) * 4;
        Color {
            red: self.data[index],
            green: self.data[index + 1],
            blue: self.data[index + 2],
            alpha: self.data[index + 3] as f32 / 255.0,
        }
    }
    /// Sets a pixel color at a @type {Point2D} in the bitmap.
    pub fn set_pixel(
        &mut self,
        #[wasm_bindgen(param_description = "The point to set the pixel color at.")]
        p: Point2D,
        #[wasm_bindgen(param_description = "The color of the pixel.")]
        color: &Color
    ) {
        let x = p.x as usize;
        let y = p.y as usize;
        let index = ((y - self.y as usize) * self.width as usize + (x - self.x as usize)) * 4;
        let data = Rc::make_mut(&mut self.data);
        data[index] = color.red;
        data[index + 1] = color.green;
        data[index + 2] = color.blue;
        data[index + 3] = (color.alpha * 255.0) as u8;
    }
    /// Gets a bitmap that is filled with a color.
    #[wasm_bindgen(return_description = "The filled image bitmap.")]
    pub fn fill(
        #[wasm_bindgen(param_description = "The x coordinate of the bitmap.")]
        x: f32,
        #[wasm_bindgen(param_description = "The y coordinate of the bitmap.")]
        y: f32,
        #[wasm_bindgen(param_description = "The width of the bitmap.")]
        width: f32,
        #[wasm_bindgen(param_description = "The height of the bitmap.")]
        height: f32,
        #[wasm_bindgen(param_description = "The color to fill the bitmap with.")]
        color: &Color
    ) -> ImageBitmap {
        let length = (width * height * 4.0) as usize;
        let mut data = vec![0; length];
        for i in 0..length {
            data[i] = match i % 4 {
                0 => color.red,
                1 => color.green,
                2 => color.blue,
                3 => (color.alpha * 255.0) as u8,
                _ => 0,
            };
        }
        ImageBitmap {
            x,
            y,
            width,
            height,
            data: Rc::new(data),
        }
    }
    /// Gets a bitmap that is filled with a linear gradient.
    #[wasm_bindgen(return_description = "The filled image bitmap.")]
    pub fn fill_linear_gradient(
        #[wasm_bindgen(param_description = "The x coordinate of the bitmap.")]
        x: f32,
        #[wasm_bindgen(param_description = "The y coordinate of the bitmap.")]
        y: f32,
        #[wasm_bindgen(param_description = "The width of the bitmap.")]
        width: f32,
        #[wasm_bindgen(param_description = "The height of the bitmap.")]
        height: f32,
        #[wasm_bindgen(param_description = "The linear gradient to fill the bitmap with.")]
        gradient: &LinearGradient
    ) -> ImageBitmap {
        let length = (width * height * 4.0) as usize;
        let mut data = vec![0; length];
        for i in 0..length {
            let x = i % (width as usize);
            let y = i / (width as usize);
            let color = gradient.color_at(Point2D::new(x as f32, y as f32));
            data[i] = match i % 4 {
                0 => color.red,
                1 => color.green,
                2 => color.blue,
                3 => (color.alpha * 255.0) as u8,
                _ => 0,
            };
        }
        ImageBitmap {
            x,
            y,
            width,
            height,
            data: Rc::new(data),
        }
    }
    /// Gets a bitmap that is filled with a radial gradient.
    #[wasm_bindgen(return_description = "The filled image bitmap.")]
    pub fn fill_radial_gradient(
        #[wasm_bindgen(param_description = "The x coordinate of the bitmap.")]
        x: f32,
        #[wasm_bindgen(param_description = "The y coordinate of the bitmap.")]
        y: f32,
        #[wasm_bindgen(param_description = "The width of the bitmap.")]
        width: f32,
        #[wasm_bindgen(param_description = "The height of the bitmap.")]
        height: f32,
        #[wasm_bindgen(param_description = "The radial gradient to fill the bitmap with.")]
        gradient: &RadialGradient
    ) -> ImageBitmap {
        let length = (width * height * 4.0) as usize;
        let mut data = vec![0; length];
        for i in 0..length {
            let x = i % (width as usize);
            let y = i / (width as usize);
            let color = gradient.color_at(Point2D::new(x as f32, y as f32));
            data[i] = match i % 4 {
                0 => color.red,
                1 => color.green,
                2 => color.blue,
                3 => (color.alpha * 255.0) as u8,
                _ => 0,
            };
        }
        ImageBitmap {
            x,
            y,
            width,
            height,
            data: Rc::new(data),
        }
    }
    /// Linearly interpolates between two @type {ImageBitmap}s given a progress value.
    #[wasm_bindgen(return_description = "The interpolated image bitmap.")]
    pub fn lerp(
        #[wasm_bindgen(param_description = "The first image bitmap.")]
        bitmap1: &ImageBitmap,
        #[wasm_bindgen(param_description = "The second image bitmap.")]
        bitmap2: &ImageBitmap,
        #[wasm_bindgen(param_description = "The progress value.")]
        t: f32
    ) -> ImageBitmap {
        let x = lerp(bitmap1.x, bitmap2.x, t);
        let y = lerp(bitmap1.y, bitmap2.y, t);
        let width = lerp(bitmap1.width, bitmap2.width, t);
        let height = lerp(bitmap1.height, bitmap2.height, t);
        let length = bitmap1.data.len().max(bitmap2.data.len());
        let mut data = vec![0; length];
        for i in 0..length {
            let color1 = if i < bitmap1.data.len() {
                Color {
                    red: bitmap1.data[i],
                    green: bitmap1.data[i + 1],
                    blue: bitmap1.data[i + 2],
                    alpha: bitmap1.data[i + 3] as f32 / 255.0,
                }
            } else {
                Color::default()
            };
            let color2 = if i < bitmap2.data.len() {
                Color {
                    red: bitmap2.data[i],
                    green: bitmap2.data[i + 1],
                    blue: bitmap2.data[i + 2],
                    alpha: bitmap2.data[i + 3] as f32 / 255.0,
                }
            } else {
                Color::default()
            };
            let color = Color::lerp(&color1, &color2, t);
            data[i] = color.red;
            data[i + 1] = color.green;
            data[i + 2] = color.blue;
            data[i + 3] = (color.alpha * 255.0) as u8;
        }
        ImageBitmap {
            x,
            y,
            width,
            height,
            data: Rc::new(data),
        }
    }
}

/// A style with a color, linear gradient, radial gradient, or image.
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Style {
    /// The color of the style.
    color: Option<Color>,
    /// The linear gradient of the style.
    linear_gradient: Option<LinearGradient>,
    /// The radial gradient of the style.
    radial_gradient: Option<RadialGradient>,
    /// The image of the style.
    image: Option<ImageBitmap>,
}

impl Default for Style {
    fn default() -> Self {
        Style {
            color: Color::default().into(),
            linear_gradient: None,
            radial_gradient: None,
            image: None,
        }
    }
}

#[wasm_bindgen]
impl Style {
    /// Creates a new @type {Style} with the given color, linear gradient, radial gradient, or image. It must have exactly one of these.
    #[wasm_bindgen(constructor, return_description = "A new style.")]
    pub fn new(
        #[wasm_bindgen(param_description = "The color of the style, if provided.")]
        color: Option<Color>,
        #[wasm_bindgen(param_description = "The linear gradient of the style, if provided.")]
        linear_gradient: Option<LinearGradient>,
        #[wasm_bindgen(param_description = "The radial gradient of the style, if provided.")]
        radial_gradient: Option<RadialGradient>,
        #[wasm_bindgen(param_description = "The image of the style, if provided.")]
        image: Option<ImageBitmap>
    ) -> Result<Style, JsError> {
        let mut not_none = 0;
        if color.is_some() {
            not_none += 1;
        }
        if linear_gradient.is_some() {
            not_none += 1;
        }
        if radial_gradient.is_some() {
            not_none += 1;
        }
        if image.is_some() {
            not_none += 1;
        }
        if not_none != 1 {
            let err = JsError::new("Exactly one of color, linear_gradient, radial_gradient, or image must be provided.");
            return Err(err);
        }
        Ok(Style {
            color,
            linear_gradient,
            radial_gradient,
            image,
        })
    }
    /// Creates a new @type {Style} with the given color.
    #[wasm_bindgen(return_description = "A new style from the color.")]
    pub fn from_color(
        #[wasm_bindgen(param_description = "The color of the style.")]
        color: Color
    ) -> Style {
        Style::new(Some(color), None, None, None).unwrap()
    }
    /// Creates a new @type {Style} with the given linear gradient.
    #[wasm_bindgen(return_description = "A new style from the linear gradient.")]
    pub fn from_linear_gradient(
        #[wasm_bindgen(param_description = "The linear gradient of the style.")]
        gradient: LinearGradient
    ) -> Style {
        Style::new(None, Some(gradient), None, None).unwrap()
    }
    /// Creates a new @type {Style} with the given radial gradient.
    #[wasm_bindgen(return_description = "A new style from the radial gradient.")]
    pub fn from_radial_gradient(gradient: RadialGradient) -> Style {
        Style::new(None, None, Some(gradient), None).unwrap()
    }
    /// Creates a new @type {Style} with the given image.
    #[wasm_bindgen(return_description = "A new style from the image.")]
    pub fn from_image(
        #[wasm_bindgen(param_description = "The image of the style.")]
        image: ImageBitmap
    ) -> Style {
        Style::new(None, None, None, Some(image)).unwrap()
    }
    /// Returns the default @type {Style}, which is a color with the default color.
    #[wasm_bindgen(return_description = "The default style.")]
    pub fn default_style() -> Style {
        Style::default()
    }
    /// Fades the @type {Style} by a given amount.
    #[wasm_bindgen(return_description = "The faded style.")]
    pub fn fade(
        &self,
        #[wasm_bindgen(param_description = "The amount to fade the style by.")]
        amount: f32
    ) -> Style {
        let mut color = self.color.clone();
        let mut linear_gradient = self.linear_gradient.clone();
        let mut radial_gradient = self.radial_gradient.clone();
        let mut image = self.image.clone();
        if let Some(color) = &mut color {
            color.alpha = color.alpha * (1.0 - amount);
        }
        if let Some(linear_gradient) = &mut linear_gradient {
            for stop in Rc::make_mut(&mut linear_gradient.color_stops) {
                stop.color.alpha = stop.color.alpha * (1.0 - amount);
            }
        }
        if let Some(radial_gradient) = &mut radial_gradient {
            for stop in Rc::make_mut(&mut radial_gradient.color_stops) {
                stop.color.alpha = stop.color.alpha * (1.0 - amount);
            }
        }
        if let Some(image) = &mut image {
            for i in 0..image.data.len() {
                if i % 4 == 3 {
                    Rc::make_mut(&mut image.data)[i] = (image.data[i] as f32 * (1.0 - amount)) as u8;
                }
            }
        }
        Style {
            color,
            linear_gradient,
            radial_gradient,
            image,
        }
    }
    /// Gets the @type {Color} of the style, if it's a color.
    #[wasm_bindgen(getter, return_description = "The color of the style.")]
    pub fn color(&self) -> Option<Color> {
        self.color.clone()
    }
    /// Sets the style to a @type {Color}.
    #[wasm_bindgen(setter)]
    pub fn set_color(
        &mut self,
        #[wasm_bindgen(param_description = "The color of the style.")]
        color: Color
    ) {
        self.color = Some(color);
        self.linear_gradient = None;
        self.radial_gradient = None;
        self.image = None;
    }
    /// Gets the @type {LinearGradient} of the style, if it's a linear gradient.
    #[wasm_bindgen(getter, return_description = "The linear gradient of the style.")]
    pub fn linear_gradient(&self) -> Option<LinearGradient> {
        self.linear_gradient.clone()
    }
    /// Sets the style to a @type {LinearGradient}.
    #[wasm_bindgen(setter)]
    pub fn set_linear_gradient(
        &mut self,
        #[wasm_bindgen(param_description = "The linear gradient of the style.")]
        linear_gradient: LinearGradient
    ) {
        self.color = None;
        self.linear_gradient = Some(linear_gradient);
        self.radial_gradient = None;
        self.image = None;
    }
    /// Gets the @type {RadialGradient} of the style, if it's a radial gradient.
    #[wasm_bindgen(getter, return_description = "The radial gradient of the style.")]
    pub fn radial_gradient(&self) -> Option<RadialGradient> {
        self.radial_gradient.clone()
    }
    /// Sets the style to a @type {RadialGradient}.
    #[wasm_bindgen(setter)]
    pub fn set_radial_gradient(
        &mut self,
        #[wasm_bindgen(param_description = "The radial gradient of the style.")]
        radial_gradient: RadialGradient
    ) {
        self.color = None;
        self.linear_gradient = None;
        self.radial_gradient = Some(radial_gradient);
        self.image = None;
    }
    /// Gets the @type {ImageBitmap} of the style, if it's an image.
    #[wasm_bindgen(getter, return_description = "The image of the style.")]
    pub fn image(&self) -> Option<ImageBitmap> {
        self.image.clone()
    }
    /// Sets the style to an @type {ImageBitmap}.
    #[wasm_bindgen(setter)]
    pub fn set_image(&mut self, image: ImageBitmap) {
        self.color = None;
        self.linear_gradient = None;
        self.radial_gradient = None;
        self.image = Some(image);
    }
    /// Gets the @type {Color} at a given point.
    #[wasm_bindgen(return_description = "The color at the given point.")]
    pub fn color_at(
        &self,
        #[wasm_bindgen(param_description = "The x coordinate of the point.")]
        p: Point2D
    ) -> Color {
        if let Some(color) = self.color() {
            return color;
        }
        if let Some(linear_gradient) = self.linear_gradient() {
            return linear_gradient.color_at(p);
        }
        if let Some(radial_gradient) = self.radial_gradient() {
            return radial_gradient.color_at(p);
        }
        if let Some(image) = self.image() {
            return image.get_pixel(p);
        }
        Color::default()
    }
    /// Linearly interpolates between two @type {Style}s given a progress value.
    #[wasm_bindgen(return_description = "The interpolated style.")]
    pub fn lerp(
        #[wasm_bindgen(param_description = "The first style.")]
        style1: &Style,
        #[wasm_bindgen(param_description = "The second style.")]
        style2: &Style,
        #[wasm_bindgen(param_description = "The progress value.")]
        t: f32
    ) -> Style {
        let color1 = style1.color();
        let color2 = style2.color();
        let linear_gradient1 = style1.linear_gradient();
        let linear_gradient2 = style2.linear_gradient();
        let radial_gradient1 = style1.radial_gradient();
        let radial_gradient2 = style2.radial_gradient();
        let image1 = style1.image();
        let image2 = style2.image();
        if color1.is_some() && color2.is_some() {
            let color = Color::lerp(&color1.unwrap(), &color2.unwrap(), t);
            return Style::from_color(color);
        }
        if linear_gradient1.is_some() && linear_gradient2.is_some() {
            let linear_gradient = LinearGradient::lerp(&linear_gradient1.unwrap(), &linear_gradient2.unwrap(), t);
            return Style::from_linear_gradient(linear_gradient);
        }
        if radial_gradient1.is_some() && radial_gradient2.is_some() {
            let radial_gradient = RadialGradient::lerp(&radial_gradient1.unwrap(), &radial_gradient2.unwrap(), t);
            return Style::from_radial_gradient(radial_gradient);
        }
        if image1.is_some() && image2.is_some() {
            let image = ImageBitmap::lerp(&image1.unwrap(), &image2.unwrap(), t);
            return Style::from_image(image);
        }
        if color1.is_some() {
            let color = color1.unwrap();
            if linear_gradient2.is_some() {
                let linear_gradient2 = linear_gradient2.unwrap();
                let linear_gradient1 = LinearGradient::single_color_gradient(linear_gradient2.p1, linear_gradient2.p2, color);
                return Style::lerp(&Style::from_linear_gradient(linear_gradient1), &Style::from_linear_gradient(linear_gradient2), t);
            }
            if radial_gradient2.is_some() {
                let radial_gradient2 = radial_gradient2.unwrap();
                let radial_gradient1 = RadialGradient::single_color_gradient(radial_gradient2.f, radial_gradient2.c, radial_gradient2.r, color);
                return Style::lerp(&Style::from_radial_gradient(radial_gradient1), &Style::from_radial_gradient(radial_gradient2), t);
            }
            if image2.is_some() {
                let image2 = image2.unwrap();
                let image1 = ImageBitmap::fill(image2.x, image2.y, image2.width, image2.height, &color);
                return Style::lerp(&Style::from_image(image1), &Style::from_image(image2), t);
            }
        }
        if linear_gradient1.is_some() {
            let linear_gradient1 = linear_gradient1.unwrap();
            if color2.is_some() {
                let color2 = color2.unwrap();
                let linear_gradient2 = LinearGradient::single_color_gradient(linear_gradient1.p1, linear_gradient1.p2, color2);
                return Style::lerp(&Style::from_linear_gradient(linear_gradient1), &Style::from_linear_gradient(linear_gradient2), t);
            }
            if radial_gradient2.is_some() {
                let radial_gradient2 = radial_gradient2.unwrap();
                let image2 = ImageBitmap::fill_radial_gradient(radial_gradient2.c.x, radial_gradient2.c.y, radial_gradient2.r * 2.0, radial_gradient2.r * 2.0, &radial_gradient2);
                let image1 = ImageBitmap::fill_linear_gradient(linear_gradient1.p1.x, linear_gradient1.p1.y, linear_gradient1.p2.x, linear_gradient1.p2.y, &linear_gradient1);
                return Style::lerp(&Style::from_image(image1), &Style::from_image(image2), t);
            }
            if image2.is_some() {
                let image2 = image2.unwrap();
                let image1 = ImageBitmap::fill_linear_gradient(image2.x, image2.y, image2.width, image2.height, &linear_gradient1);
                return Style::lerp(&Style::from_image(image1), &Style::from_image(image2), t);
            }
        }
        if radial_gradient1.is_some() {
            let radial_gradient1 = radial_gradient1.unwrap();
            if color2.is_some() {
                let color2 = color2.unwrap();
                let radial_gradient2 = RadialGradient::single_color_gradient(radial_gradient1.f, radial_gradient1.c, radial_gradient1.r, color2);
                return Style::lerp(&Style::from_radial_gradient(radial_gradient1), &Style::from_radial_gradient(radial_gradient2), t);
            }
            if linear_gradient2.is_some() {
                let linear_gradient2 = linear_gradient2.unwrap();
                let image2 = ImageBitmap::fill_linear_gradient(linear_gradient2.p1.x, linear_gradient2.p1.y, linear_gradient2.p2.x, linear_gradient2.p2.y, &linear_gradient2);
                let image1 = ImageBitmap::fill_radial_gradient(radial_gradient1.c.x, radial_gradient1.c.y, radial_gradient1.r * 2.0, radial_gradient1.r * 2.0, &radial_gradient1);
                return Style::lerp(&Style::from_image(image1), &Style::from_image(image2), t);
            }
            if image2.is_some() {
                let image2 = image2.unwrap();
                let image1 = ImageBitmap::fill_radial_gradient(image2.x, image2.y, image2.width / 2.0, image2.height / 2.0, &radial_gradient1);
                return Style::lerp(&Style::from_image(image1), &Style::from_image(image2), t);
            }
        }
        if image1.is_some() {
            let image1 = image1.unwrap();
            if color2.is_some() {
                let color2 = color2.unwrap();
                let image2 = ImageBitmap::fill(image1.x, image1.y, image1.width, image1.height, &color2);
                return Style::lerp(&Style::from_image(image1), &Style::from_image(image2), t);
            }
            if linear_gradient2.is_some() {
                let linear_gradient2 = linear_gradient2.unwrap();
                let image2 = ImageBitmap::fill_linear_gradient(linear_gradient2.p1.x, linear_gradient2.p1.y, linear_gradient2.p2.x, linear_gradient2.p2.y, &linear_gradient2);
                return Style::lerp(&Style::from_image(image1), &Style::from_image(image2), t);
            }
            if radial_gradient2.is_some() {
                let radial_gradient2 = radial_gradient2.unwrap();
                let image2 = ImageBitmap::fill_radial_gradient(radial_gradient2.c.x, radial_gradient2.c.y, radial_gradient2.r * 2.0, radial_gradient2.r * 2.0, &radial_gradient2);
                return Style::lerp(&Style::from_image(image1), &Style::from_image(image2), t);
            }
        }
        Style::default()
    }
}

impl Style {
    /// Creates a style given a Paint and a Opacity.
    pub fn from_paint_and_opacity(
        paint: &Paint,
        opacity: &Opacity,
    ) -> Style {
        match &paint {
            Paint::Color(color) => {
                let color = Color::new(color.red, color.green, color.blue, opacity.get());
                Style::from_color(color)
            }
            Paint::LinearGradient(gradient) => {
                let start = Point2D::new(gradient.x1(), gradient.y1());
                let end = Point2D::new(gradient.x2(), gradient.y2());
                let linear_gradient = LinearGradient::new(start, end, gradient.stops().iter().map(|stop| {
                    ColorStop {
                        color: Color::new(stop.color().red, stop.color().green, stop.color().blue, stop.opacity().get() * opacity.get()),
                        position: stop.offset().get(),
                    }
                }).collect());
                Style::from_linear_gradient(linear_gradient)
            }
            Paint::RadialGradient(gradient) => {
                let start = Point2D::new(gradient.fx(), gradient.fy());
                let end = Point2D::new(gradient.cx(), gradient.cy());
                let radial_gradient = RadialGradient::new(start, end, gradient.r().get(), gradient.stops().iter().map(|stop| {
                    ColorStop {
                        color: Color::new(stop.color().red, stop.color().green, stop.color().blue, stop.opacity().get() * opacity.get()),
                        position: stop.offset().get(),
                    }
                }).collect());
                Style::from_radial_gradient(radial_gradient)
            }
            Paint::Pattern(pattern) => {
                let root = pattern.root();
                for child in root.children() {
                    match &child {
                        usvg::Node::Image(image) => {
                            let x = image.abs_bounding_box().x();
                            let y = image.abs_bounding_box().y();
                            let width = image.abs_bounding_box().width();
                            let height = image.abs_bounding_box().height();
                            let kind = image.kind();
                            match &kind {
                                usvg::ImageKind::JPEG(data) => {
                                    // data is Arc<Vec<u8>>. Remove the Arc.
                                    let mut new_data = vec![];
                                    new_data.extend_from_slice(&data);
                                    return Style::from_image(ImageBitmap::new(x, y, width, height, new_data));
                                }
                                usvg::ImageKind::PNG(data) => {
                                    let mut new_data = vec![];
                                    new_data.extend_from_slice(&data);
                                    let image = ImageBitmap::new(x, y, width, height, new_data);
                                    return Style::from_image(image);
                                }
                                usvg::ImageKind::WEBP(data) => {
                                    let mut new_data = vec![];
                                    new_data.extend_from_slice(&data);
                                    let image = ImageBitmap::new(x, y, width, height, new_data);
                                    return Style::from_image(image);
                                }
                                _ => {
                                    log("Unsupported pattern. Fallback to default style (fully transparent black).");
                                    return Style::default();
                                }
                            }
                        }
                        _ => {
                            log("Unsupported pattern. Fallback to default style (fully transparent black).");
                            return Style::default();
                        }
                    }
                }
                log("Unsupported pattern. Fallback to default style (fully transparent black).");
                Style::default()
            }
        }
    }
}
