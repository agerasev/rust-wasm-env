extern crate byteorder;
#[macro_use]
extern crate lazy_static;
extern crate vecmat;

pub mod console;
pub mod canvas;
pub mod types;
pub mod event;
pub mod module;

use std::cell::Cell;
use std::sync::Mutex;
use std::collections::{VecDeque, BTreeMap};

use event::Event;

extern {
    #[allow(dead_code)]
    fn js_timeout(sec: f64);
    fn js_crypto_random(ptr: *mut u8, len: usize);
    fn js_request_frame();
    fn js_drop_object(id: u32) -> i32;
}

pub trait App {
    fn handle(&mut self, event: Event);
}

pub fn request_frame() {
    unsafe { js_request_frame(); }
}

pub fn seed(slice: &mut [u8]) {
    unsafe { js_crypto_random(slice.as_mut_ptr(), slice.len()); }
}

pub unsafe fn drop_object(id: u32) {
    if js_drop_object(id) != 0 {
        panic!("there is no object with id: {}", id);
    }
}

struct WasmData {
    heap: BTreeMap<usize, Vec<u8>>,
    events: VecDeque<Event>,
}

impl WasmData {
    fn new() -> Self {
        Self { 
            heap: BTreeMap::new(), 
            events: VecDeque::new(),
        }
    }
}

lazy_static! {
    static ref WASM: Mutex<WasmData> = Mutex::new(WasmData::new());
}

thread_local! {
    static INSIDE: Cell<bool> = Cell::new(false);
}

pub fn _alloc(size: usize) -> *mut u8 {
    let mut vec = vec!(0 as u8; size);
    let ptr = vec.as_mut_ptr();
    let key = ptr as usize;

    let mut wasm = WASM.lock().unwrap();
    let heap = &mut wasm.heap;
    if heap.contains_key(&key) {
        panic!("Address {} already occupied in the heap", key);
    }
    heap.insert(key, vec);
    
    ptr
}

pub fn _free(ptr: *mut u8) {
    let key = ptr as usize;
    let mut wasm = WASM.lock().unwrap();
    let heap = &mut wasm.heap;
    if !heap.contains_key(&key) {
        panic!("Address {} doesn't exist in the heap", key);
    }
    heap.remove(&key);
}

pub fn _handle(app: &Mutex<Box<App+Send>>, ptr: *mut u8) {
    {
        let key = ptr as usize;
        let mut wasm = WASM.lock().unwrap();
        let event = match wasm.heap.get(&key) {
            Some(vec) => Event::from(vec).unwrap(),
            None => panic!("Invalid buffer address {} in the heap", key),
        };
        wasm.events.push_front(event);
    };
    INSIDE.with(|inside| {
        if !inside.get() {
            inside.set(true);
            loop {
                if let Some(event) = WASM.lock().unwrap().events.pop_back() {
                    app.lock().unwrap().handle(event);
                } else {
                    break;
                }
            }
            inside.set(false);
        }
    });
}

#[macro_export]
macro_rules! wasm_bind {
    ($wasm:ident, $appfn:expr) => (
        lazy_static! {
            static ref APP: std::sync::Mutex<Box<$wasm::App+Send>> = std::sync::Mutex::new($appfn());
        }

        #[no_mangle]
        pub extern fn init() {
            $wasm::console::setup();
        }

        #[no_mangle]
        pub extern fn alloc(size: usize) -> *mut u8 {
            $wasm::_alloc(size)
        }

        #[no_mangle]
        pub extern fn free(ptr: *mut u8) {
            $wasm::_free(ptr)
        }

        #[no_mangle]
        pub extern fn handle(ptr: *mut u8) {
            $wasm::_handle(&APP, ptr);
        }
    )
}
