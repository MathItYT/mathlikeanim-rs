#[derive(Clone, Debug)]
pub enum GradientImageOrColor {
    LinearGradient(LinearGradient),
    RadialGradient(RadialGradient),
    Image(Image),
    Color(Color),
}

#[derive(Clone, Debug)]
pub struct GradientStop {
    pub offset: f64,
    pub color: Color,
}

#[derive(Clone, Debug)]
pub struct Image {
    pub image_base64: String,
    pub mime_type: String,
    pub top_left_corner: (f64, f64),
    pub bottom_right_corner: (f64, f64),
    pub alpha: f64,
}

#[derive(Clone, Debug)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
}

#[derive(Clone, Debug)]
pub struct LinearGradient {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub stops: Vec<GradientStop>,
    pub alpha: f64,
}

#[derive(Clone, Debug)]
pub struct RadialGradient {
    pub cx: f64,
    pub cy: f64,
    pub r: f64,
    pub fx: f64,
    pub fy: f64,
    pub stops: Vec<GradientStop>,
    pub alpha: f64,
}
