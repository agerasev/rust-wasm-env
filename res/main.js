let nocache = path => path + "?_=" + Math.floor(Math.random()*0x80000000)
let LAST_FRAME_TIME = +new Date();

let handle = (tuple) => {
        let types = tuple[0];
        let values = tuple[1];
        let size = count_args_size(types, values);
        let ptr = WASM.exports.alloc(size);
        let buffer = new DataView(WASM.exports.memory.buffer, ptr, size);
        store_args(buffer, types, values);
        WASM.exports.handle(ptr);
        WASM.exports.free(ptr);
}

let env = {
    js_console: (type, ptr, len) => {
        let str = load_str_mem(ptr, len);
        if (type == 1) {
            console.error(str);
        } else {
            console.log(str);
        }
    },
    js_crypto_random: (ptr, len) => {
        let view = new Uint8Array(WASM.exports.memory.buffer, ptr, len);
        return window.crypto.getRandomValues(view);
    },
    js_timeout: (sec) => {
        setTimeout(() => {
            handle(EVENT.TIMEOUT.pack([parseFloat(sec)]));
        }, 1000*sec);
    },
    js_mod_load: (path_ptr, path_len) => {
        let path = load_str_mem(path_ptr, path_len);
        let script = document.createElement("script");
        let id = MOD_COUNTER++;
        script._mod_id = id;
        script.addEventListener("load", () => {
            let s = 0;
            try {
                MODULES[id].init();
            } catch (e) {
                console.error(e);
                s = 2;
            }
            handle(EVENT.MODULE.pack([path, s, id]));
        });
        script.addEventListener("error", () => {
            handle(EVENT.MODULE.pack([path, 1, id]));
        });
        script.src = nocache(path);
        document.head.appendChild(script);
    },
    js_mod_call: (mod_id, func_ptr, func_len, buf_ptr, buf_len) => {
        let mod = MODULES[mod_id];
        if (mod) {
            let func = mod.exports[load_str_mem(func_ptr, func_len)];
            let buf = new DataView(WASM.exports.memory.buffer, buf_ptr, buf_len);
            if (func) {
                try {
                    call_func(func, buf);
                    return 0;
                } catch (e) {
                    console.error(e);
                    return 3;
                }
            } else {
                return 2;
            }
        } else {
            return 1;
        }
    },
    js_mod_check: (mod_id) => {
        if (MODULES[mod_id]) {
            return 0;
        }
        return 1;
    },
    js_request_frame: () => {
        window.requestAnimationFrame(() => {
            now = +new Date();
            let ms = now - LAST_FRAME_TIME;
            LAST_FRAME_TIME = now;
            handle(EVENT.RENDER.pack([parseFloat(0.001*ms)]));
        });
    },
    js_drop_object: (id) => {
        del_object(id);
    }
};

let import_env = (env, im_env, prefix) => {
    prefix = !prefix ? "" : prefix;
    for (let key in im_env) {
        if (im_env.hasOwnProperty(key)) {
            env[prefix + key] = im_env[key];
        }
    }
    return env;
};

let load_wasm = (path, env, onload) => {
    fetch(nocache(path))
    .then(response => response.arrayBuffer())
    .then(bytes => WebAssembly.instantiate(bytes, {env: env}))
    .then(results => onload(results.instance), results => console.error(results));
};

window.addEventListener("load", () => {
    // bind math routines
    import_env(env, math_env, "");

    // bind preloaded modules
    Object.keys(MODULES).forEach((key) => {
        let mod = MODULES[key];
        mod.init();
        let mod_env = {};
        Object.keys(mod.exports).forEach((fn) => {
            mod_env[fn] = mod.exports[fn].func;
        });
        import_env(env, mod_env, "js_" + key + "_");
    });

    console.log("load wasm");

    load_wasm("./main.wasm", env, instance => {
        WASM = instance;
        console.log("wasm init");
        WASM.exports.init();
        handle(EVENT.START.pack([]));
    });

});
