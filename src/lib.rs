use wasm_bindgen::prelude::*;
use web_sys::{console, ImageData};
use wasm_bindgen::Clamped;
use std::mem::MaybeUninit;

//use noise::{NoiseFn, SuperSimplex, Seedable};
use fastnoise_lite::*;

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
        
        // init noise
        NOISE = MaybeUninit::new(FastNoiseLite::new());
        let noise = NOISE.assume_init_mut();
        noise.set_noise_type(Some(NoiseType::OpenSimplex2S));

        // load it in, in a default state
        //redraw_canvas(0.0, 0.0, 0.0, 0.005, 8, 2.0, 0.5, 1000);
    }
    Ok(())
}



// global/cached variables
static mut CANVAS: MaybeUninit<web_sys::HtmlCanvasElement> = MaybeUninit::uninit();
static mut CONTEXT: MaybeUninit<web_sys::CanvasRenderingContext2d> = MaybeUninit::uninit();
//static mut NOISE: MaybeUninit<SuperSimplex> = MaybeUninit::uninit();
static mut NOISE:MaybeUninit<fastnoise_lite::FastNoiseLite> = MaybeUninit::uninit(); 

// have whatever parameters inserted there
#[wasm_bindgen]
pub unsafe fn redraw_canvas(pos_x:f64, pos_y:f64, pos_z:f64, _frequency_max:f64, _octaves:f64, _lacunarity:f64, _persistence:f64, seed:f64){
    // test whether either are uninitialized
    if CANVAS.as_mut_ptr().is_null() || CONTEXT.as_mut_ptr().is_null(){
        console::log_1(&JsValue::from_str("canvas/context uninitalized when drawing!!"));
        return
    }
    //if NOISE.as_mut_ptr().is_null(){
    //    console::log_1(&JsValue::from_str("canvas/context uninitalized when drawing!!"));
    //    return
    //}
    let canvas = CANVAS.assume_init_ref();
    let context = CONTEXT.assume_init_ref();

    let canvas_width = canvas.width();
    let canvas_height = canvas.height();

    // process in all the new parameters
    frequency_max = _frequency_max;
    octaves = _octaves as i32;
    lacunarity = _lacunarity;
    persistence = _persistence;
    max_height = (1.0 + (persistence * 2.0)) - persistence.powi(octaves - 1); // blessed be the brain, unblessed be the language
    // apply seed to noise
    //NOISE = MaybeUninit::new(SuperSimplex::new(seed as u32));
    let noise_gen = NOISE.assume_init_mut();
    noise_gen.set_seed(Some(seed as i32));

    // old method
    //let img_data = ImageData::new_with_sw(canvas_width, canvas_height).unwrap();
    //let mut pixels = img_data.data();
    let mut new_pixels: Vec<u8> = vec![0; (canvas_width * canvas_height * 4) as usize];
    let mut vartest: u32 = 0;
    let mut vartest2 = 0.0;
    for x in 0..canvas_width{
        for y in 0..canvas_height{
            let byte_offset = (((y * canvas_width) + x) * 4) as usize;
            let noise_float: f64 = get_noise_at(pos_x + f64::from(x), pos_y +f64::from(y), pos_z);
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

    let slice_data = Clamped(&new_pixels[..]);
    let img_data_edited = ImageData::new_with_u8_clamped_array(slice_data, canvas_width).unwrap();
    let among: Result<(), JsValue> = context.put_image_data(&img_data_edited, 0.0, 0.0);
    console::log_1(&JsValue::from_str("Image has been drawn!"));
}



static mut frequency_max:f64 = 0.005; // the scale of the noise
static mut octaves:i32 = 8; // amount of layers of noise
static mut lacunarity:f64 = 2.0; // frequency increase per octaves
static mut persistence:f64 = 0.5; // height influence decrease per octave

static mut max_height:f64 = 1.0; 

fn get_noise_at(x:f64, y:f64, z:f64) -> f64{
    //let noise_gen = unsafe { NOISE.assume_init_ref()};
    let noise_gen = unsafe { NOISE.assume_init_mut()};

    let mut curr_noise = 0.0;
    let mut amplitude = 1.0;
    unsafe{
        let mut frequency = frequency_max;
        for octave in 0..octaves{
            let sample_x = x * frequency;
            let sample_y = y * frequency;
            let sample_z = z * frequency;
            //curr_noise += ((noise_gen.get([sample_x,sample_y,sample_z]) + 1.0) / 2.0) * amplitude;
            curr_noise += ((noise_gen.get_noise_3d(sample_x as f32,sample_y as f32,sample_z as f32) + 1.0) / 2.0) as f64 * amplitude;
            //curr_noise += 1.0;
            frequency *= lacunarity;
            amplitude *= persistence;
        }
        return (curr_noise/max_height).clamp(0.0, 1.0); // we clamp just in case
}}