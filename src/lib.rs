extern crate byteorder;
#[macro_use]
extern crate lazy_static;
extern crate vecmat;

pub mod console;
pub mod canvas;
pub mod interop;

use std::sync::Mutex;

pub use interop::Event;


extern {
    #[allow(dead_code)]
    fn js_timeout(sec: f64);
    fn js_crypto_random(ptr: *mut u8, len: usize);
    fn js_mod_load(id: u32, path_ptr: *const u8, path_len: usize);
    fn js_mod_call(mod_ptr: *const u8, mod_len: usize, func_ptr: *const u8, func_len: usize);
    fn js_mod_check(mod_ptr: *const u8, mod_len: usize) -> i32;
}

pub trait App {
    fn handle(&mut self, event: Event);
}

pub fn seed(slice: &mut [u8]) {
    unsafe { js_crypto_random(slice.as_mut_ptr(), slice.len()); }
}

pub fn mod_load(id: u32, mod_name: &str) {
    unsafe { js_mod_load(id, mod_name.as_ptr(), mod_name.len()); }
}
pub fn mod_call(mod_name: &str, func_name: &str) {
    unsafe { js_mod_call(mod_name.as_ptr(), mod_name.len(), func_name.as_ptr(), func_name.len()); }
}

lazy_static! {
    static ref BUFFER: Mutex<Vec<u8>> = Mutex::new(vec!(0; interop::BUFFER_SIZE));
}

pub fn _handle(app: &mut Box<App+Send>, code: u32) {
    let mut guard = BUFFER.lock().unwrap();
    let data = guard.as_mut();
    let event = Event::from(code, data).unwrap();
    app.handle(event);
}

pub fn _buffer_ptr() -> *mut u8 {
    BUFFER.lock().unwrap().as_mut_ptr()
}

#[macro_export]
macro_rules! wasm_bind {
    ($wasm:ident, $appfn:expr) => (
        lazy_static! {
            static ref APP: std::sync::Mutex<Option<Box<$wasm::App+Send>>> = std::sync::Mutex::new(None);
        }

        #[no_mangle]
        pub extern fn init() -> *mut u8 {
            $wasm::console::setup();
            let mut guard = APP.lock().unwrap();
            match *guard {
                None => { *guard = Some($appfn()); },
                Some(_) => { $wasm::console::error("App is already initialized!"); },
            }
            $wasm::_buffer_ptr()
        }

        #[no_mangle]
        pub extern fn handle(code: u32) {
            let mut guard = APP.lock().unwrap();
            let app = guard.as_mut().unwrap();
            $wasm::_handle(app, code);
        }

        #[no_mangle]
        pub extern fn quit() {
            let mut guard = APP.lock().unwrap();
            match *guard {
                None => { $wasm::console::error("App is already None!"); },
                Some(_) => { *guard = None; },
            }
        }
    )
}
