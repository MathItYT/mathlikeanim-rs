from flask import Flask, request, jsonify
import manim as mn


app = Flask(__name__)
banner = mn.ManimBanner()


def manim_to_mathlikeanimrs_coords(x: float, y: float) -> tuple[float, float]:
    x += mn.config.frame_x_radius
    y *= -1
    y += mn.config.frame_y_radius
    x *= 3840 / mn.config.frame_width
    y *= 2160 / mn.config.frame_height
    return x, y


def get_points_list(vmobject: mn.VMobject) -> list[list[float]]:
    result = []
    for point in vmobject.points:
        x, y = manim_to_mathlikeanimrs_coords(point[0], point[1])
        # if abs(x) > 1e-6 or abs(y) > 1e-6:
        #     return []
        result.append([x, y])
    return result


def vmobject_to_json(vmobject: mn.VMobject) -> dict:
    return {
        'points': get_points_list(vmobject),
        'fill-color': vmobject.fill_color.to_hex(),
        'fill-opacity': vmobject.fill_opacity,
        'stroke-color': vmobject.stroke_color.to_hex(),
        'stroke-opacity': vmobject.stroke_opacity,
        'stroke-width': vmobject.stroke_width,
        'submobjects': [vmobject_to_json(submob) for submob in vmobject.submobjects],
    }


def mobject_from_animation_and_t(
    animation: mn.Animation,
    mobject: mn.VMobject,
    t: float
) -> mn.VMobject:
    if isinstance(animation, mn.Succession):
        animation.scene = mn.Scene()
    animation.begin()
    animation.interpolate(t)
    return mobject


@app.route('/banner-creation', methods=['GET'])
def banner_creation():
    t = request.args.get('t', type=float, default=0)
    mobject = banner.copy()
    mobject_from_animation_and_t(mobject.create(), mobject, t)
    return jsonify(vmobject_to_json(mobject))


@app.route('/banner-expand', methods=['GET'])
def banner_expand():
    t = request.args.get('t', type=float, default=0)
    mobject = banner.copy()
    mobject_from_animation_and_t(mobject.expand(), mobject, t)
    return jsonify(vmobject_to_json(mobject))


if __name__ == '__main__':
    app.run()