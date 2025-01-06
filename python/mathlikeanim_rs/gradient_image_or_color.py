class Color:
    def __init__(
        self,
        r: float,
        g: float,
        b: float,
        a: float = 1.0,
    ) -> None:
        self.r = r
        self.g = g
        self.b = b
        self.a = a
    
    @staticmethod
    def default() -> 'Color':
        return Color(0.0, 0.0, 0.0, 0.0)

    @staticmethod
    def from_dict(data) -> 'Color':
        return Color(data['r'], data['g'], data['b'], a=data['a'])

    def to_dict(self):
        return {
            'type': 'color',
            'r': self.r,
            'g': self.g,
            'b': self.b,
            'a': self.a,
        }


class GradientStop:
    def __init__(
        self,
        offset: float,
        color: Color,
    ) -> None:
        self.offset = offset
        self.color = color
    
    @staticmethod
    def from_dict(data) -> 'GradientStop':
        return GradientStop(data['offset'], Color.from_dict(data['color']))

    def to_dict(self):
        return {
            'type': 'gradientStop',
            'offset': self.offset,
            'color': self.color.to_dict(),
        }


class LinearGradient:
    def __init__(
        self,
        start: tuple[float, float],
        end: tuple[float, float],
        stops: list[GradientStop],
        alpha: float = 1.0,
    ) -> None:
        self.start = start
        self.end = end
        self.stops = stops
        self.alpha = alpha
    
    @staticmethod
    def from_dict(data) -> 'LinearGradient':
        return LinearGradient(
            (data['x1'], data['y1']),
            (data['x2'], data['y2']),
            [GradientStop.from_dict(stop) for stop in data['stops']],
            alpha=data['alpha'],
        )

    def to_dict(self):
        return {
            'type': 'linearGradient',
            'x1': self.start[0],
            'y1': self.start[1],
            'x2': self.end[0],
            'y2': self.end[1],
            'stops': [stop.to_dict() for stop in self.stops],
            'alpha': self.alpha,
        }


class RadialGradient:
    def __init__(
        self,
        c: tuple[float, float],
        r: float,
        f: tuple[float, float],
        stops: list[GradientStop],
        alpha: float = 1.0,
    ) -> None:
        self.c = c
        self.r = r
        self.f = f
        self.stops = stops
        self.alpha = alpha
    
    @staticmethod
    def from_dict(data) -> 'RadialGradient':
        return RadialGradient(
            (data['cx'], data['cy']),
            data['r'],
            (data['fx'], data['fy']),
            [GradientStop.from_dict(stop) for stop in data['stops']],
            alpha=data['alpha'],
        )

    def to_dict(self):
        return {
            'type': 'radialGradient',
            'cx': self.cx,
            'cy': self.cy,
            'r': self.r,
            'fx': self.fx,
            'fy': self.fy,
            'stops': [stop.to_dict() for stop in self.stops],
            'alpha': self.alpha,
        }


class Image:
    def __init__(
        self,
        image_base64: str,
        mime_type: str,
        top_left: tuple[float, float],
        bottom_right: tuple[float, float],
        alpha: float = 1.0,
    ) -> None:
        self.image_base64 = image_base64
        self.mime_type = mime_type
        self.top_left = top_left
        self.bottom_right = bottom_right
        self.alpha = alpha
    
    @staticmethod
    def from_dict(data) -> 'Image':
        return Image(
            data['imageBase64'],
            data['mimeType'],
            (data['left'], data['top']),
            (data['right'], data['bottom']),
            alpha=data['alpha'],
        )
    
    def to_dict(self):
        return {
            'type': 'image',
            'imageBase64': self.image_base64,
            'mimeType': self.mime_type,
            'left': self.top_left[0],
            'top': self.top_left[1],
            'right': self.bottom_right[0],
            'bottom': self.bottom_right[1],
            'alpha': self.alpha,
        }


class GradientImageOrColor:
    def __init__(
        self,
        gradient_image_or_color: LinearGradient | RadialGradient | Image | Color,
    ) -> None:
        self.gradient_image_or_color = gradient_image_or_color
    
    @staticmethod
    def default() -> 'GradientImageOrColor':
        return GradientImageOrColor(Color.default())
    
    @staticmethod
    def from_dict(data) -> 'GradientImageOrColor':
        if data['gradientImageOrColor']['type'] == 'linearGradient':
            return GradientImageOrColor(LinearGradient.from_dict(data['gradientImageOrColor']))
        elif data['gradientImageOrColor']['type'] == 'radialGradient':
            return GradientImageOrColor(RadialGradient.from_dict(data['gradientImageOrColor']))
        elif data['gradientImageOrColor']['type'] == 'image':
            return GradientImageOrColor(Image.from_dict(data['gradientImageOrColor']))
        elif data['gradientImageOrColor']['type'] == 'color':
            return GradientImageOrColor(Color.from_dict(data['gradientImageOrColor']))
        else:
            raise ValueError(f'Unknown type: {data["type"]}')
    
    def to_dict(self):
        return {
            'type': 'gradientImageOrColor',
            'gradientImageOrColor': self.gradient_image_or_color.to_dict(),
        }