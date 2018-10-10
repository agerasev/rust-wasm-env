let wasm = null;
let last = null;
let done = true;

let event_data = null;

let handle = (event) => {
    let pos = 0;
    for (let i = 0; i < event.args.length; ++i) {
        let type = TYPE[event.args[i].type];
        type.write(event_data, pos, event.args[i].value);
        pos += type.size;
    }
    wasm.exports.handle(event.code);
}

let load_str = (ptr, len) => {
    const view = new Uint8Array(wasm.exports.memory.buffer, ptr, len);
    //const utf8dec = new TextDecoder("utf-8");
    //return utf8dec.decode(view);
    let str = "";
    for (let i = 0; i < view.length; i++) {
        str += String.fromCharCode(view[i]);
    }
    return str;
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
    js_timeout: (sec) => {
        setTimeout(() => {
            handle(EVENT["Timeout"](parseFloat(sec)));
        }, 1000*sec);
    },
    js_crypto_random: (ptr, len) => {
        let view = new Uint8Array(wasm.exports.memory.buffer, ptr, len);
        return window.crypto.getRandomValues(view);
    },
};

let render = () => {
    if (!done) {
        let now = +new Date();
        let ms = now - last;
        last = now;
        handle(EVENT["Step"](parseFloat(0.001*ms)));
        handle(EVENT["Render"]());
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

let resize = () => {
    let width = 
        window.innerWidth || 
        document.documentElement.clientWidth || 
        document.body.clientWidth;
    let height = 
        window.innerHeight ||
        document.documentElement.clientHeight ||
        document.body.clientHeight;

    canvas_resize(width, height);
    if (wasm && done) {
        handle(EVENT["Render"]());
    }

    console.log("[info] resize: " + width + " x " + height);
};

window.addEventListener("load", () => {
    canvas_init();
    resize();
    window.addEventListener("resize", resize);

    import_env(env, math_env, "");
    import_env(env, canvas_env, "js_canvas_");

    let pause_button = document.getElementById("pause");
    let start = () => {
        done = false;
        last = +new Date();
        window.requestAnimationFrame(render);
        console.log("[info] start animation");
        pause_button.innerText = "Pause";
    };
    let stop = () => {
        done = true;
        console.log("[info] stop animation");
        pause_button.innerText = "Resume";
    };
    pause_button.addEventListener("click", () => {
        if (done) {
            start();
        } else {
            stop();
        }
    });

    console.log("load wasm");

    load_wasm("../../main.wasm", env, instance => {
        wasm = instance;
        console.log("wasm init");
        let event_data_ptr = wasm.exports.init();
        event_data = new DataView(wasm.exports.memory.buffer, event_data_ptr, EVENT_DATA_SIZE);
        start();
    });

});
