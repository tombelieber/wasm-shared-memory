use wasm_bindgen::prelude::*;

// Define a struct with 10 fields.
// With #[repr(C)], the layout is predictable. On wasm32,
// the u32 (4 bytes) is followed by 4 bytes of padding to align the f64s.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MyObject {
    pub id: u32,    // 4 bytes
    pub value: f64, // 8 bytes
    pub a: f64,     // 8 bytes
    pub b: f64,     // 8 bytes
    pub c: f64,     // 8 bytes
    pub d: f64,     // 8 bytes
    pub e: f64,     // 8 bytes
    pub f: f64,     // 8 bytes
    pub g: f64,     // 8 bytes
    pub h: f64,     // 8 bytes
}

// Global storage for our objects.
static mut OBJECTS: Option<Vec<MyObject>> = None;

#[wasm_bindgen]
pub fn populate_objects() {
    unsafe {
        // Define a "template" for the constant f64 fields.
        let template = (
            42.0, // value
            1.1,  // a
            2.2,  // b
            3.3,  // c
            4.4,  // d
            5.5,  // e
            6.6,  // f
            7.7,  // g
            8.8,  // h
        );
        // Populate with 10,000 objects with id equal to the index.
        OBJECTS = Some(
            (0..10_000)
                .map(|i| MyObject {
                    id: i as u32,
                    value: template.0,
                    a: template.1,
                    b: template.2,
                    c: template.3,
                    d: template.4,
                    e: template.5,
                    f: template.6,
                    g: template.7,
                    h: template.8,
                })
                .collect(),
        );
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

/// Export the size (in bytes) of MyObject.
/// With the given layout:
/// - id: u32 → 4 bytes
/// - padding: 4 bytes (to align the following f64 fields)
/// - 9 × f64: 9 * 8 = 72 bytes
/// Total = 4 + 4 + 72 = 80 bytes.
#[wasm_bindgen]
pub fn get_object_size() -> usize {
    std::mem::size_of::<MyObject>()
}
