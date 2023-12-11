use wasm_bindgen::prelude::*;
use web_sys::{console, ImageData};
use wasm_bindgen::Clamped;
use std::mem::MaybeUninit;

use noise::{NoiseFn, Perlin, Seedable};

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world from rust!")); // test output to verify basic functionality

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let canvas1 = document.get_element_by_id("canvas").unwrap();
    unsafe{
        CANVAS = MaybeUninit::new(canvas1.dyn_into::<web_sys::HtmlCanvasElement>().map_err(|_| ()).unwrap());
        let canvas = CANVAS.assume_init_ref(); // apparently .as_ptr().read() destroys the reference?
        CONTEXT = MaybeUninit::new(canvas.get_context("2d").unwrap().unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap());
        // setup perlin noise
        PERLIN = MaybeUninit::new(Perlin::new(15125324));
        // load it in, in a default state
        redraw_canvas(0.0, 0.0, 0.0);
    }
    Ok(())
}



// global/cached variables
static mut CANVAS: MaybeUninit<web_sys::HtmlCanvasElement> = MaybeUninit::uninit();
static mut CONTEXT: MaybeUninit<web_sys::CanvasRenderingContext2d> = MaybeUninit::uninit();
static mut PERLIN: MaybeUninit<Perlin> = MaybeUninit::uninit();

// have whatever parameters inserted there
#[wasm_bindgen]
pub unsafe fn redraw_canvas(pos_x:f64, pos_y:f64, pos_z:f64){
    // test whether either are uninitialized
    if CANVAS.as_mut_ptr().is_null() || CONTEXT.as_mut_ptr().is_null(){
        console::log_1(&JsValue::from_str("canvas/context uninitalized when drawing!!"));
        return
    }
    if PERLIN.as_mut_ptr().is_null(){
        console::log_1(&JsValue::from_str("canvas/context uninitalized when drawing!!"));
        return
    }
    let canvas = CANVAS.assume_init_ref();
    let context = CONTEXT.assume_init_ref();

    let canvas_width = canvas.width();
    let canvas_height = canvas.height();

    // old method
    //let img_data = ImageData::new_with_sw(canvas_width, canvas_height).unwrap();
    //let mut pixels = img_data.data();
    let mut new_pixels: Vec<u8> = vec![0; (canvas_width * canvas_height * 4) as usize];
    let mut vartest: u32 = 0;
    let mut vartest2 = 0.0;
    for x in 0..canvas_width{
        for y in 0..canvas_height{
            let byte_offset = (((y * canvas_width) + x) * 4) as usize;
            let noise_float: f64 = get_noise_at(f64::from(x),f64::from(y), 0.0);
            let noise_value = (noise_float * 255.0) as u8;

            vartest += noise_value as u32;
            vartest2 += noise_float;

            new_pixels[byte_offset] = noise_value;
            new_pixels[byte_offset+1] = noise_value as u8;
            new_pixels[byte_offset+2] = noise_value as u8;
            new_pixels[byte_offset+3] = 255u8; // potentially unnecessary
    }}

    let values_debug = format!("truncated value: {}, regular value: {}", vartest, vartest2);
    console::log_1(&JsValue::from_str(&values_debug));

    let perlin = Perlin::new(1);
    let val1 = perlin.get([1.0, 0.0, 0.0]);
    let val2 = perlin.get([2.0, 1.3, 0.0]);
    let val3 = perlin.get([1.6, 1.0, 0.0]);
    let value_debug: String = format!("test: {}, {}, {}", val1, val2, val3);
    console::log_1(&JsValue::from_str(&value_debug));

    let slice_data = Clamped(&new_pixels[..]);
    let img_data_edited = ImageData::new_with_u8_clamped_array(slice_data, canvas_width).unwrap();
    let among: Result<(), JsValue> = context.put_image_data(&img_data_edited, 0.0, 0.0);
    console::log_1(&JsValue::from_str("Image has been drawn!"));
}



const noise_scale:f64 = 0.02;
fn get_noise_at(x:f64, y:f64, z:f64) -> f64{
    let perlin = unsafe { PERLIN.assume_init_ref()};
    return perlin.get([x*noise_scale+0.5,y*noise_scale+0.5,z*noise_scale+0.5]);
    //return 1.0;
}