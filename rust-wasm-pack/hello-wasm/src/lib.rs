use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
  // js function
  pub fn alert(s: &str);
}
#[wasm_bindgen]
pub fn greet(name: &str) {
  
  alert(&format!("Hello {}", name));

}
