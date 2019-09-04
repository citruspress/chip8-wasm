use wasm_bindgen::JsValue;
use web_sys::WebGlRenderingContext;
use web_sys::WebGlTexture;

pub fn disable_mipmapping(context: &WebGlRenderingContext) {
    context.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_MIN_FILTER,
        WebGlRenderingContext::NEAREST as i32,
    );
    context.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_MAG_FILTER,
        WebGlRenderingContext::NEAREST as i32,
    );
}

pub fn create_texture(context: &WebGlRenderingContext) -> Result<WebGlTexture, JsValue> {
    let texture = context
        .create_texture()
        .ok_or("Failed to create texture.")?;
    Ok(texture)
}

pub fn update_texture(
    context: &WebGlRenderingContext,
    texture: &WebGlTexture,
    width: i32,
    height: i32,
    data: &Vec<u8>,
) -> Result<(), JsValue> {
    context.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&texture));
    context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        WebGlRenderingContext::TEXTURE_2D,       // target
        0,                                       // mipmap
        WebGlRenderingContext::LUMINANCE as i32, // format
        width,                                   // width
        height,                                  // height
        0,                                       // border
        WebGlRenderingContext::LUMINANCE,
        WebGlRenderingContext::UNSIGNED_BYTE,
        Some(data),
    )?;

    Ok(())
}
