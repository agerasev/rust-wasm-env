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
        pos += type.size(value);
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
        pos += type.size(value);
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
