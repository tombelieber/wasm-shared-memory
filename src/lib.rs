use wasm_bindgen::prelude::*;

// Ensure a predictable memory layout.
#[repr(C)]
#[derive(Debug)]
pub struct MyObject {
    pub id: u32,    // 4 bytes
    pub value: f64, // 8 bytes
}

// Store our objects in a global mutable vector.
static mut OBJECTS: Option<Vec<MyObject>> = None;

#[wasm_bindgen]
pub fn populate_objects() {
    unsafe {
        // Populate with some sample data.
        OBJECTS = Some(vec![
            MyObject { id: 1, value: 42.0 },
            MyObject { id: 2, value: 84.0 },
            MyObject {
                id: 3,
                value: 126.0,
            },
        ]);
    }
}

#[wasm_bindgen]
pub fn get_objects_ptr() -> *const MyObject {
    unsafe {
        OBJECTS
            .as_ref()
            .map(|vec| vec.as_ptr())
            .unwrap_or(std::ptr::null())
    }
}

#[wasm_bindgen]
pub fn get_objects_len() -> usize {
    unsafe { OBJECTS.as_ref().map(|vec| vec.len()).unwrap_or(0) }
}

#[wasm_bindgen]
pub fn get_memory() -> wasm_bindgen::JsValue {
    wasm_bindgen::memory()
}
