let EVENT_DATA_SIZE = 0x1000;

let TYPE = {
	"f64": {
		"size": 8, 
		"write": (view, pos, value) => view.setFloat64(pos, value, true),
		"read": (view, pos) => view.getFloat64(pos, true)
	},
}

let EVENT = {
	"Timeout": (dt) => { return { "code": 0x01, "args": [
		{"type": "f64", "value": dt}
	] } },
	"Step":    (dt) => { return { "code": 0x41, "args": [
		{"type": "f64", "value": dt}
	] } },
	"Render":  (dt) => { return { "code": 0x42, "args": [] } },
}


