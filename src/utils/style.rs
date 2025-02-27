use std::rc::Rc;

use image::{codecs::png::PngEncoder, guess_format, load_from_memory_with_format, ImageBuffer, RgbaImage};
use usvg::{Opacity, Paint};
use wasm_bindgen::prelude::*;
use base64::{prelude::BASE64_STANDARD, Engine};

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
    /// Creates a new Color with the given red, green, blue, and alpha components.
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
    /// Linearly interpolates between two Colors given a progress value.
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
    /// Creates a new ColorStop with the given color and position.
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
    /// Creates a new LinearGradient with the given start point, end point, and ColorStops.
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
    /// Returns the default LinearGradient, which is a gradient from the start to the end the same color.
    #[wasm_bindgen(return_description = "The default linear gradient.")]
    pub fn single_color_gradient(
        #[wasm_bindgen(param_description = "The start point of the gradient.")]
        p1: Point2D,
        #[wasm_bindgen(param_description = "The end point of the gradient.")]
        p2: Point2D,
        #[wasm_bindgen(param_description = "The color of the gradient.")]
        color: Color,
        #[wasm_bindgen(param_description = "Number of times to repeat the color.")]
        repeats: Option<usize>
    ) -> LinearGradient {
        let repeats = repeats.unwrap_or(2);
        let mut color_stops = vec![];
        for i in 0..repeats {
            color_stops.push(ColorStop { color, position: i as f32 / (repeats - 1) as f32 });
        }
        LinearGradient {
            p1,
            p2,
            color_stops: Rc::new(color_stops),
        }
    }
    /// Returns the default LinearGradient, which is a gradient from the origin to the origin with no ColorStops.
    #[wasm_bindgen(return_description = "The default linear gradient.")]
    pub fn default_linear_gradient() -> LinearGradient {
        LinearGradient::default()
    }
    /// Gets the ColorStops of the gradient.
    #[wasm_bindgen(getter, return_description = "The color stops of the gradient.")]
    pub fn color_stops(&self) -> Vec<ColorStop> {
        self.color_stops.to_vec()
    }
    /// Sets the ColorStops of the gradient.
    #[wasm_bindgen(setter)]
    pub fn set_color_stops(
        &mut self,
        #[wasm_bindgen(param_description = "The color stops of the gradient.")]
        color_stops: Vec<ColorStop>
    ) {
        self.color_stops = Rc::new(color_stops);
    }
    /// Gets the Color at a given offset along the gradient.
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
    /// Gets the Color at a given point along the gradient.
    #[wasm_bindgen(return_description = "The color at the given point.")]
    pub fn color_at(
        &self,
        #[wasm_bindgen(param_description = "The point to get the color at.")]
        p: Point2D
    ) -> Color {
        let t = p.project_onto_line(&self.p1, &self.p2);
        self.color_at_offset(t)
    }
    /// Linearly interpolates between two LinearGradients given a progress value.
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
    /// The start circle center point of the gradient.
    pub f: Point2D,
    /// The end circle center point of the gradient.
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
    /// Creates a new RadialGradient with the given focal point, center point, radius, and ColorStops.
    #[wasm_bindgen(constructor, return_description = "A new radial gradient.")]
    pub fn new(
        #[wasm_bindgen(param_description = "The start circle center point of the gradient.")]
        f: Point2D,
        #[wasm_bindgen(param_description = "The end circle center point of the gradient.")]
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
    /// Returns the default RadialGradient, which is a gradient from the focal point to the center with the same color.
    #[wasm_bindgen(return_description = "The default radial gradient.")]
    pub fn default_radial_gradient() -> RadialGradient {
        RadialGradient::default()
    }
    /// Gets the ColorStops of the gradient.
    #[wasm_bindgen(getter, return_description = "The color stops of the gradient.")]
    pub fn color_stops(&self) -> Vec<ColorStop> {
        self.color_stops.to_vec()
    }
    /// Sets the ColorStops of the gradient.
    #[wasm_bindgen(setter)]
    pub fn set_color_stops(
        &mut self,
        #[wasm_bindgen(param_description = "The color stops of the gradient.")]
        color_stops: Vec<ColorStop>
    ) {
        self.color_stops = Rc::new(color_stops);
    }
    /// Gets the Color at a given offset along the gradient.
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
    /// Gets the Color at a given point along the gradient.
    #[wasm_bindgen(return_description = "The color at the given point.")]
    pub fn color_at(
        &self,
        #[wasm_bindgen(param_description = "The point to get the color at.")]
        p: Point2D
    ) -> Color {
        // We need first to solve this equation: dist(c, f + (p - f) * t) = r
        // where dist is the distance between two points, c is the center point, f is the focal point, p is the point, and r is the radius.
        // This simplifies to a quadratic equation At^2 + Bt + C = 0, where:
        // A = (p.x - f.x)^2 + (p.y - f.y)^2
        // B = 2 * (f.x - c.x) * (p.x - f.x) + 2 * (f.y - c.y) * (p.y - f.y)
        // C = (f.x - c.x)^2 + (f.y - c.y)^2 - r^2
        // The solutions are t = (-B ± sqrt(B^2 - 4AC)) / 2A and we choose the one with plus sign.
        let a = (p.x - self.f.x).powi(2) + (p.y - self.f.y).powi(2);
        let b = 2.0 * (self.f.x - self.c.x) * (p.x - self.f.x) + 2.0 * (self.f.y - self.c.y) * (p.y - self.f.y);
        let c = (self.f.x - self.c.x).powi(2) + (self.f.y - self.c.y).powi(2) - self.r.powi(2);
        // It may be possible that A = 0, in which case the distance from p to f is 0, then p is f, so the offset is 0.0
        if a == 0.0 {
            return self.color_at_offset(0.0);
        }
        let t = (-b + (b.powi(2) - 4.0 * a * c).sqrt()) / (2.0 * a);
        // Then let's project the point onto the line segment from f to f + (p - f) * t
        let projection = p.project_onto_line(&self.f, &(self.f + (p - self.f) * t));
        self.color_at_offset(projection)
    }
    /// Returns a single color radial gradient.
    #[wasm_bindgen(return_description = "A single color radial gradient.")]
    pub fn single_color_gradient(
        #[wasm_bindgen(param_description = "The start circle center point of the gradient.")]
        f: Point2D,
        #[wasm_bindgen(param_description = "The end circle center point of the gradient.")]
        c: Point2D,
        #[wasm_bindgen(param_description = "The radius of the gradient.")]
        r: f32,
        #[wasm_bindgen(param_description = "The color of the gradient.")]
        color: Color,
        #[wasm_bindgen(param_description = "Number of times to repeat the color.")]
        repeats: Option<usize>
    ) -> RadialGradient {
        let repeats = repeats.unwrap_or(2);
        let mut color_stops = vec![];
        for i in 0..repeats {
            color_stops.push(ColorStop { color, position: i as f32 / (repeats - 1) as f32 });
        }
        RadialGradient {
            f,
            c,
            r,
            color_stops: Rc::new(color_stops),
        }
    }
    /// Linearly interpolates between two RadialGradients given a progress value.
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
    /// Number of pixels in a row of the bitmap.
    pub data_width: usize,
    /// Number of pixels in a column of the bitmap.
    pub data_height: usize,
    /// Rgba data of the bitmap.
    rgba_image: ImageBuffer<image::Rgba<u8>, Vec<u8>>,
}

impl Default for ImageBitmap {
    fn default() -> Self {
        ImageBitmap {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            data_width: 0,
            data_height: 0,
            rgba_image: RgbaImage::new(0, 0)
        }
    }
}

#[wasm_bindgen]
impl ImageBitmap {
    /// Creates a new ImageBitmap with the given x, y, width, height, and pixel data.
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
        #[wasm_bindgen(param_description = "Number of pixels in a row of the bitmap.")]
        data_width: usize,
        #[wasm_bindgen(param_description = "Number of pixels in a column of the bitmap.")]
        data_height: usize,
        #[wasm_bindgen(param_description = "The pixel data of the bitmap.")]
        data: Vec<u8>
    ) -> Result<ImageBitmap, JsError> {
        let rgba_image = guess_format(&data).and_then(|format| {
            let img = load_from_memory_with_format(&data, format)?;
            let img = img.to_rgba8();
            Ok(img)
        });
        if rgba_image.is_err() {
            log("Failed to create image bitmap.");
            return Err(JsError::new("Failed to create image bitmap."));
        }
        let rgba_image = rgba_image.unwrap();
        Ok(ImageBitmap {
            x,
            y,
            width,
            height,
            data_width,
            data_height,
            rgba_image,
        })
    }
    /// Gets the pixel data of the bitmap.
    #[wasm_bindgen(getter, return_description = "The pixel data of the bitmap.")]
    pub fn data(&self) -> Vec<u8> {
        self.rgba_image.clone().into_raw()
    }
    /// Sets the pixel data of the bitmap.
    #[wasm_bindgen]
    pub fn set_data(
        &mut self,
        #[wasm_bindgen(param_description = "The number of pixels in a row of the bitmap.")]
        data_width: f32,
        #[wasm_bindgen(param_description = "The number of pixels in a column of the bitmap.")]
        data_height: f32,
        #[wasm_bindgen(param_description = "The pixel data of the bitmap.")]
        data: Vec<u8>
    ) -> Result<(), JsError> {
        self.data_width = data_width as usize;
        self.data_height = data_height as usize;
        self.rgba_image = RgbaImage::from_raw(data_width as u32, data_height as u32, data).ok_or(JsError::new("Failed to set image bitmap data."))?;
        Ok(())
    }
    /// Returns the default ImageBitmap, which is an empty bitmap.
    #[wasm_bindgen(return_description = "The default image bitmap.")]
    pub fn default_image_bitmap() -> ImageBitmap {
        ImageBitmap::default()
    }
    /// Gets the color of a Point2D in the bitmap.
    #[wasm_bindgen(return_description = "The color of the pixel.")]
    pub fn get_pixel(
        &self,
        #[wasm_bindgen(param_description = "The point to get the pixel color at.")]
        p: Point2D
    ) -> Color {
        let x = ((p.x - self.x) % self.width / self.width * self.data_width as f32) as u32;
        let y = ((p.y - self.y) % self.height / self.height * self.data_height as f32) as u32;
        let pixel = self.rgba_image.get_pixel(x, y);
        Color::new(pixel[0], pixel[1], pixel[2], pixel[3] as f32 / 255.0)
    }
    /// Sets a pixel color at a Point2D in the bitmap.
    pub fn set_pixel(
        &mut self,
        #[wasm_bindgen(param_description = "The point to set the pixel color at.")]
        p: Point2D,
        #[wasm_bindgen(param_description = "The color of the pixel.")]
        color: &Color
    ) {
        let x = ((p.x - self.x) % self.width / self.width * self.data_width as f32) as u32;
        let y = ((p.y - self.y) % self.height / self.height * self.data_height as f32) as u32;
        self.rgba_image.put_pixel(x, y, image::Rgba([color.red, color.green, color.blue, (color.alpha * 255.0) as u8]));
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
        #[wasm_bindgen(param_description = "Number of pixels in a row of the bitmap.")]
        data_width: usize,
        #[wasm_bindgen(param_description = "Number of pixels in a column of the bitmap.")]
        data_height: usize,
        #[wasm_bindgen(param_description = "The color to fill the bitmap with.")]
        color: &Color
    ) -> ImageBitmap {
        let rgba_image = RgbaImage::from_pixel(data_width as u32, data_height as u32, image::Rgba([color.red, color.green, color.blue, (color.alpha * 255.0) as u8]));
        ImageBitmap {
            x,
            y,
            width,
            height,
            data_width,
            data_height,
            rgba_image: rgba_image.clone(),
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
        #[wasm_bindgen(param_description = "Number of pixels in a row of the bitmap.")]
        data_width: usize,
        #[wasm_bindgen(param_description = "Number of pixels in a column of the bitmap.")]
        data_height: usize,
        #[wasm_bindgen(param_description = "The linear gradient to fill the bitmap with.")]
        gradient: &LinearGradient,
    ) -> ImageBitmap {
        let rgba_image = RgbaImage::from_fn(data_width as u32, data_height as u32, |x_raw, y_raw| {
            let p = Point2D::new(x_raw as f32 / data_width as f32 * width + x, y_raw as f32 / data_height as f32 * height + y);
            let color = gradient.color_at(p);
            image::Rgba([color.red, color.green, color.blue, (color.alpha * 255.0) as u8])
        });
        ImageBitmap {
            x,
            y,
            width,
            height,
            data_width,
            data_height,
            rgba_image: rgba_image.clone(),
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
        #[wasm_bindgen(param_description = "Number of pixels in a row of the bitmap.")]
        data_width: usize,
        #[wasm_bindgen(param_description = "Number of pixels in a column of the bitmap.")]
        data_height: usize,
        #[wasm_bindgen(param_description = "The radial gradient to fill the bitmap with.")]
        gradient: &RadialGradient,
    ) -> ImageBitmap {
        let rgba_image = RgbaImage::from_fn(data_width as u32, data_height as u32, |x_raw, y_raw| {
            let p = Point2D::new(x_raw as f32 / data_width as f32 * width + x, y_raw as f32 / data_height as f32 * height + y);
            let color = gradient.color_at(p);
            image::Rgba([color.red, color.green, color.blue, (color.alpha * 255.0) as u8])
        });
        ImageBitmap {
            x,
            y,
            width,
            height,
            data_width,
            data_height,
            rgba_image: rgba_image.clone(),
        }
    }
    /// Gets the data as base64 encoded string.
    #[wasm_bindgen(getter, return_description = "The base64 encoded string of the image bitmap.")]
    pub fn base64(&self) -> Result<String, String> {
        let mut png_data = vec![];
        let encoder = PngEncoder::new(&mut png_data);
        self.rgba_image.write_with_encoder(encoder).map_err(|e| e.to_string())?;
        let base64 = BASE64_STANDARD.encode(&png_data);
        Ok(base64)
    }
    /// Linearly interpolates between two ImageBitmaps given a progress value.
    #[wasm_bindgen(return_description = "The interpolated image bitmap.")]
    pub fn lerp(
        #[wasm_bindgen(param_description = "The first image bitmap.")]
        bitmap1: &ImageBitmap,
        #[wasm_bindgen(param_description = "The second image bitmap.")]
        bitmap2: &ImageBitmap,
        #[wasm_bindgen(param_description = "The progress value.")]
        t: f32,
    ) -> ImageBitmap {
        let x = bitmap1.x.min(bitmap2.x);
        let y = bitmap1.y.min(bitmap2.y);
        let width = bitmap1.width.max(bitmap2.width);
        let height = bitmap1.height.max(bitmap2.height);
        let data_width = bitmap1.data_width.max(bitmap2.data_width);
        let data_height = bitmap1.data_height.max(bitmap2.data_height);
        let new_image = RgbaImage::from_fn(data_width as u32, data_height as u32, |x_raw, y_raw| {
            let p = Point2D::new(x_raw as f32 / data_width as f32 * width + x, y_raw as f32 / data_height as f32 * height + y);
            let color1 = bitmap1.get_pixel(p);
            let color2 = bitmap2.get_pixel(p);
            let color = Color::lerp(&color1, &color2, t);
            image::Rgba([color.red, color.green, color.blue, (color.alpha * 255.0) as u8])
        });
        ImageBitmap {
            x,
            y,
            width,
            height,
            data_width,
            data_height,
            rgba_image: new_image.clone(),
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
    /// Creates a new Style with the given color, linear gradient, radial gradient, or image. It must have exactly one of these.
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
    /// Clones the Style.
    #[wasm_bindgen(js_name = clone, return_description = "The cloned style.")]
    pub fn clone_js(&self) -> Style {
        self.clone()
    }
    /// Creates a new Style with the given color.
    #[wasm_bindgen(return_description = "A new style from the color.")]
    pub fn from_color(
        #[wasm_bindgen(param_description = "The color of the style.")]
        color: Color
    ) -> Style {
        Style::new(Some(color), None, None, None).unwrap()
    }
    /// Creates a new Style with the given linear gradient.
    #[wasm_bindgen(return_description = "A new style from the linear gradient.")]
    pub fn from_linear_gradient(
        #[wasm_bindgen(param_description = "The linear gradient of the style.")]
        gradient: LinearGradient
    ) -> Style {
        Style::new(None, Some(gradient), None, None).unwrap()
    }
    /// Creates a new Style with the given radial gradient.
    #[wasm_bindgen(return_description = "A new style from the radial gradient.")]
    pub fn from_radial_gradient(gradient: RadialGradient) -> Style {
        Style::new(None, None, Some(gradient), None).unwrap()
    }
    /// Creates a new Style with the given image.
    #[wasm_bindgen(return_description = "A new style from the image.")]
    pub fn from_image(
        #[wasm_bindgen(param_description = "The image of the style.")]
        image: ImageBitmap
    ) -> Style {
        Style::new(None, None, None, Some(image)).unwrap()
    }
    /// Returns the default Style, which is a color with the default color.
    #[wasm_bindgen(return_description = "The default style.")]
    pub fn default_style() -> Style {
        Style::default()
    }
    /// Fades the Style by a given amount.
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
            for pixel in image.rgba_image.pixels_mut() {
                pixel[3] = (pixel[3] as f32 * (1.0 - amount)) as u8;
            }
        }
        Style {
            color,
            linear_gradient,
            radial_gradient,
            image,
        }
    }
    /// Gets the Color of the style, if it's a color.
    #[wasm_bindgen(getter, return_description = "The color of the style.")]
    pub fn color(&self) -> Option<Color> {
        self.color.clone()
    }
    /// Sets the style to a Color.
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
    /// Gets the LinearGradient of the style, if it's a linear gradient.
    #[wasm_bindgen(getter, return_description = "The linear gradient of the style.")]
    pub fn linear_gradient(&self) -> Option<LinearGradient> {
        self.linear_gradient.clone()
    }
    /// Sets the style to a LinearGradient.
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
    /// Gets the RadialGradient of the style, if it's a radial gradient.
    #[wasm_bindgen(getter, return_description = "The radial gradient of the style.")]
    pub fn radial_gradient(&self) -> Option<RadialGradient> {
        self.radial_gradient.clone()
    }
    /// Sets the style to a RadialGradient.
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
    /// Gets the ImageBitmap of the style, if it's an image.
    #[wasm_bindgen(getter, return_description = "The image of the style.")]
    pub fn image(&self) -> Option<ImageBitmap> {
        self.image.clone()
    }
    /// Sets the style to an ImageBitmap.
    #[wasm_bindgen(setter)]
    pub fn set_image(&mut self, image: ImageBitmap) {
        self.color = None;
        self.linear_gradient = None;
        self.radial_gradient = None;
        self.image = Some(image);
    }
    /// Gets the Color at a given point.
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
    /// Linearly interpolates between two Styles given a progress value.
    #[wasm_bindgen(return_description = "The interpolated style.")]
    pub fn lerp(
        #[wasm_bindgen(param_description = "The first style.")]
        style1: &Style,
        #[wasm_bindgen(param_description = "The second style.")]
        style2: &Style,
        #[wasm_bindgen(param_description = "The progress value.")]
        t: f32,
        #[wasm_bindgen(param_description = "Top left x coordinate of the bitmap. Must be provided if both styles are images.")]
        x: Option<f32>,
        #[wasm_bindgen(param_description = "Top left y coordinate of the bitmap. Must be provided if both styles are images.")]
        y: Option<f32>,
        #[wasm_bindgen(param_description = "Width of the bitmap. Must be provided if both styles are images.")]
        width: Option<f32>,
        #[wasm_bindgen(param_description = "Height of the bitmap. Must be provided if both styles are images.")]
        height: Option<f32>,
        #[wasm_bindgen(param_description = "Number of pixels in a row of the bitmap. Must be provided if both styles are different kinds of gradients or one of them is an image.")]
        data_width: Option<usize>,
        #[wasm_bindgen(param_description = "Number of pixels in a column of the bitmap. Must be provided if both styles are different kinds of gradients or one of them is an image.")]
        data_height: Option<usize>
    ) -> Result<Style, String> {
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
            return Ok(Style::from_color(color));
        }
        if linear_gradient1.is_some() && linear_gradient2.is_some() {
            let linear_gradient = LinearGradient::lerp(&linear_gradient1.unwrap(), &linear_gradient2.unwrap(), t);
            return Ok(Style::from_linear_gradient(linear_gradient));
        }
        if radial_gradient1.is_some() && radial_gradient2.is_some() {
            let radial_gradient = RadialGradient::lerp(&radial_gradient1.unwrap(), &radial_gradient2.unwrap(), t);
            return Ok(Style::from_radial_gradient(radial_gradient));
        }
        if image1.is_some() && image2.is_some() {
            let image = ImageBitmap::lerp(&image1.unwrap(), &image2.unwrap(), t);
            return Ok(Style::from_image(image));
        }
        if color1.is_some() {
            let color = color1.unwrap();
            if linear_gradient2.is_some() {
                let linear_gradient2 = linear_gradient2.unwrap();
                let linear_gradient1 = LinearGradient::single_color_gradient(linear_gradient2.p1, linear_gradient2.p2, color, Some(linear_gradient2.color_stops.len()));
                return Style::lerp(&Style::from_linear_gradient(linear_gradient1), &Style::from_linear_gradient(linear_gradient2), t, x, y, width, height, data_width, data_height);
            }
            if radial_gradient2.is_some() {
                let radial_gradient2 = radial_gradient2.unwrap();
                let radial_gradient1 = RadialGradient::single_color_gradient(radial_gradient2.f, radial_gradient2.c, radial_gradient2.r, color, Some(radial_gradient2.color_stops.len()));
                return Style::lerp(&Style::from_radial_gradient(radial_gradient1), &Style::from_radial_gradient(radial_gradient2), t, x, y, width, height, data_width, data_height);
            }
            if image2.is_some() {
                if x.is_none() || y.is_none() || width.is_none() || height.is_none() {
                    return Err("Bitmap data must be provided if one of the styles is an image.".to_string());
                }
                let image2 = image2.unwrap();
                let image1 = ImageBitmap::fill(x.unwrap(), y.unwrap(), width.unwrap(), height.unwrap(), data_width.unwrap(), data_height.unwrap(), &color);
                return Style::lerp(&Style::from_image(image1), &Style::from_image(image2), t, x, y, width, height, data_width, data_height);
            }
        }
        if linear_gradient1.is_some() {
            let linear_gradient1 = linear_gradient1.unwrap();
            if color2.is_some() {
                let color2 = color2.unwrap();
                let linear_gradient2 = LinearGradient::single_color_gradient(linear_gradient1.p1, linear_gradient1.p2, color2, Some(linear_gradient1.color_stops.len()));
                return Style::lerp(&Style::from_linear_gradient(linear_gradient1), &Style::from_linear_gradient(linear_gradient2), t, x, y, width, height, data_width, data_height);
            }
            if radial_gradient2.is_some() {
                if x.is_none() || y.is_none() || width.is_none() || height.is_none() {
                    return Err("Bitmap data must be provided if both styles are different kinds of gradients.".to_string());
                }
                let radial_gradient2 = radial_gradient2.unwrap();
                let image2 = ImageBitmap::fill_radial_gradient(x.unwrap(), y.unwrap(), width.unwrap(), height.unwrap(), data_width.unwrap(), data_height.unwrap(), &radial_gradient2);
                let image1 = ImageBitmap::fill_linear_gradient(x.unwrap(), y.unwrap(), width.unwrap(), height.unwrap(), data_width.unwrap(), data_height.unwrap(), &linear_gradient1);
                return Style::lerp(&Style::from_image(image1), &Style::from_image(image2), t, x, y, width, height, data_width, data_height);
            }
            if image2.is_some() {
                if x.is_none() || y.is_none() || width.is_none() || height.is_none() {
                    return Err("Bitmap data must be provided if both styles are different kinds of gradients.".to_string());
                }
                let image2 = image2.unwrap();
                let image1 = ImageBitmap::fill_linear_gradient(x.unwrap(), y.unwrap(), width.unwrap(), height.unwrap(), data_width.unwrap(), data_height.unwrap(), &linear_gradient1);
                return Style::lerp(&Style::from_image(image1), &Style::from_image(image2), t, x, y, width, height, data_width, data_height);
            }
        }
        if radial_gradient1.is_some() {
            let radial_gradient1 = radial_gradient1.unwrap();
            if color2.is_some() {
                let color2 = color2.unwrap();
                let radial_gradient2 = RadialGradient::single_color_gradient(radial_gradient1.f, radial_gradient1.c, radial_gradient1.r, color2, Some(radial_gradient1.color_stops.len()));
                return Style::lerp(&Style::from_radial_gradient(radial_gradient1), &Style::from_radial_gradient(radial_gradient2), t, x, y, width, height, data_width, data_height);
            }
            if linear_gradient2.is_some() {
                if x.is_none() || y.is_none() || width.is_none() || height.is_none() {
                    return Err("Bitmap data must be provided if both styles are different kinds of gradients.".to_string());
                }
                let linear_gradient2 = linear_gradient2.unwrap();
                let image2 = ImageBitmap::fill_linear_gradient(x.unwrap(), y.unwrap(), width.unwrap(), height.unwrap(), data_width.unwrap(), data_height.unwrap(), &linear_gradient2);
                let image1 = ImageBitmap::fill_radial_gradient(x.unwrap(), y.unwrap(), width.unwrap(), height.unwrap(), data_width.unwrap(), data_height.unwrap(), &radial_gradient1);
                return Style::lerp(&Style::from_image(image1), &Style::from_image(image2), t, x, y, width, height, data_width, data_height);
            }
            if image2.is_some() {
                if x.is_none() || y.is_none() || width.is_none() || height.is_none() {
                    return Err("Bitmap data must be provided if both styles are different kinds of gradients.".to_string());
                }
                let image2 = image2.unwrap();
                let image1 = ImageBitmap::fill_radial_gradient(x.unwrap(), y.unwrap(), width.unwrap(), height.unwrap(), data_width.unwrap(), data_height.unwrap(), &radial_gradient1);
                return Style::lerp(&Style::from_image(image1), &Style::from_image(image2), t, x, y, width, height, data_width, data_height);
            }
        }
        if image1.is_some() {
            let image1 = image1.unwrap();
            if color2.is_some() {
                if x.is_none() || y.is_none() || width.is_none() || height.is_none() {
                    return Err("Bitmap data must be provided if one of the styles is an image.".to_string());
                }
                let color2 = color2.unwrap();
                let image2 = ImageBitmap::fill(x.unwrap(), y.unwrap(), width.unwrap(), height.unwrap(), data_width.unwrap(), data_height.unwrap(), &color2);
                return Style::lerp(&Style::from_image(image1), &Style::from_image(image2), t, x, y, width, height, data_width, data_height);
            }
            if linear_gradient2.is_some() {
                if x.is_none() || y.is_none() || width.is_none() || height.is_none() {
                    return Err("Bitmap data must be provided if one of the styles is an image.".to_string());
                }
                let linear_gradient2 = linear_gradient2.unwrap();
                let image2 = ImageBitmap::fill_linear_gradient(x.unwrap(), y.unwrap(), width.unwrap(), height.unwrap(), data_width.unwrap(), data_height.unwrap(), &linear_gradient2);
                return Style::lerp(&Style::from_image(image1), &Style::from_image(image2), t, x, y, width, height, data_width, data_height);
            }
            if radial_gradient2.is_some() {
                if x.is_none() || y.is_none() || width.is_none() || height.is_none() {
                    return Err("Bitmap data must be provided if one of the styles is an image.".to_string());
                }
                let radial_gradient2 = radial_gradient2.unwrap();
                let image2 = ImageBitmap::fill_radial_gradient(x.unwrap(), y.unwrap(), width.unwrap(), height.unwrap(), data_width.unwrap(), data_height.unwrap(), &radial_gradient2);
                return Style::lerp(&Style::from_image(image1), &Style::from_image(image2), t, x, y, width, height, data_width, data_height);
            }
        }
        Err("Exactly one of color, linear_gradient, radial_gradient, or image must be provided.".to_string())
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
                let bounding_box = pattern.rect();
                let x = bounding_box.x();
                let y = bounding_box.y();
                let width = bounding_box.width();
                let height = bounding_box.height();
                for child in root.children() {
                    return Style::from_pattern_child(&child, x, y, width, height);
                }
                log("Unsupported pattern. Fallback to default style (fully transparent black).");
                Style::default()
            }
        }
    }
    pub fn from_pattern_child(
        child: &usvg::Node,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    ) -> Style {
        match &child {
            usvg::Node::Image(image) => {
                let kind = image.kind();
                let size = image.size();
                let data_width = size.width().round() as usize;
                let data_height = size.height().round() as usize;
                match &kind {
                    usvg::ImageKind::JPEG(data) => {
                        let new_data = data.to_vec();
                        let image = ImageBitmap::new(x, y, width, height, data_width, data_height, new_data);
                        if image.is_err() {
                            log("Failed to create image bitmap.");
                            return Style::default();
                        }
                        return Style::from_image(image.unwrap());
                    }
                    usvg::ImageKind::PNG(data) => {
                        let new_data = data.to_vec();
                        let image = ImageBitmap::new(x, y, width, height, data_width, data_height, new_data);
                        if image.is_err() {
                            log("Failed to create image bitmap.");
                            return Style::default();
                        }
                        return Style::from_image(image.unwrap());
                    }
                    usvg::ImageKind::WEBP(data) => {
                        let new_data = data.to_vec();
                        let image = ImageBitmap::new(x, y, width, height, data_width, data_height, new_data);
                        if image.is_err() {
                            log("Failed to create image bitmap.");
                            return Style::default();
                        }
                        return Style::from_image(image.unwrap());
                    }
                    _ => {
                        log("Unsupported image format. Fallback to default style (fully transparent black).");
                        return Style::default();
                    }
                }
            }
            usvg::Node::Group(group) => {
                for child in group.children() {
                    return Style::from_pattern_child(&child, x, y, width, height);
                }
                log("Unsupported pattern. Fallback to default style (fully transparent black).");
                return Style::default();
            }
            _ => {
                log("Unsupported pattern. Fallback to default style (fully transparent black).");
                return Style::default();
            }
        }
    }
}
