use std;
use std::mem::size_of;
use std::io::{Read, Write};
use byteorder::{LE, ReadBytesExt, WriteBytesExt};

use module::Module;

pub static BUFFER_SIZE: usize = 0x1000;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    FromUtf8(std::string::FromUtf8Error),
    String(String),
}

pub trait Type: Sized {
    fn size(&self) -> usize {
        size_of::<Self>()
    }
    fn load<R>(r: &mut R) -> Result<Self, Error> where R: Read;
    fn store<W>(self, w: &mut W) -> Result<(),Error> where W: Write;
}

pub trait TypeRef: Type {
    fn load_ref<R>(&mut self, r: &mut R) -> Result<(), Error> where R: Read;
    fn store_ref<W>(&self, w: &mut W) -> Result<(),Error> where W: Write;
}

#[derive(Debug, Clone, Copy)]
struct Void {}

impl Type for Void {
    fn size(&self) -> usize {
        0
    }
    fn load<R>(_r: &mut R) -> Result<Self, Error> where R: Read {
        Ok(Void {})
    }
    fn store<W>(self, _w: &mut W) -> Result<(),Error> where W: Write {
        Ok({})
    }
}

impl Type for i8 {
    fn load<R>(r: &mut R) -> Result<Self, Error> where R: Read {
        r.read_i8().map_err(|e| Error::Io(e))
    }
    fn store<W>(self, w: &mut W) -> Result<(),Error> where W: Write {
        w.write_i8(self).map_err(|e| Error::Io(e))
    }
}

impl Type for u8 {
    fn load<R>(r: &mut R) -> Result<Self, Error> where R: Read {
        r.read_u8().map_err(|e| Error::Io(e))
    }
    fn store<W>(self, w: &mut W) -> Result<(),Error> where W: Write {
        w.write_u8(self).map_err(|e| Error::Io(e))
    }
}
impl Type for i16 {
    fn load<R>(r: &mut R) -> Result<Self, Error> where R: Read {
        r.read_i16::<LE>().map_err(|e| Error::Io(e))
    }
    fn store<W>(self, w: &mut W) -> Result<(),Error> where W: Write {
        w.write_i16::<LE>(self).map_err(|e| Error::Io(e))
    }
}
impl Type for u16 {
    fn load<R>(r: &mut R) -> Result<Self, Error> where R: Read {
        r.read_u16::<LE>().map_err(|e| Error::Io(e))
    }
    fn store<W>(self, w: &mut W) -> Result<(),Error> where W: Write {
        w.write_u16::<LE>(self).map_err(|e| Error::Io(e))
    }
}
impl Type for i32 {
    fn load<R>(r: &mut R) -> Result<Self, Error> where R: Read {
        r.read_i32::<LE>().map_err(|e| Error::Io(e))
    }
    fn store<W>(self, w: &mut W) -> Result<(),Error> where W: Write {
        w.write_i32::<LE>(self).map_err(|e| Error::Io(e))
    }
}
impl Type for u32 {
    fn load<R>(r: &mut R) -> Result<Self, Error> where R: Read {
        r.read_u32::<LE>().map_err(|e| Error::Io(e))
    }
    fn store<W>(self, w: &mut W) -> Result<(),Error> where W: Write {
        w.write_u32::<LE>(self).map_err(|e| Error::Io(e))
    }
}
impl Type for f32 {
    fn load<R>(r: &mut R) -> Result<Self, Error> where R: Read {
        r.read_f32::<LE>().map_err(|e| Error::Io(e))
    }
    fn store<W>(self, w: &mut W) -> Result<(),Error> where W: Write {
        w.write_f32::<LE>(self).map_err(|e| Error::Io(e))
    }
}
impl Type for f64 {
    fn load<R>(r: &mut R) -> Result<Self, Error> where R: Read {
        r.read_f64::<LE>().map_err(|e| Error::Io(e))
    }
    fn store<W>(self, w: &mut W) -> Result<(),Error> where W: Write {
        w.write_f64::<LE>(self).map_err(|e| Error::Io(e))
    }
}
impl Type for isize {
    fn load<R>(r: &mut R) -> Result<Self, Error> where R: Read {
        match r.read_i32::<LE>() {
            Ok(x) => Ok(x as isize),
            Err(e) => Err(Error::Io(e))
        }
    }
    fn store<W>(self, w: &mut W) -> Result<(),Error> where W: Write {
        w.write_i32::<LE>(self as i32).map_err(|e| Error::Io(e))
    }
}
impl Type for usize {
    fn load<R>(r: &mut R) -> Result<Self, Error> where R: Read {
        match r.read_u32::<LE>() {
            Ok(x) => Ok(x as usize),
            Err(e) => Err(Error::Io(e))
        }
    }
    fn store<W>(self, w: &mut W) -> Result<(),Error> where W: Write {
        w.write_u32::<LE>(self as u32).map_err(|e| Error::Io(e))
    }
}
impl Type for String {
    fn size(&self) -> usize {
        size_of::<usize>() + self.len()
    }
    fn load<R>(r: &mut R) -> Result<Self, Error> where R: Read {
        let len = try!(r.read_u32::<LE>().map_err(|e| Error::Io(e))) as usize;
        let mut buf = vec!(0 as u8; len);
        try!(r.read_exact(&mut buf).map_err(|e| Error::Io(e)));
        String::from_utf8(buf).map_err(|e| Error::FromUtf8(e))
    }
    fn store<W>(self, w: &mut W) -> Result<(),Error> where W: Write {
        self.store_ref(w)
    }
}

impl TypeRef for String {
    fn load_ref<R>(&mut self, r: &mut R) -> Result<(), Error> where R: Read {
        let res = Self::load(r);
        match res {
            Ok(x) => {
                self.clear();
                self.push_str(&x);
                Ok(())
            },
            Err(e) => Err(e),
        }
    }
    fn store_ref<W>(&self, w: &mut W) -> Result<(),Error> where W: Write {
        try!(w.write_u32::<LE>(self.len() as u32).map_err(|e| Error::Io(e)));
        match w.write(self.as_bytes()) {
            Ok(x) => {
                if x == self.len() {
                    Ok(())
                } else {
                    Err(Error::String(String::from("Cannot store all bytes")))
                }
            },
            Err(e) => Err(Error::Io(e))
        }
    }
}

#[derive(Debug)]
pub enum Resource {
    Module(Module),
}

#[derive(Debug)]
pub enum ResourceError {
    Load,
    ModuleInit,
}

#[derive(Debug)]
pub enum Event {
    Start,
    Timeout { dt: f64 },
    Loaded { path: String, resource: Result<Resource, ResourceError> },
    Render { dt: f64 },
}

impl Event {
    pub fn from(code: u32, data: &Vec<u8>) -> Option<Self> {
        let mut r = data as &[u8];
        match code {
            0x00 => Some(Event::Start),
            0x01 => Some(Event::Timeout { 
                dt: f64::load(&mut r).unwrap()
            }),
            0x02 => {
                let path = String::load(&mut r).unwrap();
                let status = i32::load(&mut r).unwrap();
                let resource = match status {
                    0 => Ok(Resource::Module(Module::new(path.clone()).unwrap())),
                    1 => Err(ResourceError::Load),
                    2 => Err(ResourceError::ModuleInit),
                    _ => panic!("unknown resource load status: {}", status),
                };
                Some(Event::Loaded { path, resource })
            },
            0x40 => Some(Event::Render {
                dt: f64::load(&mut r).unwrap()
            }),
            _ => None,
        }
    }
}
