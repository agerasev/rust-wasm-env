class Event {
    constructor(code, types) {
        this.code = code;
        this.types = ["u32"].concat(types);
    }
    pack(values) {
        if (this.types.length != values.length + 1) {
            throw "Types and values size mismatch";
        }
        return [this.types, [this.code].concat(values)];
    }
}

let EVENT = {
    START: new Event(0x00, []),
    TIMEOUT: new Event(0x01, ["f64"]),
    LOADED: new Event(0x02, ["str", "i32", "u32", "u32"]),
    MODULE: new Event(0x03, ["str", "i32", "u32"]),
    RENDER: new Event(0x40, ["f64"]),
    USER: new (class extends Event {
        constructor(code) {
            super(code, []);
        }
        pack(types, values) {
            if (types.length != values.length) {
                throw "Types and values size mismatch";
            }
            return [
                ["u32"].concat(types),
                [this.code].concat(values)
            ];
        }
    })(0xFF),
};
