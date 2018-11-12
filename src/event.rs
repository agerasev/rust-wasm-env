use ::types::*;
use ::module::Module;

#[derive(Debug)]
pub enum Resource {}

#[derive(Debug)]
pub enum ModuleError {
    Load,
    Init,
}

#[derive(Debug)]
pub enum Event {
    Start,
    Timeout(f64),
    Loaded,
    Module { path: String, module: Result<Module, ModuleError> },
    Render { dt: f64 },
    User(Vec<u8>),
}

impl Event {
    pub fn from(data: &Vec<u8>) -> Option<Self> {
        let r = &mut (data as &[u8]);
        match u32::load(r).unwrap() {
            0x00 => Some(Event::Start),
            0x01 => Some(Event::Timeout(f64::load(r).unwrap())),
            0x02 => {
                /*
                let path = String::load(r).unwrap();
                let status = i32::load(r).unwrap();
                let id = u32::load(r).unwrap();
                let rtype = i32::load(r).unwrap();
                */
                Some(Event::Loaded)
            },
            0x03 => {
                let path = String::load(r).unwrap();
                let status = i32::load(r).unwrap();
                let id = u32::load(r).unwrap();
                let module = match status {
                    0 => Ok(Module::new(id).unwrap()),
                    1 => Err(ModuleError::Load),
                    2 => Err(ModuleError::Init),
                    _ => panic!("unknown resource load status: {}", status),
                };
                Some(Event::Module { path, module })
            },
            0x40 => Some(Event::Render {
                dt: f64::load(r).unwrap()
            }),
            0xFF => Some(Event::User(r.to_vec())),
            _ => None,
        }
    }
}
