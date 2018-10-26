use std::io::Read;
use byteorder::{LE, ReadBytesExt};

pub static BUFFER_SIZE: usize = 0x1000;

#[derive(Debug)]
pub enum Event {
    Timeout { dt: f64 },
    Loaded { path: String, ok: bool },
    Step { dt: f64 },
    Render,
}

impl Event {
    pub fn from(code: u32, data: &Vec<u8>) -> Option<Self> {
        let mut r = data as &[u8];
        match code {
            0x01 => Some(Event::Timeout { 
                dt: r.read_f64::<LE>().unwrap()
            }),
            0x02 => Some(Event::Loaded { 
                path: {
                    let len = r.read_u32::<LE>().unwrap() as usize;
                    let mut buf = vec!(0 as u8; len);
                    r.read_exact(&mut buf).unwrap();
                    String::from_utf8(buf).unwrap()
                },
                ok: r.read_u8().unwrap() == 0
            }),
            0x41 => Some(Event::Step {
                dt: r.read_f64::<LE>().unwrap()
            }),
            0x42 => Some(Event::Render),
            _ => None,
        }
    }
}
