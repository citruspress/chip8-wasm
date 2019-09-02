use js_sys::Object;
use wasm_bindgen::JsValue;
use web_sys::Element;

pub enum WebGlError {
    JsValue(JsValue),
    Element(Element),
    Object(Object),
    String(String),
    FailedToCreateTextureError,
}

impl From<JsValue> for WebGlError {
    fn from(err: JsValue) -> WebGlError {
        WebGlError::JsValue(err)
    }
}

impl From<Element> for WebGlError {
    fn from(err: Element) -> WebGlError {
        WebGlError::Element(err)
    }
}

impl From<Object> for WebGlError {
    fn from(err: Object) -> WebGlError {
        WebGlError::Object(err)
    }
}

impl From<String> for WebGlError {
    fn from(err: String) -> WebGlError {
        WebGlError::String(err)
    }
}

impl From<&str> for WebGlError {
    fn from(err: &str) -> WebGlError {
        WebGlError::String(String::from(err))
    }
}
