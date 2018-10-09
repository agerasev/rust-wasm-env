//use std::mem::size_of;

pub static EVENT_DATA_SIZE: usize = 0x1000;

pub enum Event {
    Timeout { dt: f64 },
    Step { dt: f64 },
    Render,
}

macro_rules! event_read {
    ($Enum:ident, $ptr:expr, { $( $k:ident: $v:ident ),* }) => {
        $Enum {
            $(
                $k: {
                    
                },
            )*
        }
    }
}

impl Event {
    pub fn from(code: u32, data: &Vec<u8>) -> Option<Self> {
        match code {
            0x01 => unsafe {
                Some(Event::Timeout {
                    dt: *((data.as_ptr() + 0)
                })
            }
        }
    }
}
