#[macro_use]
extern crate lazy_static;
extern crate vecmat;

pub mod console;
pub mod canvas;
pub mod interface;

use std::sync::Mutex;

pub trait App {
    fn handle(&mut self, event: Event);
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

lazy_static! {
    static ref EVENT_DATA: Mutex<Vec<u8>> = Mutex::new(vec!(0; interface::EVENT_DATA_SIZE));
}

pub fn _handle(app: &mut Box<App+Send>, code: u32) {
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

#[macro_export]
macro_rules! wasm_bind {
    ($wasm:ident, $appfn:expr) => (
        lazy_static! {
            static ref APP: std::sync::Mutex<Option<Box<$wasm::App+Send>>> = std::sync::Mutex::new(None);
        }

        #[no_mangle]
        pub extern fn init() {
            $wasm::console::setup();
            let mut guard = APP.lock().unwrap();
            match *guard {
                None => { *guard = Some($appfn()); },
                Some(_) => { $wasm::console::error("App is already initialized!"); },
            }
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
