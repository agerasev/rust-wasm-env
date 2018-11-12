let TYPE = {
    "void": {
        "size": 0, 
        "store": (view, pos, value) => {},
        "load": (view, pos) => {}
    },
    "i8": {
        "size": 1, 
        "store": (view, pos, value) => view.setInt8(pos, value, true),
        "load": (view, pos) => view.getInt8(pos, true)
    },
    "u8": {
        "size": 1, 
        "store": (view, pos, value) => view.setUint8(pos, value, true),
        "load": (view, pos) => view.getUint8(pos, true)
    },
    "i16": {
        "size": 2, 
        "store": (view, pos, value) => view.setInt16(pos, value, true),
        "load": (view, pos) => view.getInt16(pos, true)
    },
    "u16": {
        "size": 2, 
        "store": (view, pos, value) => view.setUint16(pos, value, true),
        "load": (view, pos) => view.getUint16(pos, true)
    },
    "i32": {
        "size": 4, 
        "store": (view, pos, value) => view.setInt32(pos, value, true),
        "load": (view, pos) => view.getInt32(pos, true)
    },
    "u32": {
        "size": 4, 
        "store": (view, pos, value) => view.setUint32(pos, value, true),
        "load": (view, pos) => view.getUint32(pos, true)
    },
    "f32": {
        "size": 4, 
        "store": (view, pos, value) => view.setFloat32(pos, value, true),
        "load": (view, pos) => view.getFloat32(pos, true)
    },
    "f64": {
        "size": 8, 
        "store": (view, pos, value) => view.setFloat64(pos, value, true),
        "load": (view, pos) => view.getFloat64(pos, true)
    },
    "isize": {
        "size": 4, 
        "store": (view, pos, value) => view.setInt32(pos, value, true),
        "load": (view, pos) => view.getInt32(pos, true)
    },
    "usize": {
        "size": 4, 
        "store": (view, pos, value) => view.setUint32(pos, value, true),
        "load": (view, pos) => view.getUint32(pos, true)
    },
    "str": {
        "size": -1,
        "get_size": (value) => {
            return 4 + 2*value.length;
        },
        "store": (view, pos, value) => {
            TYPE["usize"].store(view, pos, value.length);
            let len = store_str(view, pos + TYPE["usize"].size, value);
        },
        "load": (view, pos) => {
            let len = TYPE["usize"].load(view, pos);
            return load_str(view, pos + TYPE["usize"].size, len);
        }
    }
};

let EVENT = {
    "START": {
        "code": 0x00,
        "args": [],
    },
    "TIMEOUT": {
        "code": 0x01,
        "args": ["f64"],
    },
    "LOADED": {
        "code": 0x02,
        "args": ["str", "i32", "u32", "u32"],
    },
    "MODULE": {
        "code": 0x03,
        "args": ["str", "i32", "u32"],
    },
    "RENDER": {
        "code": 0x40,
        "args": ["f64"],
    },
    "USER": {
        "code": 0xFF,
    },
};

let load_str = (view, pos, len) => {
    let str = "";
    for (let i = 0; i < len; ++i) {
        str += String.fromCharCode(view.getUint16(pos + 2*i, true));
    }
    return str;
};

let store_str = (view, pos, str) => {
    for (let i = 0; i < str.length; ++i) {
        view.setUint16(pos + 2*i, str.charCodeAt(i), true);
    }
};

let load_str_mem = (ptr, len) => {
    let view = new DataView(WASM.exports.memory.buffer, ptr, 2*len);
    return load_str(view, 0, len);
};

let load_args = (view, types) => {
    let pos = 0;
    let args = [];
    for (let i = 0; i < types.length; ++i) {
        let type = TYPE[types[i]];
        let value = type.load(view, pos);
        if (type.size >= 0) {
            pos += type.size;
        } else {
            pos += type.get_size(value);
        }
        args.push(value);
    }
    return args;
};

let store_args = (view, types, args) => {
    let pos = 0;
    for (let i = 0; i < types.length; ++i) {
        let type = TYPE[types[i]];
        let value = args[i];
        if (view !== null) {
            type.store(view, pos, value);
        }
        if (type.size >= 0) {
            pos += type.size;
        } else {
            pos += type.get_size(value);
        }
    }
    return pos;
};

let count_args_size = (types, args) => {
    return store_args(null, types, args);
};

let call_func = (func, view) => {
    let args = load_args(view, func.args);
    let ret = func.func.apply(null, args);
    store_args(view, [func.ret], [ret]);
};
