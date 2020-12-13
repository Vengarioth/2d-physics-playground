use wasm_bindgen::JsValue;

#[derive(Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r,
            g,
            b,
            a,
        }
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            r,
            g,
            b,
            a: std::u8::MAX,
        }
    }
}

impl Into<JsValue> for Color {
    fn into(self) -> JsValue {
        JsValue::from_str(&format!("rgba({}, {}, {}, {})", self.r, self.g, self.b, std::u8::MAX as f32 / self.a as f32))
    }
}
