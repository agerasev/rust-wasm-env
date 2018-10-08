#[macro_use]
extern crate lazy_static;
extern crate vecmat;

pub mod console;
pub mod canvas;

pub trait App {
    fn step(&mut self, dt: f64);
    fn render(&mut self);
}

extern {
    #[allow(dead_code)]
    fn js_timeout(sec: f64);
    fn js_crypto_random(ptr: *mut u8, len: i32);
    fn js_get_event_data(ptr: *mut u8, len: i32);
}

pub fn seed(slice: &mut [u8]) {
    unsafe { js_crypto_random(slice.as_mut_ptr(), slice.len() as i32); }
}

pub fn handle(app: &mut App, code: u32) {
    if code == 0x0101 {
        console::log(&format!("timeout"));
    } else if code == 0x0102 {
        let mut data = [0 as u8; 8];
        unsafe { js_get_event_data(data.as_mut_ptr(), data.len() as i32); }
        let dt = unsafe { *(data.as_ptr() as *const f64) };
        app.step(dt);
    } else if code == 0x0103 {
        app.render();
    } else {
        console::error(&format!("unknown event code: {}", code));
    }
}

lazy_static! {
    static ref APP: std::sync::Mutex<Option<Box<App + Send>>> = std::sync::Mutex::new(None);
}

#[no_mangle]
pub extern fn init() {
    console::setup();
    let mut guard = APP.lock().unwrap();
    match *guard {
        None => { *guard = Some($App::new()); },
        Some(_) => { console::error("App is already initialized!"); },
    }
}

#[no_mangle]
pub extern fn handle(code: u32) {
    let mut guard = APP.lock().unwrap();
    let app = guard.as_mut().unwrap();
    handle(app as &mut App, code);
}

#[no_mangle]
pub extern fn quit() {
    let mut guard = APP.lock().unwrap();
    match *guard {
        None => { console::error("App is already None!"); },
        Some(_) => { *guard = None; },
    }
}

#[macro_export]
macro_rules! bind_wasm {
    ($App:expr) => (
        
    )
}
