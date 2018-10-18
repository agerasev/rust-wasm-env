let BUFFER_SIZE = 0x1000;

let TYPE = {
	"i8": {
		"size": 1, 
		"write": (view, pos, value) => view.setInt8(pos, value, true),
		"read": (view, pos) => view.getInt8(pos, true)
	},
	"u8": {
		"size": 1, 
		"write": (view, pos, value) => view.setUint8(pos, value, true),
		"read": (view, pos) => view.getUint8(pos, true)
	},
	"i16": {
		"size": 2, 
		"write": (view, pos, value) => view.setInt16(pos, value, true),
		"read": (view, pos) => view.getInt16(pos, true)
	},
	"u16": {
		"size": 2, 
		"write": (view, pos, value) => view.setUint16(pos, value, true),
		"read": (view, pos) => view.getUint16(pos, true)
	},
	"i32": {
		"size": 4, 
		"write": (view, pos, value) => view.setInt32(pos, value, true),
		"read": (view, pos) => view.getInt32(pos, true)
	},
	"u32": {
		"size": 4, 
		"write": (view, pos, value) => view.setUint32(pos, value, true),
		"read": (view, pos) => view.getUint32(pos, true)
	},
	"f32": {
		"size": 4, 
		"write": (view, pos, value) => view.setFloat32(pos, value, true),
		"read": (view, pos) => view.getFloat32(pos, true)
	},
	"f64": {
		"size": 8, 
		"write": (view, pos, value) => view.setFloat64(pos, value, true),
		"read": (view, pos) => view.getFloat64(pos, true)
	},
	"isize": {
		"size": 4, 
		"write": (view, pos, value) => view.setInt32(pos, value, true),
		"read": (view, pos) => view.getInt32(pos, true)
	},
	"usize": {
		"size": 4, 
		"write": (view, pos, value) => view.setUint32(pos, value, true),
		"read": (view, pos) => view.getUint32(pos, true)
	}
};


let EVENT = {
	"TIMEOUT": {
		"code": 0x01,
		"args": ["f64"],
	},
	"LOADED": {
		"code": 0x02,
		"args": ["u32"],
	},
	"STEP": {
		"code": 0x41,
		"args": ["f64"],
	},
	"RENDER": {
		"code": 0x42,
		"args": [],
	},
};

let load_str = (ptr, len) => {
    const view = new Uint8Array(WASM.exports.memory.buffer, ptr, len);
    //const utf8dec = new TextDecoder("utf-8");
    //return utf8dec.decode(view);
    let str = "";
    for (let i = 0; i < view.length; i++) {
        str += String.fromCharCode(view[i]);
    }
    return str;
}

let read_args = (view, types) => {
	let pos = 0;
    let args = [];
    for (let i = 0; i < types.length; ++i) {
        let type = TYPE[types[i]];
        args.push(type.read(view, pos));
        pos += type.size;
    }
    return args;
}

let write_args = (view, types, args) => {
	let pos = 0;
    for (let i = 0; i < types.length; ++i) {
        let type = TYPE[types[i]];
        type.write(BUFFER, pos, args[i]);
        pos += type.size;
    }
}

let call_func = (func, view) => {
    return func.func.apply(null, read_args(view, func.args));
}
