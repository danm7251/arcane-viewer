use wasm_bindgen::prelude::*;
use web_sys::js_sys;

macro_rules! arcana {
    ($( ($img_src:expr, $number:expr, $title:expr) ),* $(,)?) => {
        vec![
            $(Arcana::new($img_src.into(), $number, $title.into())),*
        ]
    };
}

#[wasm_bindgen]
pub fn get_arcana() -> js_sys::Array {
    let cards: Vec<Arcana> = arcana!(
        ("images/arcanum/1.webp", 1, "The Magician"),
        ("images/arcanum/2.webp", 2, "The High Priestess"),
        ("images/arcanum/3.webp", 3, "The Empress"),
        ("images/arcanum/4.webp", 4, "The Emperor"),
        ("images/arcanum/5.webp", 5, "The Hierophant"),
    );
    cards.into_iter().map(JsValue::from).collect()
}

#[wasm_bindgen]
pub struct Arcana {
    img_src: String,
    number: u8,
    title: String,
}

#[wasm_bindgen]
impl Arcana {
    #[wasm_bindgen(constructor)]
    pub fn new(img_src: String, number: u8, title: String) -> Self {
        Self {
            img_src,
            number,
            title,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn img_src(&self) -> String {
        self.img_src.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn number(&self) -> String {
        self.number.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn title(&self) -> String {
        self.title.clone()
    }
}