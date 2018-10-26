let nocache = path => path + "?_=" + Math.floor(Math.random()*0x80000000)
let LAST_FRAME_TIME = +new Date();

let handle = (event, args) => {
    write_args(BUFFER, event.args, args);
    WASM.exports.handle(event.code);
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
            handle(EVENT.TIMEOUT, [parseFloat(sec)]);
        }, 1000*sec);
    },
    js_mod_load: (path_ptr, path_len) => {
        let path = load_str_mem(path_ptr, path_len);
        let script = document.createElement("script");
        script._mod_name = path;
        script.addEventListener("load", () => {
            let s = 0;
            try {
                MODULES[path].init();
            } catch (e) {
                s = 2;
            }
            handle(EVENT.LOADED, [path, s]);
        });
        script.addEventListener("error", () => {
            handle(EVENT.LOADED, [path, 1]);
        });
        script.src = nocache(path);
        document.head.appendChild(script);
    },
    js_mod_call: (mod_ptr, mod_len, func_ptr, func_len) => {
        let mod = MODULES[load_str_mem(mod_ptr, mod_len)];
        if (mod) {
            let func = mod.exports[load_str_mem(func_ptr, func_len)];
            if (func) {
                try {
                    call_func(func, BUFFER);
                    return 0;
                } catch (e) {
                    return 3;
                }
            } else {
                return 2;
            }
        } else {
            return 1;
        }
    },
    js_mod_check: (mod_ptr, mod_len) => {
        let mod = load_str_mem(mod_ptr, mod_len);
        if (MODULES[mod]) {
            return 0;
        }
        return 1;
    },
    js_request_frame: () => {
        window.requestAnimationFrame(() => {
            now = +new Date();
            let ms = now - LAST_FRAME_TIME;
            LAST_FRAME_TIME = now;
            handle(EVENT.RENDER, [parseFloat(0.001*ms)]);
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
        let buffer_ptr = WASM.exports.init();
        console.log("buffer init");
        BUFFER = new DataView(WASM.exports.memory.buffer, buffer_ptr, BUFFER_SIZE);
        handle(EVENT.START, []);
    });

});
