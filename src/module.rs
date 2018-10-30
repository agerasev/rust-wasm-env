extern {
    fn js_mod_load(path_ptr: *const u8, path_len: usize);
    fn js_mod_call(mod_id: u32, func_ptr: *const u8, func_len: usize) -> i32;
    fn js_mod_check(mod_id: u32) -> i32;
}

pub fn load(path: &str) {
    unsafe { js_mod_load(path.as_ptr(), path.len()); }
}

pub fn check_loaded(id: u32) -> bool {
    unsafe { js_mod_check(id) == 0 }
}

#[derive(Debug)]
pub enum CallError {
    NoMod,
    NoFn,
    FnErr,
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
    pub fn call(&mut self, func_name: &str) -> Result<(),CallError> {
        let ret = unsafe { js_mod_call(
            self.id(), func_name.as_ptr(), func_name.len()
        ) };
        match ret {
            0 => Ok(()),
            1 => Err(CallError::NoMod),
            2 => Err(CallError::NoFn),
            3 => Err(CallError::FnErr),
            _ => panic!("unknown call status"),
        }
    }
} 
