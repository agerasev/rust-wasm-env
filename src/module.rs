extern {
    fn js_mod_load(path_ptr: *const u16, path_len: usize);
    fn js_mod_call(
        mod_id: u32,
        fname_ptr: *const u16,
        fname_len: usize,
        buffer_ptr: *mut u8,
        buffer_len: usize,
    ) -> i32;
    fn js_mod_check(mod_id: u32) -> i32;
}

pub fn load(path: &str) {
    let path16: Vec<u16> = path.encode_utf16().collect();
    unsafe { js_mod_load(path16.as_ptr(), path16.len()); }
}

pub fn check_loaded(id: u32) -> bool {
    unsafe { js_mod_check(id) == 0 }
}

#[derive(Debug)]
pub enum CallError {
    NoMod,
    NoFn,
    FnErr,
    BufErr,
}

#[derive(Debug)]
pub struct Module {
    id_: u32,
}

impl Module {
    pub fn new(id: u32) -> Result<Module, ()> {
        if check_loaded(id) {
            Ok(Module { id_: id })
        } else {
            Err(())
        }
    }
    pub fn id(&self) -> u32 {
        self.id_
    }
    pub fn call(&mut self, func_name: &str, buffer: &mut [u8]) -> Result<(),CallError> {
        let fname16: Vec<u16> = func_name.encode_utf16().collect();
        let ret = unsafe { js_mod_call(
            self.id(),
            fname16.as_ptr(),
            fname16.len(),
            buffer.as_mut_ptr(),
            buffer.len(),
        ) };
        match ret {
            0 => Ok(()),
            1 => Err(CallError::NoMod),
            2 => Err(CallError::NoFn),
            3 => Err(CallError::FnErr),
            4 => Err(CallError::BufErr),
            _ => panic!("unknown call status"),
        }
    }
} 
