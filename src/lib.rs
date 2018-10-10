extern crate byteorder;
#[macro_use]
extern crate lazy_static;
extern crate vecmat;

pub mod console;
pub mod canvas;
pub mod interop;

use std::sync::Mutex;

pub use interop::Event;

pub trait App {
    fn handle(&mut self, event: Event);
}

extern {
    #[allow(dead_code)]
    fn js_timeout(sec: f64);
    fn js_crypto_random(ptr: *mut u8, len: i32);
}

pub fn seed(slice: &mut [u8]) {
    unsafe { js_crypto_random(slice.as_mut_ptr(), slice.len() as i32); }
}

lazy_static! {
    static ref EVENT_DATA: Mutex<Vec<u8>> = Mutex::new(vec!(0; interop::EVENT_DATA_SIZE));
}

pub fn _event_data_ptr() -> *mut u8 {
    EVENT_DATA.lock().unwrap().as_mut_ptr()
}

pub fn _handle(app: &mut Box<App+Send>, code: u32) {
    let mut guard = EVENT_DATA.lock().unwrap();
    let data = guard.as_mut();
    let event = Event::from(code, data).unwrap();
    app.handle(event);
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
            $wasm::_event_data_ptr()
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
