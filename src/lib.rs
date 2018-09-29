pub extern crate vecmat;

pub mod console;
pub mod canvas;

pub trait App {
    fn step(&mut self, dt: f64);
    fn render(&mut self);
    fn timeout(&mut self, dt: f64);
}

extern {
    #[allow(dead_code)]
    fn js_timeout(sec: f64);
    fn js_crypto_random(ptr: *mut u8, len: i32);
}

pub fn seed(slice: &mut [u8]) {
    unsafe { js_crypto_random(slice.as_mut_ptr(), slice.len() as i32); }
}

#[macro_export]
macro_rules! bind_wasm {
    ($App:ident, $wasm:ident) => (
        lazy_static! {
            static ref APP: std::sync::Mutex<Option<$App>> = std::sync::Mutex::new(None);
        }
        #[no_mangle]
        pub extern fn init() {
            $wasm::console::setup();
            let mut guard = APP.lock().unwrap();
            match *guard {
                None => { *guard = Some($App::new()); },
                Some(_) => { $wasm::console::error("App is already initialized!"); },
            }
        }
        
        #[no_mangle]
        pub extern fn timeout(dt: f64) {
            $wasm::console::log(&format!("timeout: {} sec", dt));
        }

        #[no_mangle]
        pub extern fn step(dt: f64) {
            let mut guard = APP.lock().unwrap();
            let app = guard.as_mut().unwrap();
            (app as &mut $wasm::App).step(dt);
        }

        #[no_mangle]
        pub extern fn render() {
            let mut guard = APP.lock().unwrap();
            let app = guard.as_mut().unwrap();
            (app as &mut $wasm::App).render();
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
