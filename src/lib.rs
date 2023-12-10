use wasm_bindgen::prelude::*;
use web_sys::{console, ImageData};
use wasm_bindgen::Clamped;
use std::mem::MaybeUninit;
use core::ptr;
//use std::ptr;
//use std::{f64, borrow::BorrowMut};

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let canvas1 = document.get_element_by_id("canvas").unwrap();
    unsafe{
        //CANVAS.as_mut_ptr().write(canvas1.dyn_into::<web_sys::HtmlCanvasElement>().map_err(|_| ()).unwrap());
        //CONTEXT.as_mut_ptr().write(CANVAS.as_ptr().read().get_context("2d").unwrap().unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap());

        
        CANVAS = MaybeUninit::new(canvas1.dyn_into::<web_sys::HtmlCanvasElement>().map_err(|_| ()).unwrap());
        CONTEXT = MaybeUninit::new(CANVAS.as_ptr().read().get_context("2d").unwrap().unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap());
        // load it in, in a default state
        redraw_canvas();
    }
    Ok(())
}

// global variables

static mut CANVAS: MaybeUninit<web_sys::HtmlCanvasElement> = MaybeUninit::uninit();
static mut CONTEXT: MaybeUninit<web_sys::CanvasRenderingContext2d> = MaybeUninit::uninit();



// have whatever parameters inserted there
pub unsafe fn redraw_canvas(){
    
    let canvas_ptr = CANVAS.as_mut_ptr();
    let context_ptr = CONTEXT.as_mut_ptr();
    // test whether either are uninitialized
    if canvas_ptr.is_null() || context_ptr.is_null(){
        console::log_1(&JsValue::from_str("canvas/context uninitalized when drawing!!"));
        return
    }

    let canvas = ptr::read(canvas_ptr);
    let context = ptr::read(context_ptr);

    //let canvas = CANVAS.assume_init_ref();
    //let context = CONTEXT.assume_init_ref();

    let canvas_width = canvas.width();
    let canvas_height = canvas.height();
    let img_data = ImageData::new_with_sw(canvas_width, canvas_height).unwrap();

    let mut pixels = img_data.data();

    for x in 0..canvas_width{
        for y in 0..canvas_height{
            let byte_offset = (((y * img_data.width()) + x) * 4) as usize;
            pixels[byte_offset] = 255u8;
            pixels[byte_offset+1] = 255u8;
            pixels[byte_offset+2] = 255u8;
            pixels[byte_offset+3] = 255u8;
    }}

    let slice_data = Clamped(&pixels.0[..]);
    let img_data_edited = ImageData::new_with_u8_clamped_array(slice_data, canvas_width).unwrap();
    let among: Result<(), JsValue> = context.put_image_data(&img_data_edited, 0.0, 0.0);
}