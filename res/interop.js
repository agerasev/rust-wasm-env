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


let Event = {};
Event.Timeout = class {
	constructor (dt) {
		this.code = 0x01;
		this.args = [
			{"type": "f64", "value": dt}
		];
	}
};

Event.Step = class {
	constructor (dt) {
		this.code = 0x41;
		this.args = [
			{"type": "f64", "value": dt}
		];
	}
};
Event.Render = class {
	constructor () {
		this.code = 0x42;
		this.args = [];
	}
};
