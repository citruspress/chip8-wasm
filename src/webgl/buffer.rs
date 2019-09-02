use js_sys::WebAssembly;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::WebGlBuffer;
use web_sys::WebGlRenderingContext;

pub fn create_buffer(context: &WebGlRenderingContext) -> Option<WebGlBuffer> {
    context.create_buffer()
}

pub fn vertex_buffer_data(
    context: &WebGlRenderingContext,
    buffer: &WebGlBuffer,
    data: &[f32],
) -> Result<(), JsValue> {
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
    unsafe {
        let array = js_sys::Float32Array::view(&data);

        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }

    Ok(())
}
