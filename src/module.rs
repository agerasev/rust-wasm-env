extern {
    fn js_mod_load(path_ptr: *const u8, path_len: usize);
    fn js_mod_call(mod_ptr: *const u8, mod_len: usize, func_ptr: *const u8, func_len: usize) -> i32;
    fn js_mod_check(mod_ptr: *const u8, mod_len: usize) -> i32;
}

pub fn load(path: &str) {
    unsafe { js_mod_load(path.as_ptr(), path.len()); }
}

pub fn check_loaded(name: &str) -> bool {
    unsafe { js_mod_check(name.as_ptr(), name.len()) == 0 }
}

#[derive(Debug)]
pub enum CallError {
    NoMod,
    NoFn,
    FnErr,
}

#[derive(Debug)]
pub struct Module {
    path_: String,
}

impl Module {
    pub fn new(path: String) -> Result<Module, ()> {
        if check_loaded(&path) {
            Ok(Module { path_: path })
        } else {
            Err(())
        }
    }
    pub fn path(&self) -> &str {
        &self.path_
    }
    pub fn call(&mut self, func_name: &str) -> Result<(),CallError> {
        let mod_name = self.path();
        let ret = unsafe { js_mod_call(
            mod_name.as_ptr(), mod_name.len(),
            func_name.as_ptr(), func_name.len()
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
