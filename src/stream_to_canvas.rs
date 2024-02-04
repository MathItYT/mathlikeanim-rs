use wasm_bindgen::{prelude::*, Clamped};


#[wasm_bindgen]
pub fn stream_to_canvas(arr: &[u8], width: usize, height: usize, context: &web_sys::CanvasRenderingContext2d) {
    let image_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(arr),
        width as u32,
        height as u32,
    ).unwrap();

    context.put_image_data(&image_data, 0.0, 0.0).unwrap();
}