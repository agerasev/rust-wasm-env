class Type {
    constructor(size, store, load) {
        this._size = size;
        this._store = store;
        this._load = load;
    }
    size(value) {
        return this._size;
    }
    store(view, pos, value) {
        this._store(view, pos, value);
    }
    load(view, pos) {
        return this._load(view, pos);
    }
}

let TYPE = {
    void: new Type(0, () => {}, () => {}),
    i8: new Type(
        1,
        (view, pos, value) => view.setInt8(pos, value, true),
        (view, pos) => view.getInt8(pos, true),
    ),
    u8: new Type(
        1,
        (view, pos, value) => view.setUint8(pos, value, true),
        (view, pos) => view.getUint8(pos, true),
    ),
    i16: new Type(
        2,
        (view, pos, value) => view.setInt16(pos, value, true),
        (view, pos) => view.getInt16(pos, true),
    ),
    u16: new Type(
        2,
        (view, pos, value) => view.setUint16(pos, value, true),
        (view, pos) => view.getUint16(pos, true),
    ),
    i32: new Type(
        4,
        (view, pos, value) => view.setInt32(pos, value, true),
        (view, pos) => view.getInt32(pos, true),
    ),
    u32: new Type(
        4,
        (view, pos, value) => view.setUint32(pos, value, true),
        (view, pos) => view.getUint32(pos, true),
    ),
    f32: new Type(
        4,
        (view, pos, value) => view.setFloat32(pos, value, true),
        (view, pos) => view.getFloat32(pos, true),
    ),
    f64: new Type(
        8,
        (view, pos, value) => view.setFloat64(pos, value, true),
        (view, pos) => view.getFloat64(pos, true),
    ),
    isize: new Type(
        4,
        (view, pos, value) => view.setInt32(pos, value, true),
        (view, pos) => view.getInt32(pos, true),
    ),
    usize: new Type(
        4,
        (view, pos, value) => view.setUint32(pos, value, true),
        (view, pos) => view.getUint32(pos, true),
    ),
    str: new (class extends Type {
        constructor() {
            super(
                -1,
                (view, pos, value) => {
                    TYPE["usize"].store(view, pos, value.length);
                    let len = store_str(view, pos + 4, value);
                },
                (view, pos) => {
                    let len = TYPE["usize"].load(view, pos);
                    return load_str(view, pos + 4, len);
                }
            );
        }
        size(value) {
            return 4 + 2*value.length;
        }
    })(),
};
