use std::panic;

extern {
    fn js_console(t: i32, ptr: *const u16, len: usize);
}

pub enum Kind {
	Log,
	Error
}

pub fn write(t: Kind, msg: &str) {
    let msg16: Vec<u16> = msg.encode_utf16().collect();
    unsafe { js_console(match t { Kind::Log => 0, Kind::Error => 1}, msg16.as_ptr(), msg16.len()); }
}

pub fn log(msg: &str) {
    write(Kind::Log, msg);
}

pub fn error(msg: &str) {
    write(Kind::Error, msg);
}

pub fn setup() {
	panic::set_hook(Box::new(|panic_info| {
        let payload = panic_info.payload();
        let payload = match payload.downcast_ref::<String>() {
            Some(payload) => payload.clone(),
            None => match payload.downcast_ref::<&str>() {
                Some(payload) => String::from(*payload),
                None => String::new(),
            },
        };
        let location = match panic_info.location() {
            Some(location) => format!(" in file '{}' at line {}", location.file(), location.line()),
            None => String::new(),
        };
        write(Kind::Error, &format!("Panic occured{}\n{}", location, payload));
    }));
}
