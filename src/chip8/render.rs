use crate::chip8::screen::Screen;
use crate::webgl;
use crate::webgl::buffer;
use crate::webgl::shader;
use crate::webgl::texture;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext;
use web_sys::WebGlTexture;

pub struct Renderer {
    context: WebGlRenderingContext,
    texture: WebGlTexture,
}

impl Renderer {
    pub fn new() -> Result<Renderer, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

        let context = canvas
            .get_context("webgl")?
            .expect("failed to get webgl context")
            .dyn_into::<WebGlRenderingContext>()?;

        let vert_shader = shader::compile_shader(
            &context,
            WebGlRenderingContext::VERTEX_SHADER,
            r#"
                attribute vec4 position;
                varying vec2 texCoords;

                void main() {
                    texCoords = (position.xy + 1.0) / 2.0;
                    gl_Position = position;
                }
            "#,
        )?;
        let frag_shader = shader::compile_shader(
            &context,
            WebGlRenderingContext::FRAGMENT_SHADER,
            r#"
                precision highp float;

                uniform sampler2D sampler;
                varying vec2 texCoords;

                void main() {
                    gl_FragColor = texture2D(sampler, vec2(texCoords.x, -texCoords.y));
                }
            "#,
        )?;
        let program = webgl::shader::link_program(&context, [vert_shader, frag_shader].iter())?;
        context.use_program(Some(&program));

        let texture = webgl::texture::create_texture(&context)?;
        context.active_texture(WebGlRenderingContext::TEXTURE0);
        texture::update_texture(&context, &texture, 64, 32, &vec![0; 64 * 32])?;
        texture::disable_mipmapping(&context);

        let texture_location = context.get_uniform_location(&program, "sampler");
        context.uniform1i(texture_location.as_ref(), 0);

        let vertices: [f32; 18] = [
            -1.0, -1.0, 0.0, -1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, -1.0, 0.0, -1.0,
            -1.0, 0.0,
        ];
        let vertex_buffer = buffer::create_buffer(&context).ok_or("failed to create buffer")?;
        buffer::vertex_buffer_data(&context, &vertex_buffer, &vertices)?;

        context.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
        context.enable_vertex_attrib_array(0);

        context.clear_color(1.0, 1.0, 0.0, 1.0);
        Ok(Renderer {
            context: context,
            texture: texture,
        })
    }

    pub fn render(&self, screen: &Screen) {
        let data = screen.get_screen_data();
        texture::update_texture(&self.context, &self.texture, 64, 32, &data.to_vec()).expect("");

        self.context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
        self.context
            .draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 6);
    }
}
