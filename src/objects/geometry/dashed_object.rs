use crate::{objects::vector_object::VectorObject, utils::interp};

pub fn dashed_object(
    vector_object: &VectorObject,
    num_dashes: Option<usize>,
    dashed_ratio: Option<f64>,
    dash_offset: Option<f64>,
    equal_lengths: Option<bool>,
) -> VectorObject {
    // Borrowed from Manim Community (https://github.com/ManimCommunity/manim/blob/dbad8a86bc6041af78d5fb7d7a07f76d9b164072/manim/mobject/types/vectorized_mobject.py)
    // r = self.dashed_ratio
    // n = self.num_dashes
    // if n > 0:
    //     # Assuming total length is 1
    //     dash_len = r / n
    //     if vmobject.is_closed():
    //         void_len = (1 - r) / n
    //     else:
    //         if n == 1:
    //             void_len = 1 - r
    //         else:
    //             void_len = (1 - r) / (n - 1)

    //     period = dash_len + void_len
    //     phase_shift = (dash_offset % 1) * period

    //     if vmobject.is_closed():
    //         # closed curves have equal amount of dashes and voids
    //         pattern_len = 1
    //     else:
    //         # open curves start and end with a dash, so the whole dash pattern with the last void is longer
    //         pattern_len = 1 + void_len

    //     dash_starts = [((i * period + phase_shift) % pattern_len) for i in range(n)]
    //     dash_ends = [
    //         ((i * period + dash_len + phase_shift) % pattern_len) for i in range(n)
    //     ]

    //     # closed shapes can handle overflow at the 0-point
    //     # open shapes need special treatment for it
    //     if not vmobject.is_closed():
    //         # due to phase shift being [0...1] range, always the last dash element needs attention for overflow
    //         # if an entire dash moves out of the shape end:
    //         if dash_ends[-1] > 1 and dash_starts[-1] > 1:
    //             # remove the last element since it is out-of-bounds
    //             dash_ends.pop()
    //             dash_starts.pop()
    //         elif dash_ends[-1] < dash_len:  # if it overflowed
    //             if (
    //                 dash_starts[-1] < 1
    //             ):  # if the beginning of the piece is still in range
    //                 dash_starts.append(0)
    //                 dash_ends.append(dash_ends[-1])
    //                 dash_ends[-2] = 1
    //             else:
    //                 dash_starts[-1] = 0
    //         elif dash_starts[-1] > (1 - dash_len):
    //             dash_ends[-1] = 1

    //     if equal_lengths:
    //         # calculate the entire length by adding up short line-pieces
    //         norms = np.array(0)
    //         for k in range(vmobject.get_num_curves()):
    //             norms = np.append(norms, vmobject.get_nth_curve_length_pieces(k))
    //         # add up length-pieces in array form
    //         length_vals = np.cumsum(norms)
    //         ref_points = np.linspace(0, 1, length_vals.size)
    //         curve_length = length_vals[-1]
    //         self.add(
    //             *(
    //                 vmobject.get_subcurve(
    //                     np.interp(
    //                         dash_starts[i] * curve_length,
    //                         length_vals,
    //                         ref_points,
    //                     ),
    //                     np.interp(
    //                         dash_ends[i] * curve_length,
    //                         length_vals,
    //                         ref_points,
    //                     ),
    //                 )
    //                 for i in range(len(dash_starts))
    //             )
    //         )
    //     else:
    //         self.add(
    //             *(
    //                 vmobject.get_subcurve(
    //                     dash_starts[i],
    //                     dash_ends[i],
    //                 )
    //                 for i in range(len(dash_starts))
    //             )
    //         )
    let mut result = VectorObject::new();
    let r = dashed_ratio.unwrap_or(0.5);
    let n = num_dashes.unwrap_or(15);
    let dash_offset = dash_offset.unwrap_or(0.0);
    let equal_lengths = equal_lengths.unwrap_or(true);
    if n > 0 {
        let dash_len = r / n as f64;
        let void_len = if vector_object.is_closed() {
            (1.0 - r) / n as f64
        } else {
            if n == 1 {
                1.0 - r
            } else {
                (1.0 - r) / (n - 1) as f64
            }
        };
        let period = dash_len + void_len;
        let phase_shift = (dash_offset % 1.0) * period;
        let pattern_len = if vector_object.is_closed() {
            1.0
        } else {
            1.0 + void_len
        };
        let mut dash_starts = vec![];
        let mut dash_ends = vec![];
        for i in 0..n {
            dash_starts.push((i as f64 * period + phase_shift) % pattern_len);
            dash_ends.push((i as f64 * period + dash_len + phase_shift) % pattern_len);
        }
        if !vector_object.is_closed() {
            if dash_ends[dash_ends.len() - 1] > 1.0 && dash_starts[dash_starts.len() - 1] > 1.0 {
                dash_ends.pop();
                dash_starts.pop();
            } else if dash_ends[dash_ends.len() - 1] < dash_len {
                if dash_starts[dash_starts.len() - 1] < 1.0 {
                    dash_starts.push(0.0);
                    dash_ends.push(dash_ends[dash_ends.len() - 1]);
                    let i = dash_ends.len() - 2;
                    dash_ends[i] = 1.0;
                } else {
                    let i = dash_starts.len() - 1;
                    dash_starts[i] = 0.0;
                }
            } else if dash_starts[dash_starts.len() - 1] > (1.0 - dash_len) {
                let i = dash_ends.len() - 1;
                dash_ends[i] = 1.0;
            }
        }
        if equal_lengths {
            let mut norms = Vec::new();
            for k in 0..vector_object.get_num_curves() {
                norms.push(vector_object.get_nth_curve_length_pieces(k, None));
            }
            let length_vals: Vec<f64> = norms.iter().flatten().cloned().collect();
            let ref_points = (0..length_vals.len()).map(|i| i as f64 / (length_vals.len() - 1) as f64).collect::<Vec<f64>>();
            let curve_length = length_vals[length_vals.len() - 1];
            for i in 0..dash_starts.len() {
                result = result.add(
                    &vector_object.get_subcurve(
                        interp(dash_starts[i] * curve_length, &length_vals, &ref_points),
                        interp(dash_ends[i] * curve_length, &length_vals, &ref_points),
                    )
                );
            }
        } else {
            for i in 0..dash_starts.len() {
                result = result.add(
                    &vector_object.get_subcurve(
                        dash_starts[i],
                        dash_ends[i],
                    )
                );
            }
        }
    }
    result = result.match_style(vector_object);
    result
}