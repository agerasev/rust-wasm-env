let MODULES = {};

let WASM = null;
let LAST = null;
let DONE = true;

let BUFFER = null;

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

let handle = (event) => {
    console.log("handle");
    let pos = 0;
    for (let i = 0; i < event.args.length; ++i) {
        let type = TYPE[event.args[i].type];
        type.write(BUFFER, pos, event.args[i].value);
        pos += type.size;
    }
    WASM.exports.handle(event.code);
}

let call_func = (func, view) => {
    let pos = 0;
    let args = [];
    for (let i = 0; i < func.args.length; ++i) {
        let type = TYPE[event.args[i]];
        args.push(type.read(view, pos, event.args[i].value));
        pos += type.size;
    }
    func.func.apply(null, args);
}

let env = {
    js_console: (type, ptr, len) => {
        let str = load_str(ptr, len);
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
            handle(new Event.Timeout(parseFloat(sec)));
        }, 1000*sec);
    },
    js_mod_load: (id, path_ptr, path_len) => {
        let path = load_str(path_ptr, path_len);
        let script = document.createElement("script");
        script.addEventListener("load", () => {
            handle(new Event.Loaded(id));
        });
        script.src = path;
        document.head.appendChild(script);
    },
    js_mod_call: (mod_ptr, mod_len, func_ptr, func_len) => {
        let mod = load_str(mod_ptr, mod_len);
        let func = load_str(func_ptr, func_len);
        call_func(MODULES[mod].exports[func], BUFFER);
    },
};

let render = () => {
    if (!DONE) {
        let now = +new Date();
        let ms = now - LAST;
        LAST = now;
        handle(new Event.Step(parseFloat(0.001*ms)));
        handle(new Event.Render());
        window.requestAnimationFrame(render);
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
    fetch(path + "?_=" + Math.floor(Math.random()*0x80000000))
    .then(response => response.arrayBuffer())
    .then(bytes => WebAssembly.instantiate(bytes, {env: env}))
    .then(results => {
        onload(results.instance);
    });
};

window.addEventListener("load", () => {
    canvas_init();

    import_env(env, math_env, "");
    import_env(env, canvas_env, "js_canvas_");

    let pause_button = document.getElementById("pause");
    let start = () => {
        DONE = false;
        LAST = +new Date();
        window.requestAnimationFrame(render);
        console.log("[info] start animation");
        pause_button.innerText = "Pause";
    };
    let stop = () => {
        DONE = true;
        console.log("[info] stop animation");
        pause_button.innerText = "Resume";
    };
    pause_button.addEventListener("click", () => {
        if (DONE) {
            start();
        } else {
            stop();
        }
    });

    console.log("load WASM");

    load_wasm("./main.wasm", env, instance => {
        WASM = instance;
        console.log("WASM init");
        let buffer_ptr = WASM.exports.init();
        BUFFER = new DataView(WASM.exports.memory.buffer, buffer_ptr, BUFFER_SIZE);
        start();
    });

});
