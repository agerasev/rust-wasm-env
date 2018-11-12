use std;
use std::mem::size_of;
use std::io::{Read, Write};
use byteorder::{LE, ReadBytesExt, WriteBytesExt};


#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    FromUtf16(std::string::FromUtf16Error),
}

pub trait Type: Sized {
    fn size(&self) -> usize {
        size_of::<Self>()
    }
    fn load<R>(r: &mut R) -> Result<Self, Error> where R: Read;
    fn store<W>(&self, w: &mut W) -> Result<(),Error> where W: Write;
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
    fn store<W>(&self, _w: &mut W) -> Result<(),Error> where W: Write {
        Ok({})
    }
}

impl Type for i8 {
    fn load<R>(r: &mut R) -> Result<Self, Error> where R: Read {
        r.read_i8().map_err(|e| Error::Io(e))
    }
    fn store<W>(&self, w: &mut W) -> Result<(),Error> where W: Write {
        w.write_i8(*self).map_err(|e| Error::Io(e))
    }
}

impl Type for u8 {
    fn load<R>(r: &mut R) -> Result<Self, Error> where R: Read {
        r.read_u8().map_err(|e| Error::Io(e))
    }
    fn store<W>(&self, w: &mut W) -> Result<(),Error> where W: Write {
        w.write_u8(*self).map_err(|e| Error::Io(e))
    }
}
impl Type for i16 {
    fn load<R>(r: &mut R) -> Result<Self, Error> where R: Read {
        r.read_i16::<LE>().map_err(|e| Error::Io(e))
    }
    fn store<W>(&self, w: &mut W) -> Result<(),Error> where W: Write {
        w.write_i16::<LE>(*self).map_err(|e| Error::Io(e))
    }
}
impl Type for u16 {
    fn load<R>(r: &mut R) -> Result<Self, Error> where R: Read {
        r.read_u16::<LE>().map_err(|e| Error::Io(e))
    }
    fn store<W>(&self, w: &mut W) -> Result<(),Error> where W: Write {
        w.write_u16::<LE>(*self).map_err(|e| Error::Io(e))
    }
}
impl Type for i32 {
    fn load<R>(r: &mut R) -> Result<Self, Error> where R: Read {
        r.read_i32::<LE>().map_err(|e| Error::Io(e))
    }
    fn store<W>(&self, w: &mut W) -> Result<(),Error> where W: Write {
        w.write_i32::<LE>(*self).map_err(|e| Error::Io(e))
    }
}
impl Type for u32 {
    fn load<R>(r: &mut R) -> Result<Self, Error> where R: Read {
        r.read_u32::<LE>().map_err(|e| Error::Io(e))
    }
    fn store<W>(&self, w: &mut W) -> Result<(),Error> where W: Write {
        w.write_u32::<LE>(*self).map_err(|e| Error::Io(e))
    }
}
impl Type for f32 {
    fn load<R>(r: &mut R) -> Result<Self, Error> where R: Read {
        r.read_f32::<LE>().map_err(|e| Error::Io(e))
    }
    fn store<W>(&self, w: &mut W) -> Result<(),Error> where W: Write {
        w.write_f32::<LE>(*self).map_err(|e| Error::Io(e))
    }
}
impl Type for f64 {
    fn load<R>(r: &mut R) -> Result<Self, Error> where R: Read {
        r.read_f64::<LE>().map_err(|e| Error::Io(e))
    }
    fn store<W>(&self, w: &mut W) -> Result<(),Error> where W: Write {
        w.write_f64::<LE>(*self).map_err(|e| Error::Io(e))
    }
}
impl Type for isize {
    fn load<R>(r: &mut R) -> Result<Self, Error> where R: Read {
        match r.read_i32::<LE>() {
            Ok(x) => Ok(x as isize),
            Err(e) => Err(Error::Io(e))
        }
    }
    fn store<W>(&self, w: &mut W) -> Result<(),Error> where W: Write {
        w.write_i32::<LE>(*self as i32).map_err(|e| Error::Io(e))
    }
}
impl Type for usize {
    fn load<R>(r: &mut R) -> Result<Self, Error> where R: Read {
        match r.read_u32::<LE>() {
            Ok(x) => Ok(x as usize),
            Err(e) => Err(Error::Io(e))
        }
    }
    fn store<W>(&self, w: &mut W) -> Result<(),Error> where W: Write {
        w.write_u32::<LE>(*self as u32).map_err(|e| Error::Io(e))
    }
}
impl Type for String {
    fn size(&self) -> usize {
        size_of::<usize>() + self.len()
    }
    fn load<R>(r: &mut R) -> Result<Self, Error> where R: Read {
        let len = try!(r.read_u32::<LE>().map_err(|e| Error::Io(e))) as usize;
        let mut buf = vec!(0 as u16; len);
        try!(r.read_u16_into::<LE>(&mut buf).map_err(|e| Error::Io(e)));
        String::from_utf16(&buf).map_err(|e| Error::FromUtf16(e))
    }
    fn store<W>(&self, w: &mut W) -> Result<(),Error> where W: Write {
        try!(w.write_u32::<LE>(self.len() as u32).map_err(|e| Error::Io(e)));
        for ch in self.encode_utf16() {
            try!(w.write_u16::<LE>(ch).map_err(|e| Error::Io(e)));
        }
        Ok(())
    }
}
