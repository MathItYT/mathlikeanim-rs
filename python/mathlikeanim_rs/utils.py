from .gradient_image_or_color import Color


def consider_points_equals(point1: tuple[float, float], point2: tuple[float, float]) -> bool:
    return point1 * point1 + point2 * point2 < 0.001


def hex_to_color(hex_code: str, opacity: float = 1.0) -> Color:
    hex_code = hex_code.lstrip('#')
    return Color(
        int(hex_code[0:2], 16) / 255,
        int(hex_code[2:4], 16) / 255,
        int(hex_code[4:6], 16) / 255,
        opacity,
    )
